//! Task context common to all executors.
//!
//! Before executor, we "enter" it by setting up some necessary thread-locals.

/// Enters the tokio context if the `tokio` feature is enabled.

pub(crate) fn enter<T>(f: impl FnOnce() -> T) -> T {
    #[cfg(not(feature = "tokio02"))]
    return f();

    #[cfg(feature = "tokio02")]
    {
        use once_cell::sync::Lazy;
        use tokio::runtime::{Builder, Runtime};
        use std::env;

        static RT: Lazy<Runtime> = Lazy::new(|| Builder::new().threaded_scheduler().core_threads(env::var("ASYNC_STD_THREAD_COUNT")
        .map(|env| {
            env.parse()
                .expect("ASYNC_STD_THREAD_COUNT must be a number")
        })
        .unwrap_or(4)).build().expect("cannot initialize tokio"));

        RT.enter(f)
    }
}
