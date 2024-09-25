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
    pub fn set_type(self, t: Self) -> String {
        match t {
            ArrowType::Info => format!("{BLUE}==>").to_string(),
            ArrowType::Warning => format!("{YELLOW}=!!").to_string(),
            ArrowType::Error => format!("{RED}=!!").to_string(),
            ArrowType::Huh => format!("{YELLOW}=??").to_string(),
        }
    }
}
