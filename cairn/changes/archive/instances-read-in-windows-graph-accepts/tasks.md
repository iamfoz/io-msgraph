---
cairn: tasks
change: instances-read-in-windows-graph-accepts
---

- [x] `MSGRAPH_INSTANCES_MAX_YEARS` and `MsgraphPatternedRecurrence::instance_windows`, under `ical`
- [x] Unit tests: a bounded series inside `within`, one longer than five years cut into windows none over the limit, an open-ended series ending at `within.1`, a series outside `within` (empty), an end date made exclusive, a numbered series bounded by its occurrences
- [x] Live test on the test tenant: an open-ended series older than five years lists its instances over today ± five years with no 400
- [x] Docs: the module header, CHANGELOG.md (Added)
- [x] Fold into cairn/spec/events-ical.md, log, archive
- [ ] Release 0.9.1, left to the owner
- [ ] neverest drops `series_windows` for it; calendula takes it (its change series-exceptions-follow-the-listed-range)
