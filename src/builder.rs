use super::Progress;

/// **Deprecation**: Using this directly as `progress::builder::ProgressBuilder`
/// is deprecated, please use `progress::Builder` instead.
#[derive(Default)]
pub struct ProgressBuilder {
    starting_value: Option<usize>,
    ending_value: Option<usize>,

    caption: Option<String>,
}

impl ProgressBuilder {
    /// Creates a new `Builder` with default values.
    pub fn new() -> Self {
        ProgressBuilder::default()
    }

    /// Will build the `Progress`.
    pub fn build(self) -> Progress {
        Progress {
            current: self.starting_value.unwrap_or(0),
            total: self.ending_value.unwrap_or(100),
            caption: self.caption.unwrap_or("Progress".to_string()),
            started: false,
        }
    }

    /// Sets the starting value of the `Progress`.
    pub fn set_start(self, v: usize) -> Self {
        ProgressBuilder { starting_value: Some(v),
                          .. self }
    }

    /// Sets the maximum value of the `Progress`.
    pub fn set_finish(self, v: usize) -> Self {
        ProgressBuilder { ending_value: Some(v),
                          .. self }
    }

    /// Sets the caption for the `Progress`.
    pub fn set_caption(self, v: String) -> Self {
        ProgressBuilder { caption: Some(v),
                          .. self }
    }
}
