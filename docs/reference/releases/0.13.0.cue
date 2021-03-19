package metadata

releases: "0.13.0": {
	date:     "2021-03-19"
	codename: ""

	whats_next: []

	commits: [
		{sha: "81e045e3d3b33d40399e5757ebb30c55c3da901f", date: "2021-03-14 17:43:25 UTC", description: "Add new VRL comparison benchmark", pr_number:                                6387, scopes: ["performance"], type:              "enhancement", breaking_change: false, author: "Luc Perkins", files_count:        7, insertions_count:  582, deletions_count: 2},
		{sha: "ef32bab16a9256782a853bb82edc8b0d54a8ea71", date: "2021-03-15 12:49:32 UTC", description: "Use next_addr instead of fixed addresses", pr_number:                        6766, scopes: ["prometheus_exporter sink"], type: "fix", breaking_change:         false, author: "Bruce Guenter", files_count:      1, insertions_count:  10, deletions_count:  16},
		{sha: "234e4665ad0c80fee1c4dfeafe1887e97a012c7a", date: "2021-03-17 10:41:15 UTC", description: "Remap function `to_timestamp` panics on an out of range integer", pr_number: 6777, scopes: ["remap"], type:                    "fix", breaking_change:         false, author: "Vladimir Zhuk", files_count:      1, insertions_count:  54, deletions_count:  8},
		{sha: "c3f4a50c1ab5e0f0589db60c24a4177ff090e94c", date: "2021-03-18 15:03:32 UTC", description: "Add `events_in_total` to sources", pr_number:                                6758, scopes: ["observability"], type:            "enhancement", breaking_change: false, author: "Kruno Tomola Fabro", files_count: 51, insertions_count: 280, deletions_count: 46},
		{sha: "3618ea5ecd928467a6bf5e66e72cfd0a43158e07", date: "2021-03-18 17:39:50 UTC", description: "Fix when to validate healthchecks", pr_number:                               6810, scopes: ["config"], type:                   "fix", breaking_change:         false, author: "Kruno Tomola Fabro", files_count: 1, insertions_count:  1, deletions_count:   1},
		{sha: "ab0beeeb5b2e7736816a529dce8f18bef8342414", date: "2021-03-18 15:23:54 UTC", description: "Correct the glibc requirements for packages", pr_number:                     6774, scopes: ["releasing"], type:                "enhancement", breaking_change: false, author: "Jesse Szwedko", files_count:      4, insertions_count:  24, deletions_count:  8},
	]
}
