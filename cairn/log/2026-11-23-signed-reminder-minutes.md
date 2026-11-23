---
cairn: log
change: signed-reminder-minutes
landed: 2026-11-23
---

# Signed reminder minutes

Graph answers a negative `reminderMinutesBeforeStart` for a reminder set to fire after the start (-480 for eight hours after). The field was `MsgraphField<u32>`, so one such event failed the whole events page with `invalid value: integer -480, expected u32` and took the calendar listing down with it.

`MsgraphEvent::reminder_minutes_before_start` is now `MsgraphField<i32>`. The iCalendar projection is unchanged in behaviour: it still reads only an alarm triggering before the start as the reminder, now as an `i32`.

Not handled here: a negative reminder still exports as the trigger `-PT-480M`, which is not a valid iCalendar duration.

Test `event_with_negative_reminder_minutes_parses`. Landed without a proposal.

Capabilities moved: none.
