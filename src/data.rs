use crate::api1::cmd;
use crate::CommandLineEmbeddingInterface;
use std::fmt::Display;
use std::io::Write;

pub struct Data {
    name: String,
    value: String,
}

impl Data {
    pub fn new<D: Display>(name: impl Into<String>, data: impl Iterator<Item = D>) -> Self {
        Data {
            name: name.into(),
            value: data
                .map(|d| format!("{}", d))
                .collect::<Vec<String>>()
                .join(", "),
        }
    }
}

impl CommandLineEmbeddingInterface for Data {
    fn write<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        cmd::SetData(&self.name, &self.value).write(writer)
    }
}
