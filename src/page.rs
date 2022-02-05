use crate::api1::{cmd, AutoName, AutoNameIncrement, ToParentDropGuard};
use crate::CommandLineEmbeddingInterface;
use std::fmt::format;
use std::io::Write;
use std::ptr::write;
use std::sync::atomic::{AtomicUsize, Ordering};

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
}

impl CommandLineEmbeddingInterface for Page {
    fn write<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        cmd::Add("page", &self.name).write(writer);
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
}

impl CommandLineEmbeddingInterface for PageItem {
    fn write<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        match self {
            PageItem::Graph(graph) => graph.write(writer),
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

pub struct Axis {
    name: String,
    label: String,
}

impl Axis {
    pub fn x(label: impl Into<String>) -> Self {
        Self {
            name: "x".into(),
            label: label.into(),
        }
    }

    pub fn y(label: impl Into<String>) -> Self {
        Self {
            name: "y".into(),
            label: label.into(),
        }
    }
}

impl CommandLineEmbeddingInterface for Axis {
    fn write<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        cmd::Add("axis", &self.name).write(writer)?;
        cmd::ToUnique(&self.name).for_call(writer, |writer| {
            cmd::Set("label", &self.label).write(writer)
        })
    }
}

pub struct Xy {
    name: AutoName<Self>,
    x_data: String,
    y_data: String,
}

impl Xy {
    pub fn data(x_data: impl Into<String>, y_data: impl Into<String>) -> Self {
        Self {
            name: AutoName::default(),
            x_data: x_data.into(),
            y_data: y_data.into(),
        }
    }
}

impl CommandLineEmbeddingInterface for Xy {
    fn write<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        cmd::Add("xy", &self.name).write(writer)?;
        cmd::ToUnique(&self.name).for_call(writer, |writer| {
            cmd::Set("xData", &self.x_data).write(writer)?;
            cmd::Set("yData", &self.y_data).write(writer)
        })
    }
}
