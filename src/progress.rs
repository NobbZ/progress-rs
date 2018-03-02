use std::iter::repeat;
use std::io::{stdout, Write};
use std::ops::{AddAssign, SubAssign};
use terminal_size::{terminal_size, Height, Width};

/// Tracks progress of a task.
///
/// There should be not more than a single instance started at a given point in
/// time, it will mangle your terminal output.
pub struct Progress {
    current: usize,
    total: usize,

    caption: String,

    width: Option<u16>,

    started: bool,
}

impl Default for Progress {
    fn default() -> Progress {
        Progress::new("Progress", 0, 100)
    }
}

impl Progress {
    pub fn new<S: Into<String>>(caption: S, start: usize, end: usize) -> Self {
        Progress {
            current: start,
            total: end,
            caption: caption.into(),
            width: None,
            started: false,
        }
    }

    pub fn new_with_width<S>(caption: S, start: usize, end: usize, width: u16) -> Self
    where
        S: Into<String>,
    {
        Progress {
            current: start,
            total: end,
            caption: caption.into(),
            width: Some(width),
            started: false,
        }
    }

    /// Returns the current progress absolute value.
    ///
    /// # Example
    ///
    /// ```
    /// use progress::Progress;
    /// let p = Progress::default();
    /// assert_eq!(0, p.current());
    /// ```
    pub fn current(&self) -> usize {
        self.current
    }

    /// Returns the total value.
    ///
    /// # Example
    ///
    /// ```
    /// use progress::Progress;
    /// let p = Progress::default();
    /// assert_eq!(100, p.total());
    /// ```
    pub fn total(&self) -> usize {
        self.total
    }

    /// Advances the Progress by a certain amount.
    ///
    /// # Examplw
    ///
    /// ```
    /// use progress::Progress;
    /// let mut p = Progress::default();
    /// assert_eq!(0, p.current());
    /// p.forward(10);
    /// assert_eq!(10, p.current());
    /// p += 20;
    /// assert_eq!(30, p.current());
    /// ```
    pub fn forward(&mut self, step: usize) -> &Self {
        *self += step;
        self
    }

    /// Advances the Progress by a certain amount.
    ///
    /// # Examplw
    ///
    /// ```
    /// use progress::Builder;
    /// let mut p = Builder::new().set_start(30).build();
    /// assert_eq!(30, p.current());
    /// p.backward(10);
    /// assert_eq!(20, p.current());
    /// p -= 20;
    /// assert_eq!(0, p.current());
    /// ```
    pub fn backward(&mut self, step: usize) -> &Self {
        *self -= step;
        self
    }

    /// Advances the Progress by exactly one.
    pub fn increment(&mut self) -> &Self {
        self.forward(1)
    }

    /// Does a step backwards at the Progress.
    pub fn decrement(&mut self) -> &Self {
        self.backward(1)
    }

    /// Determines wheter a Progress has finished (reached the total) or not.
    pub fn finished(&self) -> bool {
        self.current >= self.total
    }

    /// Returns the relative value of the Progress.
    pub fn process(&self) -> u8 {
        (self.current * 100 / self.total) as u8
    }

    /// Activates the Progress.
    pub fn start(&mut self) -> &Self {
        self.started = true;
        self
    }

    /// Returns the current caption.
    pub fn caption(&self) -> &String {
        &self.caption
    }
}

impl AddAssign<usize> for Progress {
    fn add_assign(&mut self, step: usize) {
        let new = self.current + step;
        if new > self.total {
            self.current = self.total;
        } else {
            self.current += step;
        }
        print_bar(self);
    }
}

impl SubAssign<usize> for Progress {
    fn sub_assign(&mut self, step: usize) {
        self.current = self.current.saturating_sub(step);
        print_bar(self);
    }
}

fn print_bar(p: &Progress) {
    const COLONS: usize = 1;
    const SPACES: usize = 2;
    const PIPES: usize = 2;
    const OTHER: usize = COLONS + SPACES + PIPES;
    let p_info = format!(
        "{current} / {total} ({process})",
        current = p.current(),
        total = p.total(),
        process = p.process()
    );
    let caption = p.caption();

    let terminal_width = p.width
        .unwrap_or_else(|| terminal_size().unwrap_or((Width(79), Height(0))).0 .0);

    let bar_width = terminal_width as usize // Width of terminal
        - p_info.len()  // Width of right summary
        - caption.len() // Width of caption
        - OTHER;
    let done_width = (bar_width * p.process() as usize) / 100;
    let todo_width = bar_width - done_width;
    let done_bar = repeat('#').take(done_width).collect::<String>();
    let todo_bar = repeat('-').take(todo_width).collect::<String>();
    let bar = format!("|{}{}|", done_bar, todo_bar);

    print!(
        "{caption}: {bar} {info}\r",
        caption = caption,
        bar = bar,
        info = p_info
    );
    stdout().flush().unwrap();
}
