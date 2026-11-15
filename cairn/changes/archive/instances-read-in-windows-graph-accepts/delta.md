---
cairn: delta
change: instances-read-in-windows-graph-accepts
---

# Delta

## ADDED Requirements

### Requirement: A series' instances are read in windows Graph accepts
With the `ical` feature, a series' recurrence SHALL give the windows its instances are read in within a range the caller names: the series' bounds, an open-ended series ending where the range does, intersected with that range, its last day included, cut into windows of five years less a day, each as the RFC 3339 UTC instants `event_instances` takes. Graph refuses an `instances` listing over more than five years, so no window SHALL exceed it. A series the range does not meet SHALL give no window, and one whose bounds do not read none at all.

#### Scenario: An open-ended series older than five years
- GIVEN a weekly series starting 2019-01-07 with no end
- WHEN its windows are asked within 2021-11-15 to 2031-11-15
- THEN they start on 2021-11-15, end on 2031-11-15, and none spans five years or more
