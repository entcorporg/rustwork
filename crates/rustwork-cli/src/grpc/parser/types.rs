/// Parser pour le DSL Rustwork (.rwk)
pub struct Parser {
    pub(super) source: String,
    pub(super) lines: Vec<String>,
    pub(super) current_line: usize,
}

impl Parser {
    pub fn new(source: impl Into<String>) -> Self {
        let source = source.into();
        let lines = source.lines().map(|l| l.to_string()).collect();
        Self {
            source,
            lines,
            current_line: 0,
        }
    }
}
