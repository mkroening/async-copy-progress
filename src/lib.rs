//! [![github]](https://github.com/mkroening/async-copy-progress)&ensp;[![crates-io]](https://crates.io/crates/async-copy-progress)&ensp;[![docs-rs]](https://docs.rs/async-copy-progress)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K
//!
//! <br>
//!
//! Asynchronous copying with progress callbacks.
//!
//! See [`copy`] for details.
//!
//! <br>
//!
//! # Example
//!
//! ```
//! # spin_on::spin_on(async {
//! # use std::sync::atomic::{AtomicU64, Ordering};
//! let mut reader: &[u8] = b"hello";
//! let mut writer: Vec<u8> = vec![];
//!
//! let progress = AtomicU64::new(0);
//! let report_progress = |amt| progress.store(amt, Ordering::Relaxed);
//!
//! async_copy_progress::copy(&mut reader, &mut writer, report_progress).await?;
//!
//! assert_eq!(&b"hello"[..], &writer[..]);
//! assert_eq!(5, progress.load(Ordering::Relaxed));
//! # std::io::Result::Ok(()) });
//! ```

#![deny(rust_2018_idioms)]

use std::{
    future::Future,
    io,
    pin::Pin,
    task::{Context, Poll},
};

use futures_core::ready;
use futures_io::{AsyncBufRead, AsyncWrite};
use pin_project_lite::pin_project;

/// Creates a future which copies all the bytes from one object to another while
/// reporting the progress.
///
/// The returned future will copy all the bytes read from `reader` into the
/// `writer` specified. After each write, `report_progress` is called with the
/// current amount of copied bytes. This future will only complete once the
/// `reader` has hit EOF and all bytes have been written to and flushed from the
/// `writer` provided.
///
/// On success the number of bytes is returned.
///
/// <br>
///
/// # Errors
///
/// This function will return an error immediately if any call to
/// [`poll_fill_buf`], [`poll_write`] or [`poll_flush`] returns an error.
///
/// [`poll_fill_buf`]: AsyncBufRead::poll_fill_buf
/// [`poll_write`]: AsyncWrite::poll_write
/// [`poll_flush`]: AsyncWrite::poll_flush
///
/// <br>
///
/// # Example
///
/// ```
/// # spin_on::spin_on(async {
/// # use std::sync::atomic::{AtomicU64, Ordering};
/// let mut reader: &[u8] = b"hello";
/// let mut writer: Vec<u8> = vec![];
///
/// let progress = AtomicU64::new(0);
/// let report_progress = |amt| progress.store(amt, Ordering::Relaxed);
///
/// async_copy_progress::copy(&mut reader, &mut writer, report_progress).await?;
///
/// assert_eq!(&b"hello"[..], &writer[..]);
/// assert_eq!(5, progress.load(Ordering::Relaxed));
/// # std::io::Result::Ok(()) });
/// ```
pub fn copy<R, W, F>(reader: R, writer: W, report_progress: F) -> Copy<R, W, F>
where
    R: AsyncBufRead,
    W: AsyncWrite,
    F: FnMut(u64),
{
    Copy {
        reader,
        writer,
        amt: 0,
        report_progress,
    }
}

pin_project! {
    /// Future for the [`copy`] function.
    #[derive(Debug)]
    #[must_use = "futures do nothing unless you `.await` or poll them"]
    pub struct Copy<R, W, F> {
        #[pin]
        reader: R,
        #[pin]
        writer: W,
        amt: u64,
        report_progress: F,
    }
}

impl<R, W, F> Future for Copy<R, W, F>
where
    R: AsyncBufRead,
    W: AsyncWrite,
    F: FnMut(u64),
{
    type Output = io::Result<u64>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut this = self.project();
        loop {
            let buffer = ready!(this.reader.as_mut().poll_fill_buf(cx))?;
            if buffer.is_empty() {
                ready!(this.writer.poll_flush(cx))?;
                return Poll::Ready(Ok(*this.amt));
            }

            let i = ready!(this.writer.as_mut().poll_write(cx, buffer))?;
            if i == 0 {
                return Poll::Ready(Err(io::ErrorKind::WriteZero.into()));
            }
            *this.amt += i as u64;
            this.reader.as_mut().consume(i);
            (this.report_progress)(*this.amt);
        }
    }
}
