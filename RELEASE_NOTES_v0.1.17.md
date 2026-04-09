# macpow v0.1.17

Release date: 2026-04-07

## Highlights

- Improved runtime stability for background samplers:
  - graceful shutdown for worker threads
  - explicit cleanup for IOReport native resources
- Hardened subprocess usage:
  - added execution timeouts for external commands
  - made `nettop` PID parsing stricter and more resilient
- Better test reliability:
  - expanded unit tests for pure computation paths
  - CI now runs unit tests (`cargo test --lib`) instead of compile-only checks
- Cleaner library/CLI boundaries:
  - moved CLI args out of library types
  - added explicit macOS-only gating for the crate

## Fixes

- Fixed periodic Dock flashing caused by EDR probing subprocesses.
  - Root cause: the EDR Python probe called `NSApplication sharedApplication`, which made subprocesses appear as GUI apps.
  - Fix: removed `sharedApplication` call and added safe screen fallbacks for EDR reads.
  - Tracked issue: [#9](https://github.com/k06a/macpow/issues/9)

## Notes for maintainers

- This release includes changes in:
  - main crate (`macpow`)
  - Homebrew formula (`homebrew-tap`)
  - conda feedstock (`macpow-feedstock`)
- Ensure release artifact SHA256 matches the tagged GitHub source tarball before publishing formula/feedstock updates.
