---
cairn: log
change: a-test-leaves-nothing-in-deleted-items
landed: 2026-11-19
---

# A test leaves nothing in Deleted Items

A plain delete of a calendar, a contact folder, a mail folder or a To Do list moves it into the mailbox's Deleted Items, where Graph no longer lists it as a mail folder: the live suite had piled up 1,087 such folders and some 900 items there by 2026-11-19, unreachable through `mailFolders/deleteditems/childFolders` (empty while `childFolderCount` counted them). Probed live that day: a calendar `DELETE` raised the count by one, a `POST .../permanentDelete` with a `{}` body left it unchanged, the same call answering 411 with no body and 400 with no `Content-Type`.

The crate now carries `MsgraphCalendarPermanentDelete`, `MsgraphContactFolderPermanentDelete`, `MsgraphMailFolderPermanentDelete` and `MsgraphMessagePermanentDelete`, and `MsgraphClientStd` their `*_permanent_delete` methods, each posting an empty JSON object. The live suite's cleanup uses them, so a run leaves in Deleted Items only its two To Do lists (Graph has no permanent delete for a list) and the messages it deletes to test the plain delete.

Test `permanent_delete_posts_an_empty_object`; live suite run 2026-11-19, 12 of 13 passing, `contacts` failing once on a list read right after a create (Graph's eventual consistency, unrelated, passing on two reruns). Additive, landed without a proposal.

Capabilities moved: none.
