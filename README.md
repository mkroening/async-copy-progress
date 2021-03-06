async-copy-progress
===================

[<img alt="github" src="https://img.shields.io/badge/github-mkroening/async--copy--progress-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/mkroening/async-copy-progress)
[<img alt="crates.io" src="https://img.shields.io/crates/v/async-copy-progress.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/async-copy-progress)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-async--copy--progress-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K" height="20">](https://docs.rs/async-copy-progress)
[<img alt="build status" src="https://img.shields.io/github/workflow/status/mkroening/async-copy-progress/CI/main?style=for-the-badge" height="20">](https://github.com/mkroening/async-copy-progress/actions?query=branch%3Amain)

Asynchronous copies with progress updates.

This library provides an asynchronous copy function which calls a function with the current progress after each step.

```toml
[dependencies]
async-copy-progress = "1.0"
```

<br>

## Example

```rust
let mut reader: &[u8] = b"hello";
let mut writer: Vec<u8> = vec![];

let progress = AtomicU64::new(0);
let report_progress = |amt| progress.store(amt, Ordering::Relaxed);

async_copy_progress::copy(&mut reader, &mut writer, report_progress).await?;

assert_eq!(&b"hello"[..], &writer[..]);
assert_eq!(5, progress.load(Ordering::Relaxed));
```

<br>

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
