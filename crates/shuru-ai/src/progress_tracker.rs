use console::style;

pub struct ProgressTracker {
    total: usize,
    current: usize,
}

impl ProgressTracker {
    pub fn new(total: usize) -> Self {
        Self { total, current: 0 }
    }

    pub fn update(&mut self, action_desc: &str, is_interactive: bool) {
        self.current += 1;

        if !is_interactive {
            println!(
                "{} {} [{}/{}]",
                style("[>]").bold().blue(),
                action_desc,
                self.current,
                self.total
            );
        }
    }

    pub fn complete_action(&mut self, success: bool, message: Option<&str>, is_interactive: bool) {
        if !is_interactive {
            println!(
                "{} {}",
                if success {
                    style("✓").bold().green()
                } else {
                    style("✗").bold().red()
                },
                message.unwrap_or(if success { "Completed" } else { "Failed" })
            );
        }
    }
}
