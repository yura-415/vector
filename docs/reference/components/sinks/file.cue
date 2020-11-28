package metadata

components: sinks: file: {
	title: "File"

	classes: {
		commonly_used: false
		delivery:      "at_least_once"

		development:   "beta"
		egress_method: "stream"
		service_providers: []
	}

	features: {
		buffer: enabled:      false
		healthcheck: enabled: true
		send: {
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
					enum: ["ndjson", "text"]
				}
			}
			request: enabled: false
			tls: enabled:     false
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
		idle_timeout_secs: {
			common: false
			description: """
				The amount of time a file can be idle and stay open. If no events are received
				within this duration, the file is flushed and closed.
				"""
			required: false
			warnings: []
			type: uint: {
				default: 30
				unit:    null
			}
		}
		path: {
			description: "File name to write events to."
			required:    true
			warnings: []
			type: string: {
				examples: [
					"/tmp/vector-%Y-%m-%d.log",
					"/tmp/application-{{ application_id }}-%Y-%m-%d.log",
				]
				templateable: true
			}
		}
	}

	input: {
		logs:    true
		metrics: null
	}

	how_it_works: {
		dir_and_file_creation: {
			title: "File and Directory Creation"
			body: """
				Vector attempts to create both the file and the entire directory structure when
				emitting events to the `file` sink. The Vector agent thus needs to have the correct
				permissions to create and write to files in the specified directories.
				"""
		}
	}
}
