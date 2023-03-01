use crate::api1::{cmd, AsVueszApi1ValueStr};
use crate::style::line::LineStyle;
use crate::style::{ColorMap, ColorName};
use crate::CommandLineEmbeddingInterface;
use std::io::Write;

#[derive(Copy, Clone, PartialOrd, PartialEq)]
pub enum Marker {
    None,
    Circle,
}

#[derive(Default)]
pub struct MarkerLine {
    color: Option<String>,
    width: Option<f32>,
    style: Option<LineStyle>,
    transparency: Option<u8>,
    scale: Option<bool>,
    hide: Option<bool>,
}

impl MarkerLine {
    pub fn set_color(&mut self, color: impl Into<String>) {
        self.color = Some(color.into());
    }

    pub fn with_color(mut self, color: impl Into<String>) -> Self {
        self.set_color(color);
        self
    }

    pub fn set_color_by_name(&mut self, color_name: ColorName) {
        self.color = Some(color_name.as_veusz_api1_value_str().to_string());
    }

    pub fn with_color_by_name(mut self, color_name: ColorName) -> Self {
        self.set_color_by_name(color_name);
        self
    }

    pub fn set_width(&mut self, pt: f32) {
        self.width = Some(pt);
    }

    pub fn with_width(mut self, pt: f32) -> Self {
        self.set_width(pt);
        self
    }

    pub fn set_style(&mut self, style: LineStyle) {
        self.style = Some(style);
    }

    pub fn with_style(mut self, style: LineStyle) -> Self {
        self.set_style(style);
        self
    }

    pub fn set_transparency(&mut self, transparency: u8) {
        self.transparency = Some(transparency);
    }

    pub fn with_transparency(mut self, transparency: u8) -> Self {
        self.set_transparency(transparency);
        self
    }

    pub fn set_scale(&mut self, scale: bool) {
        self.scale = Some(scale);
    }

    pub fn with_scale(mut self, scale: bool) -> Self {
        self.set_scale(scale);
        self
    }

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
        if let Some(color) = &self.color {
            cmd::Set("MarkerLine/color", color).write(writer)?;
        }

        if let Some(width) = &self.width {
            cmd::Set("MarkerLine/width", &format!("{width}pt")).write(writer)?;
        }

        if let Some(style) = &self.style {
            cmd::Set("MarkerLine/style", style.as_veusz_api1_value_str()).write(writer)?;
        }

        if let Some(transparency) = &self.transparency {
            cmd::SetRaw("MarkerLine/transparency", transparency).write(writer)?;
        }

        if let Some(scale) = &self.scale {
            cmd::Set("MarkerLine/scaleLine", scale.as_veusz_api1_value_str()).write(writer)?;
        }

        if let Some(hide) = &self.hide {
            cmd::SetRaw("MarkerLine/hide", hide.as_veusz_api1_value_str()).write(writer)?;
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
            cmd::SetRaw("MarkerFill/hide", hide.as_veusz_api1_value_str()).write(writer)?;
        }

        if let Some(color_map) = &self.color_map {
            cmd::Set("MarkerFill/colorMap", color_map.as_veusz_api1_value_str()).write(writer)?;
        }

        if let Some(invert) = &self.color_map_invert {
            cmd::SetRaw(
                "MarkerFill/colorMapInvert",
                invert.as_veusz_api1_value_str(),
            )
            .write(writer)?;
        }

        Ok(())
    }
}
