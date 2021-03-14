#[cfg(feature = "listenfd")]
use super::{
    CreateAcceptor, IncomingListener, MaybeTlsSettings, MaybeTlsStream, SslBuildError, TcpBind,
    TlsError, TlsSettings,
};
#[cfg(all(unix, feature = "sources-utils-tcp-socket"))]
use crate::tcp;
#[cfg(feature = "sources-utils-tcp-keepalive")]
use crate::tcp::TcpKeepaliveConfig;
use futures::{future::BoxFuture, stream, FutureExt, Stream};
use openssl::ssl::{Ssl, SslAcceptor, SslMethod};
use snafu::ResultExt;
use std::{
    future::Future,
    net::SocketAddr,
    pin::Pin,
    task::{Context, Poll},
};
use tokio::{
    io::{self, AsyncRead, AsyncWrite, ReadBuf},
    net::{TcpListener, TcpStream},
};
use tokio_openssl::SslStream;

impl TlsSettings {
    pub(crate) fn acceptor(&self) -> crate::tls::Result<SslAcceptor> {
        match self.identity {
            None => Err(TlsError::MissingRequiredIdentity),
            Some(_) => {
                let mut acceptor =
                    SslAcceptor::mozilla_intermediate(SslMethod::tls()).context(CreateAcceptor)?;
                self.apply_context(&mut acceptor)?;
                Ok(acceptor.build())
            }
        }
    }
}

impl MaybeTlsSettings {
    pub(crate) async fn bind(&self, addr: &SocketAddr) -> crate::tls::Result<MaybeTlsListener> {
        let listener = TcpListener::bind(addr).await.context(TcpBind)?;

        let acceptor = match self {
            Self::Tls(tls) => Some(tls.acceptor()?),
            Self::Raw(()) => None,
        };

        Ok(MaybeTlsListener { listener, acceptor })
    }
}

pub(crate) struct MaybeTlsListener {
    listener: TcpListener,
    acceptor: Option<SslAcceptor>,
}

impl MaybeTlsListener {
    pub(crate) async fn accept(&mut self) -> crate::tls::Result<MaybeTlsIncomingStream<TcpStream>> {
        self.listener
            .accept()
            .await
            .map(|(stream, peer_addr)| {
                MaybeTlsIncomingStream::new(stream, peer_addr, self.acceptor.clone())
            })
            .context(IncomingListener)
    }

    async fn into_accept(
        mut self,
    ) -> (crate::tls::Result<MaybeTlsIncomingStream<TcpStream>>, Self) {
        (self.accept().await, self)
    }

    pub(crate) fn accept_stream(
        self,
    ) -> impl Stream<Item = crate::tls::Result<MaybeTlsIncomingStream<TcpStream>>> {
        let mut accept = Box::pin(self.into_accept());
        stream::poll_fn(move |context| match accept.as_mut().poll(context) {
            Poll::Ready((item, this)) => {
                accept.set(this.into_accept());
                Poll::Ready(Some(item))
            }
            Poll::Pending => Poll::Pending,
        })
    }

    #[cfg(feature = "listenfd")]
    pub(crate) fn local_addr(&self) -> Result<SocketAddr, std::io::Error> {
        self.listener.local_addr()
    }
}

impl From<TcpListener> for MaybeTlsListener {
    fn from(listener: TcpListener) -> Self {
        Self {
            listener,
            acceptor: None,
        }
    }
}

pub struct MaybeTlsIncomingStream<S> {
    state: StreamState<S>,
    // BoxFuture doesn't allow access to the inner stream, but users
    // of MaybeTlsIncomingStream want access to the peer address while
    // still handshaking, so we have to cache it here.
    peer_addr: SocketAddr,
}

enum StreamState<S> {
    Accepted(MaybeTlsStream<S>),
    Accepting(BoxFuture<'static, Result<SslStream<S>, TlsError>>),
    AcceptError(String),
}

impl<S> MaybeTlsIncomingStream<S> {
    #[cfg_attr(not(feature = "listenfd"), allow(dead_code))]
    pub fn peer_addr(&self) -> SocketAddr {
        self.peer_addr
    }

    /// None if connection still hasn't been established.
    #[cfg(any(
        feature = "listenfd",
        feature = "sources-utils-tcp-keepalive",
        feature = "sources-utils-tcp-socket"
    ))]
    pub fn get_ref(&self) -> Option<&S> {
        use super::MaybeTls;

        match &self.state {
            StreamState::Accepted(stream) => Some(match stream {
                MaybeTls::Raw(s) => s,
                MaybeTls::Tls(s) => s.get_ref(),
            }),
            StreamState::Accepting(_) => None,
            StreamState::AcceptError(_) => None,
        }
    }
}

