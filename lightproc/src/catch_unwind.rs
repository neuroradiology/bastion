use pin_utils::unsafe_pinned;
use std::any::Any;
use std::future::Future;
use std::panic::{catch_unwind, AssertUnwindSafe, UnwindSafe};
use std::pin::Pin;
use std::task::{Context, Poll};

#[derive(Debug)]
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct CatchUnwind<Fut>
where
    Fut: Future,
{
    future: Fut,
}

impl<Fut> CatchUnwind<Fut>
where
    Fut: Future + UnwindSafe,
{
    unsafe_pinned!(future: Fut);

    pub(super) fn new(future: Fut) -> CatchUnwind<Fut> {
        CatchUnwind { future }
    }
}

impl<Fut> Future for CatchUnwind<Fut>
where
    Fut: Future + UnwindSafe,
{
    type Output = Result<Fut::Output, Box<dyn Any + Send>>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        catch_unwind(AssertUnwindSafe(|| self.future().poll(cx)))?.map(Ok)
    }
}