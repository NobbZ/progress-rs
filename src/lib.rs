extern crate terminal_size;

use std::io::Write;

use terminal_size::{Width, Height, terminal_size};

pub mod builder;

/// Tracks progress of a task.
///
/// There should be not more than a single instance started at a given point in
/// time, it will mangle your terminal output.
pub struct Progress {
    current: usize,
    total: usize,

    caption: String,

    started: bool
}

impl Default for Progress {
    fn default() -> Progress {
        builder::ProgressBuilder::default().build()
    }
}

impl Progress {
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

    /// Advances the Progress by exactly one.
    pub fn increment(&mut self) -> &Self {
        if self.current < self.total {
            self.current += 1;
        }
        print_bar(self);
        self
    }

    /// Does a step backwards at the Progress.
    pub fn decrement(&mut self) -> &Self {
        if self.current > 0 {
            self.current -= 1;
        }
        self
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

fn print_bar(p: &Progress) {
    let p_info = format!("{current} / {total} ({process})",
                         current = p.current(),
                         total   = p.total(),
                         process = p.process());
    let caption = p.caption();

    let (Width(terminal_width), _) = terminal_size()
        .unwrap_or((Width(79), Height(0)));

    let bar_width  = terminal_width as usize // Width of terminal
        - p_info.len()  // Width of right summary
        - caption.len() // Width of caption
        - 3  // Colon and spaces
        - 2; // vertical bars in the progress meter
    let done_width = (bar_width * p.process() as usize) / 100;
    let todo_width = bar_width - done_width;
    let done_bar   = std::iter::repeat("#").take(done_width).collect::<String>();
    let todo_bar   = std::iter::repeat("-").take(todo_width).collect::<String>();
    let bar = format!("|{}{}|", done_bar, todo_bar);

    print!("{caption}: {bar} {info}\r",
           caption = caption, bar = bar, info = p_info);
    std::io::stdout().flush().unwrap();
}
