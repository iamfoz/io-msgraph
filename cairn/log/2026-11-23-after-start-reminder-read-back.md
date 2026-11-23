---
cairn: log
change: after-start-reminder-read-back
landed: 2026-11-23
---

# After-start reminder read back

The projection wrote a reminder firing after the start as `TRIGGER:PT480M` and read that alarm back as one Graph cannot hold: it was stashed and the reminder read as off, so an after-start reminder did not survive an edit through iCalendar.

A DISPLAY or AUDIO alarm with a duration trigger counted from the start now reads as the reminder whichever side of the start it falls on: `-PT15M` as 15, `PT8H` and `+PT8H` as -480, `PT0M` as 0. An alarm counted from the end (`RELATED=END`) is stashed, where one before the end was read as that many minutes before the start, which is another instant. An absolute trigger and one that is not whole minutes are stashed as before.

Tests `a_reminder_reads_back_from_its_trigger` and `a_trigger_reads_as_minutes_from_the_start`. Landed without a proposal.

Capabilities moved: none.
