package metadata

components: sinks: aws_kinesis_streams: components._aws & {
	title:       "AWS Kinesis Data Streams"
	description: """
		[Amazon Kinesis Data Streams](\(urls.aws_kinesis_streams)) is a scalable and durable
		real-time data streaming service that can continuously capture gigabytes of data per second
		from hundreds of thousands of sources, making it an excellent candidate for streaming logs
		and metrics data.
		"""

	classes: {
		commonly_used: false
		delivery:      "at_least_once"
		development:   "stable"
		egress_method: "batch"
		service_providers: ["AWS"]
	}

	features: {
		buffer: enabled:      true
		healthcheck: enabled: true
		send: {
			batch: {
				enabled:      true
				common:       false
				max_bytes:    5000000
				max_events:   500
				timeout_secs: 1
			}
			compression: {
				enabled: true
				default: "none"
				algorithms: ["none", "gzip"]
				levels: ["none", "fast", "default", "best", 0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
			}
			encoding: {
				enabled: true
				codec: {
					enabled: true
					default: null
					enum: ["json", "text"]
				}
			}
			request: {
				enabled:                    true
				concurrency:                5
				rate_limit_duration_secs:   1
				rate_limit_num:             5
				retry_initial_backoff_secs: 1
				retry_max_duration_secs:    10
				timeout_secs:               30
			}
			tls: enabled: false
			to: {
				service: services.aws_kinesis_data_streams

				interface: {
					socket: {
						api: {
							title: "AWS Kinesis Data Streams API"
							url:   urls.aws_kinesis_streams_api
						}
						direction: "outgoing"
						protocols: ["http"]
						ssl: "required"
					}
				}
			}
		}
	}

	support: {
		targets: {
			"aarch64-unknown-linux-gnu":  true
			"aarch64-unknown-linux-musl": true
			"x86_64-apple-darwin":        true
			"x86_64-pc-windows-msv":      true
			"x86_64-unknown-linux-gnu":   true
			"x86_64-unknown-linux-musl":  true
		}

		requirements: []
		warnings: []
		notices: []
	}

	configuration: {
		partition_key_field: {
			common:      true
			description: "The log field used as the Kinesis record's partition key value."
			required:    false
			warnings: []
			type: string: {
				default: null
				examples: ["user_id"]
			}
		}
		stream_name: {
			description: """
				The [stream name](\(urls.aws_cloudwatch_logs_stream_name)) of the target Kinesis
				Logs stream.
				"""
			required:    true
			warnings: []
			type: string: {
				examples: ["my-stream"]
			}
		}
	}

	input: {
		logs:    true
		metrics: null
	}

	how_it_works: {
		partitioning: {
			title: "Partitioning"
			body:  """
				By default, Vector issues random 16-byte values for each [Kinesis record's partition
				key](\(urls.aws_kinesis_partition_key)), evenly distributing records across your
				Kinesis partitions. Depending on your use case, this may not be sufficient since
				random distribution does not preserve order. To override this, supply the
				[`partition_key_field`](#partition_key_field) option. This option enables you to
				specify a field on your event to use as the partition key value rather than allowing
				the partition to be chosen randomly. This is useful if your events have a field
				that's a good candidate for a [partition
				key](\(urls.aws_kinesis_streams_partition_key)). If your events don't have such a
				field, you can add one one using the [`add_fields`][docs.transforms.add_fields]
				transform.
				"""
			sub_sections: [
				{
					title: "Missing partition keys"
					body: """
						Kinesis requires a value for the partition key. If the key is missing or the
						value is blank, the event is dropped and a [`warning`-level log
						event][docs.monitoring#logs] is logged. The field specified using the
						[`partition_key_field`](#partition_key_field) option should thus always
						contain a value.
						"""
				},
				{
					title: "Partition keys that exceed 256 characters"
					body: """
						If the value provided exceeds the maximum allowed length of 256 characters,
						Vector slices the value and uses the first 256 characters.
						"""
				},
				{
					title: "Non-string partition keys"
					body:  "Vector coerces the value into a string."
				},
			]
		}
	}
}
