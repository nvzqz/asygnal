use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

#[inline]
pub fn poll_fn<T, F>(f: F) -> PollFn<F>
where
    F: FnMut(&mut Context<'_>) -> Poll<T>,
{
    PollFn { f }
}

pub struct PollFn<F> {
    f: F,
}

impl<F> Unpin for PollFn<F> {}

impl<T, F> Future for PollFn<F>
where
    F: FnMut(&mut Context<'_>) -> Poll<T>,
{
    type Output = T;

    #[inline]
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<T> {
        (&mut self.f)(cx)
    }
}
