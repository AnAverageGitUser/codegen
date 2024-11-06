use crate::docs::Docs;
use crate::r#type::Type;
use crate::Formatter;
use core::fmt;
use std::fmt::Write;

/// Defines a constant.
#[derive(Debug, Clone)]
pub struct Const {
    docs: Option<Docs>,
    vis: String,
    name: String,
    ty: Type,
    value: String,
}

impl Const {
    pub fn new<T>(ty: T) -> Self
    where
        T: Into<Type>,
    {
        Const {
            docs: None,
            vis: String::new(),
            name: String::new(),
            ty: ty.into(),
            value: String::new(),
        }
    }

    /// Set field's documentation.
    pub fn doc(&mut self, docs: Docs) -> &mut Self {
        self.docs = Some(docs);
        self
    }

    pub fn vis(&mut self, vis: impl ToString) -> &mut Self {
        self.vis = vis.to_string();
        self
    }

    pub fn ty(&mut self, ty: impl ToString) -> &mut Self {
        self.ty = Type::new(ty.to_string());
        self
    }

    pub fn name(&mut self, name: impl ToString) -> &mut Self {
        self.name = name.to_string();
        self
    }

    pub fn value(&mut self, value: impl ToString) -> &mut Self {
        self.value = value.to_string();
        self
    }

    /// Formats the function using the given formatter.
    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        if let Some(ref docs) = self.docs {
            docs.fmt(fmt)?;
        }

        if !self.vis.is_empty() {
            write!(fmt, "{} ", self.vis)?;
        }

        write!(fmt, "const {}: ", self.name)?;
        self.ty.fmt(fmt)?;
        writeln!(fmt, " = {};", self.value)
    }
}
