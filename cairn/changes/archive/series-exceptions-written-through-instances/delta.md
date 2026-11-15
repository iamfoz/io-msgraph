---
cairn: delta
change: series-exceptions-written-through-instances
---

# Delta

## ADDED Requirements

### Requirement: A series is written with the changes of its instances
With the `ical` feature, a series document SHALL project onto the write of its master and the change of each instance the document sets apart from its rule, made once the master is written: a delete for an occurrence it cancels, by an EXDATE or by an override whose STATUS is CANCELLED, and a patch for an occurrence it overrides, holding the fields the override sets differently from the instance as Graph holds it, never its recurrence nor its stash. For a create, every occurrence SHALL be taken to follow the series, carried to its original start. For an update, the instance SHALL be taken as `base` holds it: its exception, or the series carried there; an occurrence `base` overrides and the document does not SHALL be patched back to the series carried there, and one both cancel SHALL take no change. An EXDATE and an override of the same occurrence SHALL cancel it. An update whose master patch moves the series (its start, end, all-day flag or recurrence) SHALL take the exceptions and cancelled occurrences of `base` as dropped, Graph not keeping them across such a patch, and SHALL change the instances as after a create.

#### Scenario: A series with one EXDATE and one override
- GIVEN a daily series of four from 2026-11-23 14:00 in Europe/Paris, an EXDATE on 2026-11-25 and an override moving 2026-11-24 to 16:00
- WHEN it is projected for a create
- THEN the master is the create body `create_from_ical` gives, and the changes, in the order of their original start, are a patch of the instance originally starting 2026-11-24T13:00:00Z holding its new start and end alone, then a delete of the one originally starting 2026-11-25T13:00:00Z

### Requirement: A change names the instance it touches
Each instance change SHALL carry the occurrence's original start as Graph's `originalStart` tells it, an RFC 3339 UTC instant, or the date for an all-day series, and the window `event_instances` lists it within: from the day before that start, or before the start `base` moved its exception to, to two days after the later of them, at midnight UTC. A listed instance SHALL be the one a change names when its `originalStart` is the same instant, or falls on the same UTC date for an all-day series.

### Requirement: What Graph cannot hold of a series is named
The series projection SHALL drop nothing silently: it SHALL name, as its logical line, every RDATE and EXRULE of the master; every EXDATE value and RECURRENCE-ID whose instant does not read; every RECURRENCE-ID with a RANGE; every override and EXDATE beside a master that is no series; every occurrence `base` cancels and the document no longer does, Graph having no call restoring one; and every unmanaged line of an override other than its UID, an instance patch carrying no stash.

## MODIFIED Requirements

None.

## REMOVED Requirements

None.
