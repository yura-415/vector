---
title: VRL Quick Start
description: "Get up and running with Vector Remap Language"
---

## Syslog

```toml
[sources.logs]
type = "generator"
format = "syslog"
interval = 0.5

[transforms.shape]
type = "remap"
inputs = ["logs"]
source = '''
.
'''

[sinks.out]
type = "console"
inputs = ["shape"]
encoding.codec = "json"
```

```json
{"message":"<159>1 2021-01-18T19:37:39.213Z up.us Karimmove 9864 ID933 - Pretty pretty pretty good","timestamp":"2021-01-18T19:37:39.214072200Z"}
```

```toml
[transforms.shape]
type = "remap"
inputs = ["logs"]
source = '''
. = parse_syslog!(.message)
'''
```

## JSON

```toml
[sources.logs]
type = "generator"
format = "json"
interval = 0.5

[transforms.shape]
type = "remap"
inputs = ["logs"]
source = '''
.
'''

[sinks.out]
type = "console"
inputs = ["shape"]
encoding.codec = "json"
```

```toml
[transforms.shape]
type = "remap"
inputs = ["logs"]
source = '''
. = parse_json!(.message)
'''
```
