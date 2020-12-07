---
title: Vector for Amazon Web Services
description: Use Vector to power observability on AWS
---

[Amazon Web Services][aws] (AWS) offers a variety of cloud services that you can use to power your
observability infrastructure, from object storage ([S3][aws.s3]) to streaming data ([Kinesis][aws.kinesis])

Vector can act as a major force multiplier if you're

## Sources

Vector has dedicated [sources][docs.sources] for these AWS services:

<ComponentList kind="source" platform="aws" />

## Sinks

Vector also has dedicated [sinks][docs.sinks] for these services:

<ComponentList kind="sink" platform="aws" />

[aws]: https://aws.amazon.com
[aws.kinesis]: https://aws.amazon.com/kinesis
[aws.s3]: https://aws.amazon.com/s3
[docs.sinks]: /docs/reference/sinks
[docs.sources]: /docs/reference/sources
