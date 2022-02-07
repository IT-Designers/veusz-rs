use crate::api1::cmd;
use crate::style::ColorMap;
use crate::CommandLineEmbeddingInterface;
use std::io::Write;

#[derive(Copy, Clone, PartialOrd, PartialEq)]
pub enum Marker {
    None,
    Circle,
}

#[derive(Default)]
pub struct MarkerLine {
    hide: Option<bool>,
}

impl MarkerLine {
    pub fn set_hide(&mut self, hide: bool) {
        self.hide = Some(hide);
    }

    pub fn with_hide(mut self, hide: bool) -> Self {
        self.set_hide(hide);
        self
    }
}

impl CommandLineEmbeddingInterface for MarkerLine {
    fn write<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        if let Some(hide) = &self.hide {
            cmd::SetRaw(
                "MarkerLine/hide",
                match hide {
                    true => "True",
                    false => "False",
                },
            )
            .write(writer)?;
        }

        Ok(())
    }
}

#[derive(Default)]
pub struct MarkerFill {
    hide: Option<bool>,
    color_map: Option<ColorMap>,
    color_map_invert: Option<bool>,
}

impl MarkerFill {
    pub fn set_hide(&mut self, hide: bool) {
        self.hide = Some(hide);
    }

    pub fn with_hide(mut self, hide: bool) -> Self {
        self.set_hide(hide);
        self
    }

    pub fn set_color_map(&mut self, color_map: ColorMap) {
        self.color_map = Some(color_map);
    }

    pub fn with_color_map(mut self, color_map: ColorMap) -> Self {
        self.set_color_map(color_map);
        self
    }

    pub fn set_color_map_invert(&mut self, invert: bool) {
        self.color_map_invert = Some(invert);
    }

    pub fn with_color_map_invert(mut self, invert: bool) -> Self {
        self.set_color_map_invert(invert);
        self
    }
}

impl CommandLineEmbeddingInterface for MarkerFill {
    fn write<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        if let Some(hide) = &self.hide {
            cmd::SetRaw(
                "MarkerFill/hide",
                match hide {
                    true => "True",
                    false => "False",
                },
            )
            .write(writer)?;
        }

        if let Some(color_map) = &self.color_map {
            cmd::Set("MarkerFill/colorMap", color_map.as_veusz_value_str()).write(writer)?;
        }

        if let Some(invert) = &self.color_map_invert {
            cmd::SetRaw(
                "MarkerFill/colorMapInvert",
                match invert {
                    true => "True",
                    false => "False",
                },
            )
            .write(writer)?;
        }

        Ok(())
    }
}
