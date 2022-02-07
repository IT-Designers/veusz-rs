use std::fmt::{Debug, Display, Formatter};
use std::io::Result;
use std::io::Write;
use std::marker::PhantomData;
use std::sync::atomic::{AtomicUsize, Ordering};

pub(crate) mod cmd;

pub(crate) trait CommandLineEmbeddingInterface {
    fn write<W: Write>(&self, writer: &mut W) -> Result<()>;
}

pub struct ToParentDropGuard;

impl ToParentDropGuard {
    pub fn on<W: Write, F: FnOnce(&mut W) -> Result<()>>(writer: &mut W, f: F) -> Result<()> {
        let result = f(writer);
        Self.write(writer).and(result)
    }
}

impl CommandLineEmbeddingInterface for ToParentDropGuard {
    fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        writeln!(writer, "To('..')")
    }
}

#[derive(derive_more::Deref)]
pub(crate) struct AutoName<T>(PhantomData<T>, #[deref] String);

impl<T: CommandLineEmbeddingInterface> Default for AutoName<T> {
    fn default() -> Self {
        Self(Default::default(), AutoNameIncrement::<T>::next())
    }
}

impl<T: CommandLineEmbeddingInterface> Display for AutoName<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

pub(crate) struct AutoNameIncrement<T>(PhantomData<T>);

impl<T: CommandLineEmbeddingInterface> AutoNameIncrement<T> {
    pub fn next() -> String {
        static COUNTER: AtomicUsize = AtomicUsize::new(1);
        format!(
            "{}{}",
            std::any::type_name::<T>(),
            COUNTER.fetch_add(1, Ordering::Relaxed)
        )
    }
}
