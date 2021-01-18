---
title: VRL Quick Start
description: "Get up and running with Vector Remap Language"
---

## Syslog

Let's start by parsing and transforming some [Syslog] logs. Create a `vector.toml` file in the current directory and copy/paste this into it:

```toml
[sources.logs]
type = "generator"
format = "syslog"
interval = 0.5

[transforms.shape]
type = "remap"
inputs = ["logs"]
source = '''
.
'''

[sinks.out]
type = "console"
inputs = ["shape"]
encoding.codec = "json"
```

Here, we're using the [`generator`][generator]

Each event should include a `message` field and an automatically added `timestamp` field:

```json
{"message":"<93>2 2021-01-18T20:01:07.207Z for.us shaneIxD 4767 ID327 - Maybe we just shouldn't use computers","timestamp":"2021-01-18T20:01:07.207897800Z"}
```

What we want, though, is to *parse* the Syslog messages into named fields (`appname`, `facility`, `hostname`, etc.). To do that, let's update the script to use the [`parse_syslog`][parse_syslog] function:

```toml
[transforms.shape]
type = "remap"
inputs = ["logs"]
source = '''
. = parse_syslog(.message)
'''
```

Restart Vector, and you should see JSON output like this:

```json
{
  "appname": "shaneIxD",
  "facility": "ftp",
  "hostname": "for.us",
  "message": "Maybe we just shouldn't use computers",
  "msgid": "ID327",
  "procid": 4767,
  "severity": "notice",
  "timestamp": "2021-01-18T20:01:07.207897800Z"
}
```

## JSON

```toml
[sources.logs]
type = "generator"
format = "json"
interval = 0.5

[transforms.shape]
type = "remap"
inputs = ["logs"]
source = '''
.
'''

[sinks.out]
type = "console"
inputs = ["shape"]
encoding.codec = "json"
```

```json
{"message":"{\"host\":\"168.65.234.217\",\"user-identifier\":\"shaneIxD\",\"datetime\":\"18/Jan/2021:19:55:07\",\"method\":\"GET\",\"request\":\"/do-not-access/needs-work\",\"protocol\":\"HTTP/1.0\",\"status\":\"300\",\"bytes\":22523,\"referer\":\"https://some.de/do-not-access/needs-work\"}","timestamp":"2021-01-18T19:55:07.448552Z"}
```

```toml
[transforms.shape]
type = "remap"
inputs = ["logs"]
source = '''
. = parse_json!(.message)
'''
```

[generator]: /docs/reference/remap/functions/generator
[parse_syslog]: /docs/reference/remap/functions/parse_syslog
[syslog]: https://tools.ietf.org/html/rfc5424
