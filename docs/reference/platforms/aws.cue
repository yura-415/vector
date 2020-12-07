package metadata

platforms: aws: {
	components: [
		{
			tag:  "aws_cloudwatch_logs"
			kind: "sink"
		},
		{
			tag:  "aws_cloudwatch_metrics"
			kind: "sink"
		},
		{
			tag:  "awsecs"
			kind: "source"
		},
		{
			tag:  "aws_kinesis_firehose"
			kind: "sink"
		},
		{
			tag:  "aws_kinesis_firehose"
			kind: "source"
		},
		{
			tag:  "aws_kinesis_streams"
			kind: "sink"
		},
		{
			tag:  "aws_s3"
			kind: "sink"
		},
		{
			tag:  "aws_s3"
			kind: "source"
		},
		{
			tag:  "aws_sqs"
			kind: "sink"
		},
	]
}
