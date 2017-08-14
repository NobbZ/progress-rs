use super::Progress;

#[derive(Default)]
pub struct ProgressBuilder {
    starting_value: Option<usize>,
    ending_value: Option<usize>,

    caption: Option<String>,
}

impl ProgressBuilder {
    pub fn new() -> Self {
        ProgressBuilder::default()
    }

    pub fn build(self) -> Progress {
        Progress {
            current: self.starting_value.unwrap_or(0),
            total: self.ending_value.unwrap_or(100),
            caption: self.caption.unwrap_or("Progress".to_string()),
            started: false,
        }
    }

    pub fn set_start(self, v: usize) -> Self {
        ProgressBuilder { starting_value: Some(v),
                          .. self }
    }

    pub fn set_finish(self, v: usize) -> Self {
        ProgressBuilder { ending_value: Some(v),
                          .. self }
    }

    pub fn set_caption(self, v: String) -> Self {
        ProgressBuilder { caption: Some(v),
                          .. self }
    }
}
