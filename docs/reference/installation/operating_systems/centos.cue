package metadata

installation: operating_systems: {
	centos: {
		title:       "CentOS"
		description: """
			[CentOS](\(urls.centos)) is a Linux distribution that's functionally compatible with its
			upstream source, [Red Hat Enterprise Linux (RHEL)](\(urls.rhel)).
			"""

		interfaces: [
			installation._interfaces.yum,
			installation._interfaces.rpm,
			installation._interfaces."vector-installer" & {
				roles: agent: roles._journald_agent
			},
			installation._interfaces."docker-cli",
			installation._interfaces."helm3",
			installation._interfaces.kubectl,
		]

		family:                    "Linux"
		minimum_supported_version: "6"
		shell:                     "bash"
	}
}
