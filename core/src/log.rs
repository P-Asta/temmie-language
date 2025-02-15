use colorfully::*;

pub struct Logging {
    path: String,
}

impl Logging {
    pub fn new(path: String) -> Self {
        Self { path }
    }
    pub fn error(&self, position: (usize, usize), message: String) -> ! {
        println!(
            "[{} {}] {}",
            "ERROR".bold().red(),
            format!("{}:{}:{}", self.path, position.0, position.1).gray(),
            message
        );
        std::process::exit(1)
    }
}
