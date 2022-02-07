use crate::api1::CommandLineEmbeddingInterface;
use crate::data::Data;
use crate::page::Page;
use std::borrow::BorrowMut;
use std::io::Write;

pub mod api1;
pub mod data;
pub mod page;

#[derive(Default)]
pub struct Veusz {
    pages: Vec<Page>,
    data: Vec<Data>,
}

impl Veusz {
    pub fn add_page(&mut self, page: Page) {
        self.pages.push(page);
    }

    pub fn with_page(mut self, page: Page) -> Self {
        self.add_page(page);
        self
    }

    pub fn add_data(&mut self, data: Data) {
        self.data.push(data);
    }

    pub fn with_data(mut self, data: Data) -> Self {
        self.add_data(data);
        self
    }

    pub fn with_data_sets(mut self, datasets: impl IntoIterator<Item = Data>) -> Self {
        self.data.extend(datasets);
        self
    }

    pub fn open(self) {
        let mut proc = std::process::Command::new("veusz")
            .arg("--listen")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .spawn()
            .unwrap();

        self.write(std::io::stdout().borrow_mut()).unwrap();
        self.write(proc.stdin.as_mut().unwrap()).unwrap();

        proc.wait().unwrap();
    }
}

impl CommandLineEmbeddingInterface for Veusz {
    fn write<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        // cmd::Set("colorTheme", "default-latest").write(writer)?;
        // cmd::Set("StyleSheet/axis-function/autoRange", "next-tick").write(writer)?;

        for data in &self.data {
            data.write(writer)?;
        }
        for page in &self.pages {
            page.write(writer)?;
        }
        Ok(())
    }
}
