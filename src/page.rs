use crate::api1::{cmd, AutoName};
use crate::size::SizeUnit;
use crate::style::marker::{Marker, MarkerFill, MarkerLine};
use crate::style::plot::PlotLine;
use crate::style::Color;
use crate::CommandLineEmbeddingInterface;
use std::borrow::Cow;
use std::io::Write;

#[derive(Default)]
pub struct Page {
    name: AutoName<Self>,
    items: Vec<PageItem>,
    width: Option<SizeUnit>,
    height: Option<SizeUnit>,
}

impl Page {
    pub fn add(&mut self, item: impl Into<PageItem>) {
        self.items.push(item.into());
    }

    pub fn with_item(mut self, item: impl Into<PageItem>) -> Self {
        self.add(item);
        self
    }

    pub fn with_items(mut self, items: impl IntoIterator<Item = impl Into<PageItem>>) -> Self {
        self.items.extend(items.into_iter().map(Into::into));
        self
    }

    pub fn with_width(mut self, width: impl Into<Option<SizeUnit>>) -> Self {
        self.width = width.into();
        self
    }

    pub fn with_height(mut self, height: impl Into<Option<SizeUnit>>) -> Self {
        self.height = height.into();
        self
    }
}

impl CommandLineEmbeddingInterface for Page {
    fn write<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        cmd::Add("page", &self.name).write(writer)?;
        cmd::ToUnique(&self.name).for_call(writer, |writer| {
            for item in &self.items {
                item.write(writer)?;
            }
            Ok(())
        })?;

        if let Some(width) = self.width {
            cmd::Set("width", &width.to_string()).write(writer)?;
        }

        if let Some(height) = self.height {
            cmd::Set("height", &height.to_string()).write(writer)?;
        }

        Ok(())
    }
}

#[derive(derive_more::From)]
pub enum PageItem {
    Graph(Graph),
    Grid(Grid),
    Label(Label),
}

impl CommandLineEmbeddingInterface for PageItem {
    fn write<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        match self {
            PageItem::Graph(graph) => graph.write(writer),
            PageItem::Grid(grid) => grid.write(writer),
            PageItem::Label(label) => label.write(writer),
        }
    }
}

#[derive(Default)]
pub struct Graph {
    name: AutoName<Self>,
    aspect: Option<AspectRatio>,
    axes: Vec<Axis>,
    xy_data: Vec<Xy>,
}

impl Graph {
    pub fn set_aspect(&mut self, aspect: impl Into<AspectRatio>) {
        self.aspect = Some(aspect.into());
    }

    pub fn with_aspect(mut self, aspect: impl Into<AspectRatio>) -> Self {
        self.set_aspect(aspect);
        self
    }

    pub fn add_axis(&mut self, axis: Axis) {
        self.axes.push(axis);
    }

    pub fn with_xy_axis(mut self, x: impl Into<String>, y: impl Into<String>) -> Self {
        self.add_axis(Axis::x(x));
        self.add_axis(Axis::y(y));
        self
    }

    pub fn with_axis(mut self, axis: Axis) -> Self {
        self.add_axis(axis);
        self
    }

    pub fn add_xy(&mut self, xy: Xy) {
        self.xy_data.push(xy);
    }

    pub fn with_xy(mut self, xy: Xy) -> Self {
        self.add_xy(xy);
        self
    }

    pub fn with_xy_sets(mut self, sets: impl IntoIterator<Item = Xy>) -> Self {
        self.xy_data.extend(sets);
        self
    }
}

impl CommandLineEmbeddingInterface for Graph {
    fn write<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        cmd::Add("graph", &self.name).write(writer)?;
        cmd::ToUnique(&self.name).for_call(writer, |writer| {
            if let Some(aspect) = self.aspect {
                match aspect {
                    AspectRatio::Auto => cmd::Set("aspect", "Auto").write(writer)?,
                    AspectRatio::Fix(value) => cmd::SetRaw("aspect", value).write(writer)?,
                }
            }
            for axis in &self.axes {
                axis.write(writer)?;
            }
            for xy in &self.xy_data {
                xy.write(writer)?;
            }
            Ok(())
        })
    }
}
#[derive(derive_more::From, Copy, Clone, PartialEq)]
pub enum AspectRatio {
    Auto,
    Fix(f64),
}

#[derive(Default)]
pub struct Grid {
    name: AutoName<Self>,
    rows: Option<u32>,
    columns: Option<u32>,
    items: Vec<PageItem>,
}

impl Grid {
    pub fn set_rows(&mut self, rows: u32) {
        self.rows = Some(rows);
    }

    pub fn with_rows(mut self, rows: u32) -> Self {
        self.set_rows(rows);
        self
    }

    pub fn set_columns(&mut self, columns: u32) {
        self.columns = Some(columns);
    }

    pub fn with_columns(mut self, columns: u32) -> Self {
        self.set_columns(columns);
        self
    }

    pub fn add(&mut self, item: impl Into<PageItem>) {
        self.items.push(item.into());
    }

