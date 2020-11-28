package metadata

components: sinks: aws_cloudwatch_metrics: components._aws & {
	title:       "AWS Cloudwatch Metrics"
	description: sinks._aws_cloudwatch.description

	classes: {
		commonly_used: false
		delivery:      "at_least_once"
		development:   "stable"
		egress_method: "batch"
		service_providers: ["AWS"]
	}

	features: {
		buffer: enabled:      false
		healthcheck: enabled: true
		send: {
			batch: {
				enabled:      true
				common:       false
				max_bytes:    null
				max_events:   20
				timeout_secs: 1
			}
			compression: {
				enabled: true
				default: "none"
				algorithms: ["none", "gzip"]
				levels: ["none", "fast", "default", "best", 0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
			}
			encoding: enabled: false
			request: enabled:  false
			tls: enabled:      false
			to: {
				service: services.aws_cloudwatch_metrics

				interface: {
					socket: {
						api: {
							title: "AWS Cloudwatch metrics API"
							url:   urls.aws_cloudwatch_metrics_api
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
		warnings: [
			"""
				Gauge values are persisted between flushes. When Vector starts up, each gauge is
				assumed to have a value of zero (0.0). That value can be updated explicitly either
				by the subsequent absolute (not delta) gauge observation or by delta
				increments/decrements. Delta gauges are considered an advanced feature that's
				mostly useful in distributed settings and should be used with due care.
				""",
		]
		notices: [
			"""
				CloudWatch Metrics types are organized not by their semantics but rather by their
				storage properties:

				* Statistic sets
				* Data points

				In Vector, only the latter is used to allow lossless statistics calculations on the
				CloudWatch side.
				""",
		]
	}

	configuration: {
		default_namespace: {
			description: """
				A [namespace](\(urls.aws_cloudwatch_namespace)) that will isolate different metrics
				from each other. Used as a namespace for metrics that don't have it.
				"""
			required:    true
			warnings: []
			type: string: {
				examples: ["service"]
			}
		}
	}

	input: {
		logs: false
		metrics: {
			counter:      true
			distribution: true
			gauge:        true
			histogram:    false
			set:          false
			summary:      false
		}
	}
}
