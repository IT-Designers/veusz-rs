use crate::api1::cmd;
use crate::CommandLineEmbeddingInterface;
use std::io::Write;

pub mod marker;

#[derive(Copy, Clone, PartialOrd, PartialEq)]
pub enum ColorMap {
    Blank,
    Heat,
    Spectrum2,
    Spectrum2Step,
    Spectrum,
    Grey,
    Blue,
    Red,
    Green,
    BlueGreen,
    TransBlack,
    Royal,
    Complement,
    CoolWarm,
    GreenMagenta,
    BlueDarkred,
    BlueDarkorange,
    BrownBlue,
    BlueOrange,
    Seq,
    HotDesaturated,
    YellowGreen,
    YellowGreenBlue,
    YellowOrangeBrown,
    YellowOrangeRed,
    None,

    // https://github.com/veusz/veusz/blob/5a1e2af5f24df0eb2a2842be51f2997c4999c7fb/veusz/utils/colormap.py#L878
    CubeHelix,
    BlueGreenStep,
    ComplementStep,
    GreyStep5,
    GreyStep6,
    RoyalStep,
    SpectrumStep,
    TransBlackStep5,
    GreenMagentaStep16,
    BlueDarkredStep12,
    BlueDarkOrangeStep12,
    BrownBlueStep12,
    BlueOrangeStep12,
    SeqStep25,
}

impl ColorMap {
    pub(crate) fn as_veusz_value_str(self) -> &'static str {
        match self {
            ColorMap::Blank => "blank",
            ColorMap::Heat => "heat",
            ColorMap::Spectrum2 => "spectrum2",
            ColorMap::Spectrum2Step => "spectrum2-step",
            ColorMap::Spectrum => "spectrum",
            ColorMap::Grey => "grey",
            ColorMap::Blue => "blue",
            ColorMap::Red => "red",
            ColorMap::Green => "green",
            ColorMap::BlueGreen => "bluegreen",
            ColorMap::TransBlack => "transblack",
            ColorMap::Royal => "royal",
            ColorMap::Complement => "complement",
            ColorMap::CoolWarm => "cool-warm",
            ColorMap::GreenMagenta => "green-magenta",
            ColorMap::BlueDarkred => "blue-darkred",
            ColorMap::BlueDarkorange => "blue-darkorange",
            ColorMap::BrownBlue => "brown-blue",
            ColorMap::BlueOrange => "blue-orange",
            ColorMap::Seq => "seq",
            ColorMap::HotDesaturated => "hot_desaturated",
            ColorMap::YellowGreen => "yellow-green",
            ColorMap::YellowGreenBlue => "yellow-green-blue",
            ColorMap::YellowOrangeBrown => "yellow-green-brown",
            ColorMap::YellowOrangeRed => "yellow-orange-red",
            ColorMap::None => "none",

            ColorMap::CubeHelix => "cubehelix(0.5,-1.5,1,1)",
            ColorMap::BlueGreenStep => "bluegreen-step",
            ColorMap::ComplementStep => "complement-step",
            ColorMap::GreyStep5 => "grey-step5",
            ColorMap::GreyStep6 => "grey-step6",
            ColorMap::RoyalStep => "royal-step",
            ColorMap::SpectrumStep => "spectrum-step",
            ColorMap::TransBlackStep5 => "transblack-step5",
            ColorMap::GreenMagentaStep16 => "green-magenta-step16",
            ColorMap::BlueDarkredStep12 => "blue-darkred-step12",
            ColorMap::BlueDarkOrangeStep12 => "blue-darkorange-step12",
            ColorMap::BrownBlueStep12 => "brown-blue-step12",
            ColorMap::BlueOrangeStep12 => "blue-orange-step12",
            ColorMap::SeqStep25 => "seq-step25",
        }
    }
}

pub struct Color {
    points: String,
    min: Option<f32>,
    max: Option<f32>,
}

impl Color {
    pub fn points(points: impl Into<String>) -> Self {
        Self {
            points: points.into(),
            min: None,
            max: None,
        }
    }

    pub fn set_min(&mut self, min: f32) {
        self.min = Some(min);
    }

    pub fn with_min(mut self, min: f32) -> Self {
        self.set_min(min);
        self
    }

    pub fn set_max(&mut self, max: f32) {
        self.max = Some(max);
    }

    pub fn with_max(mut self, max: f32) -> Self {
        self.set_max(max);
        self
    }
}

impl CommandLineEmbeddingInterface for Color {
    fn write<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        cmd::Set("Color/points", &self.points).write(writer)?;

        if let Some(min) = &self.min {
            cmd::SetRaw("Color/min", min).write(writer)?;
        }

        if let Some(max) = &self.max {
            cmd::SetRaw("Color/max", max).write(writer)?;
        }

        Ok(())
    }
}
