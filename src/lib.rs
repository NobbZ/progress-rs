extern crate terminal_size;

pub mod builder;
#[allow(deprecated)]
pub use builder::ProgressBuilder as Builder;

mod progress;
pub use progress::Progress;
