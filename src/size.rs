#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SizeUnit {
    Centimeter(f64),
}

impl ToString for SizeUnit {
    fn to_string(&self) -> String {
        match self {
            SizeUnit::Centimeter(value) => format!("{value} cm"),
        }
    }
}
