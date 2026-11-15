---
cairn: log
change: instances-read-in-windows-graph-accepts
landed: 2026-11-15
---

# Instances read in windows Graph accepts

Graph refuses an `instances` listing over more than five years (400 `ErrorInvalidRequest`, "Maximum number of years: 5"), which neverest (2026-10-04) and calendula ([calendula#31](https://github.com/pimalaya/calendula/issues/31)) each met and worked around on their own. Under `ical`, `MsgraphPatternedRecurrence::instance_windows(within: (Date, Date))` now gives the windows a series' instances are read in, and `MSGRAPH_INSTANCES_MAX_YEARS` (5) names the limit, both in src/v1/rest/users/events/ical.rs. It ports neverest's `series_windows` without its policy: the series' `bounds`, its end made exclusive (one day past `endDate`), an open-ended series ending at `within.1`, cut to `within` and split into consecutive windows of five years less a day, each as the `%Y-%m-%dT%H:%M:%SZ` instants `event_instances` takes; `None` when the bounds do not read, no window when the series and `within` do not meet. The caller picks `within`, the crate reading no clock; `event_instances` is unchanged.

Unit tests cover a bounded series inside the range, a twelve-year series cut into windows under the limit, an open-ended series ending with the range (the delta's scenario), series outside the range, the end date read whole, a numbered series, and unreadable bounds. The live `ical_instance_windows` ran on the Microsoft test tenant on 2026-11-15 and passed: a weekly series from 2019-01-07 with no end listed every Monday of today plus or minus five years over three windows, and Graph answered 400 to the same range asked in one listing.

Left: the 0.9.1 release, to the owner; neverest dropping `series_windows` and calendula taking it (its change series-exceptions-follow-the-listed-range).

Capabilities moved: events-ical ("A series' instances are read in windows Graph accepts").
