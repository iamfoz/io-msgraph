---
cairn: log
change: the-layered-features
landed: 2026-11-21
---

# The layered features

Landed without a proposal (the family's cargo-006 feature shape, the one io-imap, io-smtp and io-webdav ship, applied as a packaging change).

**Features.** The old `client` feature, which gated the blocking client and std, is renamed `std`. `client` now gates the coroutines (`coroutine`, `v1::send`, `v1::page`, `v1::rest::batch` and every operation module under `v1::rest::users`) and makes io-http, base64-codec, log and url optional; it is on by default and implied by `std`, which the TLS features imply. The vocabulary stays compiled under no feature: the resource types, `v1::field`, `v1::query`, and the `ical` and `vcard` projections under their own feature. `MSGRAPH_API_BASE`, `user_path`, `path_segment`, the send errors and `MsgraphPage` live in `v1::send` and `v1::page` beside the coroutines, so they need `client` too.

**Verified.** Builds with no features, `client`, `std`, default, all features, and no features plus `ical`, `vcard`, `schemars` are warning-free; `cargo test --all-features` and the lib tests with `ical`, `vcard`, `schemars` alone pass; clippy over all features and all targets, rustdoc with warnings denied and `cargo fmt --check` are clean. CI runs `cargo test --all-features`, valid.

Capabilities moved: none (no spec states the feature layering).