impl<T> MaybeTlsIncomingStream<T>
where
    T: tokio::io::AsyncWriteExt + Unpin,
{
    pub async fn shutdown(&mut self) -> io::Result<()> {
        use super::MaybeTls;

        match &mut self.state {
            StreamState::Accepted(ref mut stream) => match stream {
                MaybeTls::Raw(ref mut s) => s.shutdown().await,
                MaybeTls::Tls(s) => s.get_mut().shutdown().await,
            },
            StreamState::Accepting(_) | StreamState::AcceptError(_) => {
                Err(io::ErrorKind::NotConnected.into())
            }
        }
    }
}

impl MaybeTlsIncomingStream<TcpStream> {
    pub(super) fn new(
        stream: TcpStream,
        peer_addr: SocketAddr,
        acceptor: Option<SslAcceptor>,
    ) -> Self {
        let state = match acceptor {
            Some(acceptor) => StreamState::Accepting(
                async move {
                    let ssl = Ssl::new(acceptor.context()).context(SslBuildError)?;
                    let mut stream = SslStream::new(ssl, stream).context(SslBuildError)?;
                    Pin::new(&mut stream).accept().await.unwrap();
                    Ok(stream)
                }
                .boxed(),
            ),
            None => StreamState::Accepted(MaybeTlsStream::Raw(stream)),
        };
        Self { peer_addr, state }
    }

    // Explicit handshake method
    #[cfg(feature = "listenfd")]
    pub(crate) async fn handshake(&mut self) -> crate::tls::Result<()> {
        if let StreamState::Accepting(fut) = &mut self.state {
            let stream = fut.await?;
            self.state = StreamState::Accepted(MaybeTlsStream::Tls(stream));
        }

        Ok(())
    }

    // TODO: Fix.
    #[cfg(feature = "sources-utils-tcp-keepalive")]
    pub(crate) fn set_keepalive(&mut self, _keepalive: TcpKeepaliveConfig) -> io::Result<()> {
        // let stream = self.get_ref().ok_or_else(|| {
        //     io::Error::new(
        //         io::ErrorKind::NotConnected,
        //         "Can't set keepalive on connection that has not been accepted yet.",
        //     )
        // })?;

        // stream.set_keepalive(keepalive.time_secs.map(std::time::Duration::from_secs))?;

        Ok(())
    }

    #[cfg(all(unix, feature = "sources-utils-tcp-socket"))]
    pub(crate) fn set_receive_buffer_bytes(&mut self, bytes: usize) -> std::io::Result<()> {
        let stream = self.get_ref().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::NotConnected,
                "Can't set receive buffer size on connection that has not been accepted yet.",
            )
        })?;

        tcp::set_receive_buffer_size(stream, bytes);

        Ok(())
    }

    fn poll_io<T, F>(self: Pin<&mut Self>, cx: &mut Context, poll_fn: F) -> Poll<io::Result<T>>
    where
        F: FnOnce(Pin<&mut MaybeTlsStream<TcpStream>>, &mut Context) -> Poll<io::Result<T>>,
    {
        let mut this = self.get_mut();
        loop {
            return match &mut this.state {
                StreamState::Accepted(stream) => poll_fn(Pin::new(stream), cx),
                StreamState::Accepting(fut) => match futures::ready!(fut.as_mut().poll(cx)) {
                    Ok(stream) => {
                        this.state = StreamState::Accepted(MaybeTlsStream::Tls(stream));
                        continue;
                    }
                    Err(error) => {
                        let error = io::Error::new(io::ErrorKind::Other, error);
                        this.state = StreamState::AcceptError(error.to_string());
                        Poll::Ready(Err(error))
                    }
                },
                StreamState::AcceptError(error) => {
                    Poll::Ready(Err(io::Error::new(io::ErrorKind::Other, error.to_owned())))
                }
            };
        }
    }
}

impl AsyncRead for MaybeTlsIncomingStream<TcpStream> {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        self.poll_io(cx, |s, cx| s.poll_read(cx, buf))
    }
}

impl AsyncWrite for MaybeTlsIncomingStream<TcpStream> {
    fn poll_write(self: Pin<&mut Self>, cx: &mut Context, buf: &[u8]) -> Poll<io::Result<usize>> {
        self.poll_io(cx, |s, cx| s.poll_write(cx, buf))
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context) -> Poll<io::Result<()>> {
        self.poll_io(cx, |s, cx| s.poll_flush(cx))
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context) -> Poll<io::Result<()>> {
        self.poll_io(cx, |s, cx| s.poll_shutdown(cx))
    }
}