    pub fn with(mut self, item: impl Into<PageItem>) -> Self {
        self.add(item);
        self
    }

    pub fn with_items(mut self, items: impl IntoIterator<Item = impl Into<PageItem>>) -> Self {
        self.items.extend(items.into_iter().map(Into::into));
        self
    }
}

impl CommandLineEmbeddingInterface for Grid {
    fn write<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        cmd::Add("grid", &self.name).write(writer)?;
        cmd::ToUnique(&self.name).for_call(writer, |writer| {
            if let Some(rows) = self.rows {
                cmd::SetRaw("rows", rows).write(writer)?;
            }
            if let Some(columns) = self.columns {
                cmd::SetRaw("columns", columns).write(writer)?;
            }
            for item in &self.items {
                item.write(writer)?;
            }
            Ok(())
        })
    }
}

#[derive(Copy, Clone, PartialOrd, PartialEq)]
pub enum AxisDirection {
    Vertical,
    Horizontal,
}

pub struct Axis {
    name: String,
    label: String,
    direction: Option<AxisDirection>,
    min: Option<f64>,
    max: Option<f64>,
}

impl Axis {
    pub fn x(label: impl Into<String>) -> Self {
        Self {
            name: "x".into(),
            label: label.into(),
            direction: None,
            min: None,
            max: None,
        }
    }

    pub fn y(label: impl Into<String>) -> Self {
        Self {
            name: "y".into(),
            label: label.into(),
            direction: Some(AxisDirection::Vertical),
            min: None,
            max: None,
        }
    }

    pub fn with_min(mut self, min: impl Into<Option<f64>>) -> Self {
        self.min = min.into();
        self
    }

    pub fn with_max(mut self, max: impl Into<Option<f64>>) -> Self {
        self.max = max.into();
        self
    }
}

impl CommandLineEmbeddingInterface for Axis {
    fn write<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        cmd::Add("axis", &self.name).write(writer)?;
        cmd::ToUnique(&self.name).for_call(writer, |writer| {
            cmd::Set("label", &self.label).write(writer)?;

            if let Some(direction) = &self.direction {
                cmd::Set(
                    "direction",
                    match direction {
                        AxisDirection::Vertical => "vertical",
                        AxisDirection::Horizontal => "horizontal",
                    },
                )
                .write(writer)?;
            }

            if let Some(min) = self.min {
                cmd::SetRaw("min", min).write(writer)?;
            }

            if let Some(max) = self.max {
                cmd::SetRaw("max", max).write(writer)?;
            }

            Ok(())
        })
    }
}

pub struct Xy {
    name: AutoName<Self>,
    color: Option<Color>,
    marker: Option<Marker>,
    marker_line: Option<MarkerLine>,
    marker_fill: Option<MarkerFill>,
    plot_line: Option<PlotLine>,
    x_data: String,
    y_data: String,
}

impl Xy {
    pub fn data(x_data: impl Into<String>, y_data: impl Into<String>) -> Self {
        Self {
            name: AutoName::default(),
            color: None,
            marker: None,
            marker_line: None,
            marker_fill: None,
            plot_line: None,
            x_data: x_data.into(),
            y_data: y_data.into(),
        }
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = Some(color);
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.set_color(color);
        self
    }

    pub fn set_marker(&mut self, marker: Marker) {
        self.marker = Some(marker);
    }

    pub fn with_marker(mut self, marker: Marker) -> Self {
        self.set_marker(marker);
        self
    }

    pub fn set_marker_line(&mut self, marker_line: MarkerLine) {
        self.marker_line = Some(marker_line);
    }

    pub fn with_marker_line(mut self, marker_line: MarkerLine) -> Self {
        self.set_marker_line(marker_line);
        self
    }

    pub fn set_marker_fill(&mut self, marker_fill: MarkerFill) {
        self.marker_fill = Some(marker_fill);
    }

    pub fn with_marker_fill(mut self, marker_fill: MarkerFill) -> Self {
        self.set_marker_fill(marker_fill);
        self
    }

    pub fn set_plot_line(&mut self, plot_line: PlotLine) {
        self.plot_line = Some(plot_line);
    }

    pub fn with_plot_line(mut self, plot_line: PlotLine) -> Self {
        self.set_plot_line(plot_line);
        self
    }
}

impl CommandLineEmbeddingInterface for Xy {
    fn write<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        cmd::Add("xy", &self.name).write(writer)?;
        cmd::ToUnique(&self.name).for_call(writer, |writer| {
            if let Some(marker) = &self.marker {
                cmd::Set(
                    "marker",
                    match marker {
                        Marker::None => "none",
                        Marker::Circle => "circle",
                    },
                )
                .write(writer)?;
            }

            if let Some(color) = &self.color {
                color.write(writer)?;
            }

            if let Some(marker_line) = &self.marker_line {
                marker_line.write(writer)?;
            }

            if let Some(marker_fill) = &self.marker_fill {
                marker_fill.write(writer)?;
            }

            if let Some(plot_line) = &self.plot_line {
                plot_line.write(writer)?;
            }

            cmd::Set("xData", &self.x_data).write(writer)?;
            cmd::Set("yData", &self.y_data).write(writer)?;

            Ok(())
        })
    }
}

