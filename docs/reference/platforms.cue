package metadata

platforms: [!=""]: {
	#ComponentRef: {
		tag:         !=""
		kind:        "source" | "sink"

	}

	components: [#ComponentRef, ...#ComponentRef]
}
