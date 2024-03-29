use crate::api1::{cmd, CommandLineEmbeddingInterface};
use crate::data::Data;
use crate::export::Export;
use crate::page::Page;
use std::borrow::BorrowMut;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::process::ExitStatus;

pub mod api1;
pub mod data;
pub mod export;
pub mod page;
pub mod size;
pub mod style;

#[derive(Default)]
pub struct Veusz {
    data: Vec<Data>,
    pages: Vec<Page>,
    exports: Vec<Export>,
}

impl Veusz {
    pub fn add_page(&mut self, page: Page) {
        self.pages.push(page);
    }

    pub fn with_page(mut self, page: Page) -> Self {
        self.add_page(page);
        self
    }

    pub fn with_pages(mut self, pages: impl IntoIterator<Item = Page>) -> Self {
        self.pages.extend(pages);
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

    pub fn with_export(mut self, export: impl Into<Export>) -> Self {
        self.exports.push(export.into());
        self
    }

    pub fn with_exports(mut self, exports: impl Iterator<Item = impl Into<Export>>) -> Self {
        for export in exports {
            self.exports.push(export.into());
        }
        self
    }

    /// Please consider [`BufWriter`] for optimal performance.
    pub fn save_configuration<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        self.write(writer)
    }

    /// Please consider [`BufWriter`] for optimal performance.
    pub fn with_saved_configuration<W: Write>(self, writer: &mut W) -> std::io::Result<Self> {
        self.save_configuration(writer)?;
        Ok(self)
    }

    /// Warning: might not work properly or with very poor performance
    pub fn open(self) {
        let mut proc = std::process::Command::new("veusz")
            .arg("--listen")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .spawn()
            .unwrap();

        self.write(BufWriter::new(std::io::stdout()).borrow_mut())
            .unwrap();
        self.write(BufWriter::new(proc.stdin.as_mut().unwrap()).borrow_mut())
            .unwrap();

        proc.wait().unwrap();
    }

    pub fn open_saved_configuration<P: AsRef<Path>>(
        self,
        path: P,
        options: &OpenOptions,
    ) -> impl FnMut() -> std::io::Result<ExitStatus> {
        self.write(BufWriter::new(std::io::stdout()).borrow_mut())
            .unwrap();
        self.write(BufWriter::new(options.open(path.as_ref()).unwrap()).borrow_mut())
            .unwrap();

        let mut proc = std::process::Command::new("veusz")
            .arg("--listen")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .spawn()
            .unwrap();

        cmd::Load(path.as_ref().as_os_str().to_string_lossy().as_ref())
            .write(BufWriter::new(proc.stdin.as_mut().unwrap()).borrow_mut())
            .unwrap();

        move || proc.wait()
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

        for export in &self.exports {
            export.write(writer)?;
        }

        Ok(())
    }
}
