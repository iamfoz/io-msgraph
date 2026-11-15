---
cairn: change
id: instances-read-in-windows-graph-accepts
status: landed
created: 2026-11-15
---

# Instances read in windows Graph accepts

## Why

Graph refuses an `instances` listing whose range spans more than five years: HTTP 400 `ErrorInvalidRequest`, "The range between the start and end dates is greater than the allowed range. Maximum number of years: 5". The crate takes the range as two strings and says nothing of the limit, so every consumer meets it on its own:

- neverest hit it on 2026-10-04 (its log graph-series-in-five-year-windows) and wrote `series_windows` (src/msgraph/client/calendar.rs): windows of five years less a day, an open-ended series read within five years either side of today.
- calendula hit it again ([calendula#31](https://github.com/pimalaya/calendula/issues/31)): its `series_window` (src/msgraph/backend.rs) runs an open-ended series five years and a day past its start, and leaves a bounded one uncapped.

The limit is Graph's, so cutting a range to fit it is a primitive of the Graph client, not of each app.

## What

- **The windows.** With the `ical` feature, which already gives `MsgraphPatternedRecurrence::bounds`, add `MsgraphPatternedRecurrence::instance_windows(&self, within: (Date, Date)) -> Option<Vec<(String, String)>>`:
  - the series' bounds, an open-ended series ending at `within.1`, intersected with `within`, the series' end made exclusive (one day past `endDate`);
  - cut into consecutive windows of five years less a day, the RFC 3339 UTC instants `event_instances` takes;
  - `None` when the bounds do not read, an empty list when the series and `within` do not meet.
- **The constant.** `MSGRAPH_INSTANCES_MAX_YEARS = 5`, public, named after Graph's message.
- **No client change.** `event_instances` keeps its signature: a caller walks the windows, then each window's pages as today.
- **Who picks `within`.** The caller: neverest passes today ± five years, calendula the listed range or today ± five years. The crate does not read the clock.

## Scope / non-goals

- No `event_instances` variant walking the windows itself: the caller already walks pages and decides what a failed window means.
- `calendarView` has the same limit and is left out: no consumer asks it for more than a listed range.
- Released as io-msgraph 0.9.1, additive.
