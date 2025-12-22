use std::sync::Arc;
use tokio::sync::Mutex;

/// Macro that Boxes, Mutexes and Arcs the argument passed.
#[macro_export]
macro_rules! bmarc {
    ($val:expr) => {{ ::std::sync::Arc::new(::tokio::sync::Mutex::new(::std::boxed::Box::new($val))) }};
}

/// Macro that Boxes and Arcs the argument passed.
#[macro_export]
macro_rules! barc {
    ($val:expr) => {{ ::std::sync::Arc::new(::std::boxed::Box::new($val)) }};
}

/// Macro that Mutexes and Arcs the argument passed.
#[macro_export]
macro_rules! marc {
    ($val:expr) => {{ ::std::sync::Arc::new(::tokio::sync::Mutex::new($val)) }};
}

pub type BMArc<T> = Arc<Mutex<Box<T>>>;
pub type MArc<T> = Arc<Mutex<T>>;
pub type BArc<T> = Arc<Box<T>>;
