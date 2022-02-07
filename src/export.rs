use crate::api1::AsVueszApi1ValueStr;
use crate::CommandLineEmbeddingInterface;
use std::io::Write;

#[derive(derive_more::From)]
pub enum Export {
    Svg(SvgExport),
}

impl CommandLineEmbeddingInterface for Export {
    fn write<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        match self {
            Export::Svg(svg) => svg.write(writer),
        }
    }
}

pub struct SvgExport {
    filename: String,
    color: Option<bool>,
    page: Option<usize>,
    text_as_text: Option<bool>,
}

impl SvgExport {
    pub fn target(filename: impl Into<String>) -> Self {
        let mut filename = filename.into();

        if !filename.ends_with(".svg") {
            filename.push_str(".svg");
        }

        SvgExport {
            filename,
            color: None,
            page: None,
            text_as_text: None,
        }
    }

    pub fn set_color(&mut self, color: bool) {
        self.color = Some(color);
    }

    pub fn with_color(mut self, color: bool) -> Self {
        self.set_color(color);
        self
    }

    pub fn set_page(&mut self, page: usize) {
        self.page = Some(page);
    }

    pub fn with_page(mut self, page: usize) -> Self {
        self.set_page(page);
        self
    }

    pub fn set_text_as_text(&mut self, text_as_text: bool) {
        self.text_as_text = Some(text_as_text);
    }

    pub fn with_text_as_text(mut self, text_as_text: bool) -> Self {
        self.set_text_as_text(text_as_text);
        self
    }
}

impl CommandLineEmbeddingInterface for SvgExport {
    fn write<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        let mut parameters = Vec::with_capacity(4);

        parameters.push(format!("'{}'", self.filename));

        if let Some(color) = self.color {
            parameters.push(format!("color={}", color.as_veusz_api1_value_str()));
        }

        if let Some(page) = self.page {
            parameters.push(format!("page={page}"));
        }

        if let Some(text_as_text) = self.text_as_text {
            parameters.push(format!(
                "svgtextastext={}",
                text_as_text.as_veusz_api1_value_str()
            ));
        }

        writeln!(writer, "Export({})", parameters.join(", "))
    }
}
