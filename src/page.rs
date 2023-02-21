use crate::api1::{cmd, AutoName};
use crate::style::marker::{Marker, MarkerFill, MarkerLine};
use crate::style::plot::PlotLine;
use crate::style::Color;
use crate::CommandLineEmbeddingInterface;
use std::io::Write;

#[derive(Default)]
pub struct Page {
    name: AutoName<Self>,
    items: Vec<PageItem>,
}

impl Page {
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

impl CommandLineEmbeddingInterface for Page {
    fn write<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        cmd::Add("page", &self.name).write(writer)?;
        cmd::ToUnique(&self.name).for_call(writer, |writer| {
            for item in &self.items {
                item.write(writer)?;
            }
            Ok(())
        })
    }
}

#[derive(derive_more::From)]
pub enum PageItem {
    Graph(Graph),
    Grid(Grid),
}

impl CommandLineEmbeddingInterface for PageItem {
    fn write<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        match self {
            PageItem::Graph(graph) => graph.write(writer),
            PageItem::Grid(grid) => grid.write(writer),
        }
    }
}

#[derive(Default)]
pub struct Graph {
    name: AutoName<Self>,
    axes: Vec<Axis>,
    xy_data: Vec<Xy>,
}

impl Graph {
    pub fn add_axis(&mut self, axis: Axis) {
        self.axes.push(axis);
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
}

impl Axis {
    pub fn x(label: impl Into<String>) -> Self {
        Self {
            name: "x".into(),
            label: label.into(),
            direction: None,
        }
    }

    pub fn y(label: impl Into<String>) -> Self {
        Self {
            name: "y".into(),
            label: label.into(),
            direction: Some(AxisDirection::Vertical),
        }
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
