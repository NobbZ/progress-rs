extern crate terminal_size;

pub mod builder;
pub use builder::ProgressBuilder as Builder;

mod progress;
pub use progress::Progress;
