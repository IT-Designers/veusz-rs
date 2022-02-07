use crate::api1::ToParentDropGuard;
use std::io::Write;

#[must_use]
pub struct Add<'a, 'b>(pub &'a str, pub &'b str);

impl Add<'_, '_> {
    pub fn write<W: Write>(self, writer: &mut W) -> std::io::Result<()> {
        writeln!(
            writer,
            "Add('{}', name=u'{}', autoadd=False)",
            self.0, self.1
        )
    }
}

#[must_use]
pub struct ToUnique<'a>(pub &'a str);

impl ToUnique<'_> {
    pub fn for_call<W: Write, F: FnOnce(&mut W) -> std::io::Result<()>>(
        self,
        writer: &mut W,
        f: F,
    ) -> std::io::Result<()> {
        writeln!(writer, "To(u'{}')", self.0)?;
        ToParentDropGuard::on(writer, f)
    }
}

#[must_use]
pub struct Set<'a, 'b>(pub &'a str, pub &'b str);

impl Set<'_, '_> {
    pub fn write<W: Write>(self, writer: &mut W) -> std::io::Result<()> {
        writeln!(writer, "Set('{}', u'{}')", self.0, self.1)
    }
}

#[must_use]
pub struct SetData<'a, 'b>(pub &'a str, pub &'b str);

impl SetData<'_, '_> {
    pub fn write<W: Write>(self, writer: &mut W) -> std::io::Result<()> {
        writeln!(writer, "SetData(u'{}', [{}])", self.0, self.1)
    }
}
