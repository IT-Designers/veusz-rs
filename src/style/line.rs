use crate::api1::AsVueszApi1ValueStr;

/// https://github.com/veusz/veusz/blob/b06b5da124c7d712bafadfc86f75f474a655625c/veusz/setting/setting.py#L1424
#[derive(Copy, Clone, PartialOrd, PartialEq)]
pub enum LineStyle {
    Solid,
    Dashed,
    Dotted,
    DashDot,
    DashDotDot,
    DottedFine,
    DashedFine,
    DashDotFine,
    Dot1,
    Dot2,
    Dot3,
    Dot4,
    Dash1,
    Dash2,
    Dash3,
    Dash4,
    Dash5,
    DashDot1,
    DashDot2,
    DashDot3,
}

impl AsVueszApi1ValueStr for LineStyle {
    fn as_veusz_api1_value_str(&self) -> &str {
        match self {
            LineStyle::Solid => "solid",
            LineStyle::Dashed => "dashed",
            LineStyle::Dotted => "dotted",
            LineStyle::DashDot => "dash-dot",
            LineStyle::DashDotDot => "dash-dot-dot",
            LineStyle::DottedFine => "dotted-fine",
            LineStyle::DashedFine => "dashed-fine",
            LineStyle::DashDotFine => "dash-dot-fine",
            LineStyle::Dot1 => "dot1",
            LineStyle::Dot2 => "dot2",
            LineStyle::Dot3 => "dot3",
            LineStyle::Dot4 => "dot4",
            LineStyle::Dash1 => "dash1",
            LineStyle::Dash2 => "dash2",
            LineStyle::Dash3 => "dash3",
            LineStyle::Dash4 => "dash4",
            LineStyle::Dash5 => "dash5",
            LineStyle::DashDot1 => "dashdot1",
            LineStyle::DashDot2 => "dashdot2",
            LineStyle::DashDot3 => "dashdot3",
        }
    }
}
