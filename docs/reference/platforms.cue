package metadata

platforms: [!=""]: {
	#ComponentRef: {
		tag:         string
		kind:        "source" | "sink"
		used_for: != string
	}

	components: [#ComponentRef, ...#ComponentRef]
}
