---
title: Vector for Amazon Web Services
description: Use Vector to power observability on AWS
---

[Amazon Web Services][aws] (AWS) offers a variety of cloud services that you can use to power your
observability infrastructure, from object storage ([S3][aws.s3]) to streaming data
([Kinesis][aws.kinesis]) to logs ([Cloudwatch Logs][aws.cloudwatch_logs]) and metrics ([Cloudwatch
Metrics][aws.cloudwatch_metrics]).

Vector can act as a major force multiplier inside of your AWS environment, enabling you to use AWS
services at lower cost and more effectively.

## Vector components for AWS

### Sources

Vector has dedicated [sources][docs.sources] for these AWS services:

<ComponentList kind="source" platform="aws" />

### Sinks

Vector also has dedicated [sinks][docs.sinks] for these services:

<ComponentList kind="sink" platform="aws" />

## Migrating to and from AWS

Because Vector has [sources][docs.sources] and [sinks][docs.sinks] that span a wide variety of
platforms, it's an ideal tool for multi-platform use cases, including migrating between platforms, experimenting
with new platforms,

Here are some examples of wahts that Vector can help you migrate *to* AWS:

* TODO

And here are some examples of ways that Vector can help you migrate *from* AWS:

* Being piping your logs that are currently going to Cloudwatch Logs into a [Kinesis][aws.kinesis]
  stream as well. Have Vector listen on that stream using the
  [`aws_kinesis_firehose`][docs.sources.aws_kinesis_firehose] and route them to
  [Datadog][docs.sinks.datadog].

## Related guides

* [Ingesting AWS CloudWatch Logs via AWS Kinesis Firehose][guides.cloudwatch_kinesis]
* [Send logs to Amazon Simple Queue Service][guides.sqs_logs]
* [Send logs from AWS S3 to anywhere][guides.s3_logs]
* [Send logs from AWS Kinesis Firehose to anywhere][guides.kinesis_firehose_source]
* [Send metrics from Amazon ECS to anywhere][guides.ecs_metrics]
* [Send logs to AWS S3][guides.s3_logs_send]
* [Send logs to AWS Kinesis Firehose][guides.kinesis_firehose_sink]
* [Send logs to AWS Kinesis Data Streams][guides.kinesis_streams]
* [Send metrics to AWS Cloudwatch metrics][guides.cloudwatch_metrics]
* [Send logs to AWS Cloudwatch logs][guides.cloudwatch_logs]

[aws]: https://aws.amazon.com
[aws.cloudwatch_logs]: https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/WhatIsCloudWatchLogs.html
[aws.cloudwatch_metrics]: https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/working_with_metrics.html
[aws.kinesis]: https://aws.amazon.com/kinesis
[aws.s3]: https://aws.amazon.com/s3
[docs.sinks]: /docs/reference/sinks
[docs.sinks.datadog_logs]: /docs/reference/sinks/datadog_logs
[docs.sources.aws_kinesis_firehose]: /docs/reference/sources/aws_kinesis_firehose
[docs.sources]: /docs/reference/sources
[guides.cloudwatch_kinesis]: /guides/advanced/cloudwatch-logs-firehose
[guides.cloudwatch_logs]: /guides/integrate/sinks/aws_cloudwatch_logs
[guides.cloudwatch_metrics]: /guides/integrate/sinks/aws_cloudwatch_metrics
[guides.ecs_metrics]: /guides/integrate/sources/aws_ecs_metrics
[guides.sqs_logs]: /guides/integrate/sinks/aws_sqs
[guides.s3_logs]: /guides/integrate/sources/aws_s3
[guides.s3_logs_send]: /guides/integrate/sinks/aws_s3
[guides.kinesis_firehose_sink]: /guides/integrate/sinks/aws_kinesis_firehose
[guides.kinesis_firehose_source]: /guides/integrate/sources/aws_kinesis_firehose
[guides.kinesis_streams]: /guides/integrate/sinks/aws_kinesis_streams
