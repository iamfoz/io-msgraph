---
cairn: log
change: after-start-reminder-trigger
landed: 2026-11-23
---

# After-start reminder trigger

An event whose reminder is on with a negative `reminderMinutesBeforeStart` (Graph's spelling of a reminder firing after the start) projected its alarm as `TRIGGER:-PT-480M`, which is not an RFC 5545 duration. It now projects as the positive duration counted from the start, `TRIGGER:PT480M`. A reminder of zero or more minutes is unchanged, zero staying `-PT0M`, which the projection reads back as the reminder.

Not handled here: reading back. An alarm triggering after the start is still stashed rather than read as a negative reminder, as it was before.

Test `a_reminder_is_a_trigger_counted_from_the_start`. Landed without a proposal.

Capabilities moved: none.
