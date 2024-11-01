#![deprecated = "Use colog with the `log!()` macro instead"]
const YELLOW: &str = "\x1b[38;5;220m";
const BLUE: &str = "\x1b[38;5;57m";
const RED: &str = "\x1b[38;5;197m";
#[derive(Default)]
pub enum ArrowType {
    #[default]
    Info,
    Warning,
    Error,
    Huh,
}

impl ArrowType {
    pub fn get_arrow(&self) -> String {
        match self {
            ArrowType::Info => format!("{BLUE}==>"),
            ArrowType::Warning => format!("{YELLOW}=!!"),
            ArrowType::Error => format!("{RED}=!!"),
            ArrowType::Huh => format!("{YELLOW}=??"),
        }
    }
}
