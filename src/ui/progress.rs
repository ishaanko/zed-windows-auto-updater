use std::io::{self, Write};

pub struct ProgressBar {
    total: u64,
    current: u64,
}

impl ProgressBar {
    pub fn new(total: u64) -> Self {
        ProgressBar { total, current: 0 }
    }

    pub fn update(&mut self, increment: u64) {
        self.current += increment;
        self.display();
    }

    pub fn display(&self) {
        let percentage = (self.current as f64 / self.total as f64) * 100.0;
        let bar_length = 50;
        let filled_length = (bar_length as f64 * (self.current as f64 / self.total as f64)) as usize;
        let bar = "█".repeat(filled_length) + &" ".repeat(bar_length - filled_length);
        
        print!("\r|{}| {:.2}%", bar, percentage);
        io::stdout().flush().unwrap();
    }

    pub fn finish(&self) {
        println!("\nUpdate download complete!");
    }
}