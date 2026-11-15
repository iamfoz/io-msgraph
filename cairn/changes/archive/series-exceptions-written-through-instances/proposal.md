---
cairn: change
id: series-exceptions-written-through-instances
status: landed
created: 2026-11-15
---

# Series exceptions written through instances

## Why

The live testing campaign of 2026-11-15 found a recurring series losing its exceptions on its way to Graph, with no warning (.github cairn changes/live-testing-2026-11-15, reports/calendula: msgraph-microsoft.md M-F1, part2-pimdir.md P2-F5):

- `calendula -a microsoft event create` of a weekly series with an EXDATE and an override (a RECURRENCE-ID moving one occurrence) lands as the master alone: the excluded occurrence and the moved one both read at their rule's times.
- `neverest sync -s msgraph-calendar` pushes the same series the same way; the next sync reads the reduced series back and rewrites the store copy, so the user's write is gone from both sides, no report line saying so.

Both go through the crate's projection, which is where the loss is:

- src/v1/rest/users/events/ical.rs:210-215, `MsgraphEvent::from_ical` (and through it `create_from_ical` and `update_from_ical`) reads the VEVENT with no RECURRENCE-ID and nothing else: every override is skipped.
- src/v1/rest/users/events/ical.rs:557, `project` consumes an EXDATE and keeps nothing of it, by design (a stashed copy would outlive Graph's own cancelled occurrences).

The module header already says Graph takes an exception and a cancelled occurrence only through the instance, and `instance_update_from_ical` projects one occurrence onto its instance's update, the live test `ical_instances` showing both the instance patch and the instance delete reading back as written. What is missing is the step from one document to those writes: a caller holding a series document has no way to learn which instances to touch, with what, nor what Graph cannot take, so each caller writes the master and stops.

## What

Under `ical`, in src/v1/rest/users/events/ical.rs:

- **`MsgraphEventSeries`**, a series document cut the way Graph takes it: `master`, the create body or the update patch of the series master; `instances`, the change of each instance the document cancels or overrides, made once the master is written; `left_out`, the logical line of each thing the document holds that Graph cannot hold, never dropped silently.
- **`MsgraphEventSeries::create_from_ical(contents)`**: the master as `MsgraphEvent::create_from_ical` gives it, then, every occurrence of a series just created following its master, a delete for each occurrence the document cancels (an EXDATE, an override whose STATUS is CANCELLED) and a patch for each one it overrides, holding what the override sets differently from the series carried to that occurrence.
- **`MsgraphEventSeries::update_from_ical(contents, base)`**: the master as `MsgraphEvent::update_from_ical` gives it, and the instance changes against `base`, the series as Graph holds it (`to_ical_series`): a delete for an occurrence newly cancelled, a patch for an override added or edited, against the base's exception or the series carried there, and a patch carrying the series back to an occurrence whose override the document dropped.
- **`MsgraphEventInstanceChange`**: `original_start`, the occurrence's original start as Graph's `originalStart` tells it (an RFC 3339 UTC instant, a date for an all-day series); `window`, the `start` and `end` `event_instances` lists it within; `action`, `MsgraphEventInstanceAction::Delete` or `Update(Box<MsgraphEvent>)`; and `is_instance(&MsgraphEvent)`, whether a listed instance is that occurrence, by its `originalStart`.
- **What `left_out` names**: an EXDATE, a RECURRENCE-ID or an EXDATE value whose instant does not read (a zone neither table knows); a RECURRENCE-ID with a RANGE (Graph changes one instance at a time); an occurrence `base` cancels and the document no longer does (Graph has no call restoring a cancelled occurrence); an exception or an EXDATE beside a master Graph holds as no series; an RDATE or an EXRULE (Graph's pattern has neither, the stash keeping the line but Graph's own occurrences leaving it out); an unmanaged line of an override other than its UID (an instance patch sends no stash).
- `instance_update_from_ical` and the series' patches share one projection of an occurrence onto its instance, so the two cannot drift.

A caller writes the master, then, for each change, lists the instances of the series in `window` with `MSGRAPH_EVENT_ICAL_SELECT` (the default listing leaves `originalStart` out), takes the one `is_instance` names and deletes or patches it, and warns of or refuses `left_out`.

## Decision for the owner: delete, not cancel

A cancelled occurrence is carried as a **delete of its instance** (`DELETE /events/{instance id}`), not the `cancel` action (`POST /events/{id}/cancel`). The delete is what lands the occurrence in the master's `cancelledOccurrences`, which the projection reads back as an EXDATE (live test `ical_instances`); it works on any event, where `cancel` is the organizer's meeting action, refused (400) to an attendee and carrying a comment no EXDATE has. Graph documents that deleting a meeting on the organizer's calendar sends the attendees a cancellation, so an EXDATE pushed on an organised meeting notifies them, as an occurrence removed in Outlook does. If the owner wants a silent removal on meetings, or the `cancel` action with its comment, the change is in `MsgraphEventInstanceAction`, not in the projection.

## Scope / non-goals

- No coroutine: the four calls a caller needs exist (`event_create`, `event_update`, `event_instances`, `event_delete`), and how a caller treats a failed instance write (retry, refuse, warn) is its policy.
- A master patch moving the series (start, end, all-day flag or recurrence) is taken to drop its exceptions and cancelled occurrences, as Exchange does on a pattern change and as neverest assumes (776fca9, "a moved start or rule drops Graph's cancellations"): `update_from_ical` then changes the instances as after a create, a delete whose instance is no longer listed being done already. Not verified live in this session.
- No edit to calendula or neverest: their callers keep `create_from_ical` and `update_from_ical` until they move (notes in the log entry).
- No release: left to the owner.
