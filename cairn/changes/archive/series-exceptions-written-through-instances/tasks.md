---
cairn: tasks
change: series-exceptions-written-through-instances
---

- [x] Regression tests first, failing on the master-only projection: a series with one EXDATE and one override yields the master plus one delete and one patch; an update adding an EXDATE and editing an override; a reverted override; a restored occurrence, a RANGE and an RDATE named in `left_out`; the instance a change names picked from a listing; the requests a caller sends, on the scripted coroutines; an update moving the series writes its instances again
- [x] `MsgraphEventSeries`, `MsgraphEventInstanceChange`, `MsgraphEventInstanceAction`, under `ical`
- [x] `instance_update_from_ical` on the shared occurrence projection
- [x] Docs: the module header, CHANGELOG.md (Added)
- [x] `cargo test --all-features`, `cargo clippy --all-features -- -D warnings`, `cargo fmt`, `cargo build --no-default-features`
- [x] Fold into cairn/spec/events-ical.md, log, archive
- [ ] Live test on the test tenant: `ical_series_with_its_instances` (written, not run), and that a master patch moving the series drops its exceptions (left to the owner, no live account in this session)
- [ ] calendula and neverest move to `MsgraphEventSeries` (notes in the log entry)
- [ ] Release, left to the owner
