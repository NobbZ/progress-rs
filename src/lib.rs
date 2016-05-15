use std::io::Write;

/// Tracks progress of a task
pub struct Progress {
    current: usize,
    total: usize,

    started: bool
}

impl Progress {
    /// Creates a new one
    pub fn new(current: usize, total: usize) -> Self {
        Progress { current: current, total: total, started: false }
    }

    /// Creates a new progress with default values.
    pub fn default() -> Self {
        Progress::new(0, 100)
    }

    pub fn current(&self) -> usize {
        self.current
    }

    pub fn total(&self) -> usize {
        self.total
    }

    pub fn increment(&mut self) -> &Self {
        if self.current < self.total {
            self.current += 1;
        }
        print_bar(self);
        self
    }

    pub fn decrement(&mut self) -> &Self {
        if self.current > 0 {
            self.current -= 1;
        }
        self
    }

    pub fn finished(&self) -> bool {
        self.current >= self.total
    }

    pub fn process(&self) -> u8 {
        (self.current * 100 / self.total) as u8
    }

    pub fn start(&mut self) -> &Self {
        self.started = true;
        self
    }
}

fn print_bar(p: &Progress) {
    let p_info = format!("{current} / {total} ({process})",
                         current = p.current(),
                         total   = p.total(),
                         process = p.process());
    let caption = "Progress";

    let bar_width  = 79 - p_info.len() - caption.len() - 3 - 2;
    let done_width = (bar_width * p.process() as usize) / 100;
    let todo_width = bar_width - done_width;
    let done_bar   = std::iter::repeat("#").take(done_width).collect::<String>();
    let todo_bar   = std::iter::repeat("-").take(todo_width).collect::<String>();
    let bar = format!("|{}{}|", done_bar, todo_bar);

    print!("{caption}: {bar} {info}\r",
           caption = caption, bar = bar, info = p_info);
    std::io::stdout().flush().unwrap();
}