pub struct Label {
    name: AutoName<Self>,
    text: Cow<'static, str>,
    x_positions: Vec<f64>,
    y_positions: Vec<f64>,
    align_horizontal: Option<Alignment>,
    align_vertical: Option<Alignment>,
    positioning: Option<Positioning>,
    text_config: Option<TextConfig>,
    // config: LabelConfig, // TODO
}

impl Label {
    pub fn set_x_positions(&mut self, positions: impl Into<Vec<f64>>) {
        self.x_positions = positions.into();
    }

    pub fn with_x_positions(mut self, positions: impl Into<Vec<f64>>) -> Self {
        self.set_x_positions(positions);
        self
    }

    pub fn set_y_positions(&mut self, positions: impl Into<Vec<f64>>) {
        self.y_positions = positions.into();
    }

    pub fn with_y_positions(mut self, positions: impl Into<Vec<f64>>) -> Self {
        self.set_y_positions(positions);
        self
    }

    pub fn set_align_horizontal(&mut self, alignment: Alignment) {
        self.align_horizontal = Some(alignment);
    }

    pub fn with_alignment_horizontal(mut self, alignemnt: Alignment) -> Self {
        self.set_align_horizontal(alignemnt);
        self
    }

    pub fn set_align_vertical(&mut self, alignment: Alignment) {
        self.align_vertical = Some(alignment);
    }

    pub fn with_alignment_vertical(mut self, alignment: Alignment) -> Self {
        self.set_align_vertical(alignment);
        self
    }

    pub fn set_text_config(&mut self, text_config: impl Into<TextConfig>) {
        self.text_config = Some(text_config.into());
    }

    pub fn with_text_config(mut self, text_config: impl Into<TextConfig>) -> Self {
        self.set_text_config(text_config);
        self
    }

    pub fn set_positioning(&mut self, positioning: impl Into<Positioning>) {
        self.positioning = Some(positioning.into());
    }

    pub fn with_positioning(mut self, positioning: impl Into<Positioning>) -> Self {
        self.set_positioning(positioning);
        self
    }
}

impl<I: Into<Cow<'static, str>>> From<I> for Label {
    fn from(value: I) -> Self {
        Self {
            name: Default::default(),
            text: value.into(),
            x_positions: Vec::default(),
            y_positions: Vec::default(),
            align_horizontal: None,
            align_vertical: None,
            positioning: None,
            text_config: None,
        }
    }
}

impl CommandLineEmbeddingInterface for Label {
    fn write<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        cmd::Add("label", &self.name).write(writer)?;
        cmd::ToUnique(&self.name).for_call(writer, |writer| {
            cmd::Set("label", &self.text).write(writer)?;
            if !self.x_positions.is_empty() {
                cmd::SetData(
                    "xPos",
                    &self
                        .x_positions
                        .iter()
                        .map(ToString::to_string)
                        .collect::<Vec<_>>()
                        .join(","),
                )
                .write(writer)?;
            }
            if !self.y_positions.is_empty() {
                cmd::SetData(
                    "yPos",
                    &self
                        .y_positions
                        .iter()
                        .map(ToString::to_string)
                        .collect::<Vec<_>>()
                        .join(","),
                )
                .write(writer)?;
            }
            if let Some(alignment) = &self.align_horizontal {
                cmd::Set("alignHorz", alignment.as_str()).write(writer)?;
            }
            if let Some(alignment) = &self.align_vertical {
                cmd::Set("alignVert", alignment.as_str()).write(writer)?;
            }
            if let Some(positioning) = &self.positioning {
                cmd::Set("positioning", positioning.as_str()).write(writer)?;
            }
            if let Some(text_config) = &self.text_config {
                if let Some(size) = &text_config.size {
                    cmd::Set("Text/size", &size.to_string()).write(writer)?;
                }
            }
            Ok(())
        })
    }
}

#[derive(Default)]
pub struct TextConfig {
    size: Option<TextSize>,
}

impl<T: Into<TextSize>> From<T> for TextConfig {
    fn from(value: T) -> Self {
        Self {
            size: Some(value.into()),
        }
    }
}

pub enum TextSize {
    Pt(f64),
}

impl ToString for TextSize {
    fn to_string(&self) -> String {
        match self {
            TextSize::Pt(pt) => format!("{pt}pt"),
        }
    }
}

pub enum Alignment {
    Top,
    Bottom,
    Left,
    Right,
    Center,
}

impl Alignment {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Alignment::Top => "top",
            Alignment::Bottom => "bottom",
            Alignment::Left => "left",
            Alignment::Right => "right",
            Alignment::Center => "centre",
        }
    }
}

pub enum Positioning {
    Relative,
    Axes,
}

impl Positioning {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Positioning::Relative => "relative",
            Positioning::Axes => "axes",
        }
    }
}
