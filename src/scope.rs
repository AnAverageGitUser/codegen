use std::fmt::{self, Debug, Display, Write};

use indexmap::IndexMap;

use crate::docs::Docs;
use crate::formatter::Formatter;
use crate::function::Function;
use crate::import::Import;
use crate::item::Item;
use crate::module::Module;
use crate::r#const::Const;
use crate::r#enum::Enum;
use crate::r#impl::Impl;
use crate::r#struct::Struct;
use crate::r#trait::Trait;
use crate::type_alias::TypeAlias;

/// Defines a scope.
///
/// A scope contains modules, types, etc...
#[derive(Debug, Clone)]
pub struct Scope {
    /// Scope documentation
    docs: Option<Docs>,

    /// Imports
    imports: IndexMap<String, IndexMap<String, Import>>,

    /// Contents of the documentation,
    items: Vec<Item>,
}

impl Default for Scope {
    fn default() -> Self {
        Self::new()
    }
}

impl Scope {
    /// Returns a new scope
    pub fn new() -> Self {
        Scope {
            docs: None,
            imports: IndexMap::new(),
            items: vec![],
        }
    }

    /// Set the scope documentation.
    pub fn doc(&mut self, docs: impl ToString) -> &mut Self {
        self.docs = Some(Docs::new(docs));
        self
    }

    /// Import a type into the scope.
    ///
    /// This results in a new `use` statement being added to the beginning of
    /// the scope.
    pub fn import(&mut self, path: impl ToString, ty: impl ToString) -> &mut Import {
        // handle cases where the caller wants to refer to a type namespaced
        // within the containing namespace, like "a::B".
        let ty = ty.to_string();
        let ty = ty.split("::").next().unwrap_or(ty.as_str());
        self.imports
            .entry(path.to_string())
            .or_default()
            .entry(ty.to_string())
            .or_default()
    }

    /// Push a new module definition, returning a mutable reference to it.
    ///
    /// # Panics
    ///
    /// Since a module's name must uniquely identify it within the scope in
    /// which it is defined, pushing a module whose name is already defined
    /// in this scope will cause this function to panic.
    ///
    /// In many cases, the [`get_or_new_module`] function is preferrable, as it
    /// will return the existing definition instead.
    ///
    /// [`get_or_new_module`]: #method.get_or_new_module
    pub fn new_module(&mut self, name: impl ToString) -> &mut Module {
        self.push_module(Module::new(name));

        match *self.items.last_mut().unwrap() {
            Item::Module(ref mut v) => v,
            _ => unreachable!(),
        }
    }

    /// Returns a mutable reference to a module if it is exists in this scope.
    pub fn get_module_mut<Q: ?Sized>(&mut self, name: &Q) -> Option<&mut Module>
    where
        String: PartialEq<Q>,
    {
        self.items
            .iter_mut()
            .filter_map(|item| match item {
                &mut Item::Module(ref mut module) if module.name == *name => Some(module),
                _ => None,
            })
            .next()
    }

    /// Returns a mutable reference to a module if it is exists in this scope.
    pub fn get_module<Q: ?Sized>(&self, name: &Q) -> Option<&Module>
    where
        String: PartialEq<Q>,
    {
        self.items
            .iter()
            .filter_map(|item| match item {
                Item::Module(module) if module.name == *name => Some(module),
                _ => None,
            })
            .next()
    }

    /// Returns a mutable reference to a module, creating it if it does
    /// not exist.
    pub fn get_or_new_module<Q: ?Sized + Display>(&mut self, name: &Q) -> &mut Module
    where
        String: PartialEq<Q>,
    {
        if self.get_module(name).is_some() {
            self.get_module_mut(name).unwrap()
        } else {
            self.new_module(name)
        }
    }

    /// Push a module definition.
    ///
    /// # Panics
    ///
    /// Since a module's name must uniquely identify it within the scope in
    /// which it is defined, pushing a module whose name is already defined
    /// in this scope will cause this function to panic.
    ///
    /// In many cases, the [`get_or_new_module`] function is preferrable, as it will
    /// return the existing definition instead.
    ///
    /// [`get_or_new_module`]: #method.get_or_new_module
    pub fn push_module(&mut self, item: Module) -> &mut Self {
        assert!(self.get_module(&item.name).is_none());
        self.items.push(Item::Module(item));
        self
    }

    /// Push a new struct definition, returning a mutable reference to it.
    pub fn new_struct(&mut self, name: impl ToString) -> &mut Struct {
        self.push_struct(Struct::new(name));

        match *self.items.last_mut().unwrap() {
            Item::Struct(ref mut v) => v,
            _ => unreachable!(),
        }
    }

    /// Push a struct definition
    pub fn push_struct(&mut self, item: Struct) -> &mut Self {
        self.items.push(Item::Struct(item));
        self
    }

    /// Push a new function definition, returning a mutable reference to it.
    pub fn new_fn(&mut self, name: impl ToString) -> &mut Function {
        self.push_fn(Function::new(name));

        match *self.items.last_mut().unwrap() {
            Item::Function(ref mut v) => v,
            _ => unreachable!(),
        }
    }

    /// Push a function definition
    pub fn push_fn(&mut self, item: Function) -> &mut Self {
        self.items.push(Item::Function(item));
        self
    }

    /// Push a new trait definition, returning a mutable reference to it.
    pub fn new_trait(&mut self, name: impl ToString) -> &mut Trait {
        self.push_trait(Trait::new(name));

        match *self.items.last_mut().unwrap() {
            Item::Trait(ref mut v) => v,
            _ => unreachable!(),
        }
    }

    /// Push a trait definition
    pub fn push_trait(&mut self, item: Trait) -> &mut Self {
        self.items.push(Item::Trait(item));
        self
    }

    /// Push a new struct definition, returning a mutable reference to it.
    pub fn new_enum(&mut self, name: impl ToString) -> &mut Enum {
        self.push_enum(Enum::new(name));

        match *self.items.last_mut().unwrap() {
            Item::Enum(ref mut v) => v,
            _ => unreachable!(),
        }
    }

    /// Push a structure definition
    pub fn push_enum(&mut self, item: Enum) -> &mut Self {
        self.items.push(Item::Enum(item));
        self
    }

    /// Push a new `impl` block, returning a mutable reference to it.
    pub fn new_impl(&mut self, target: impl ToString) -> &mut Impl {
        self.push_impl(Impl::new(target));

        match *self.items.last_mut().unwrap() {
            Item::Impl(ref mut v) => v,
            _ => unreachable!(),
        }
    }

    /// Push an `impl` block.
    pub fn push_impl(&mut self, item: Impl) -> &mut Self {
        self.items.push(Item::Impl(item));
        self
    }

    /// Push a new const, returning a mutable reference to it.
    pub fn new_const(&mut self, target: impl ToString) -> &mut Const {
        self.push_const(Const::new(target));

        match *self.items.last_mut().unwrap() {
            Item::Const(ref mut v) => v,
            _ => unreachable!(),
        }
    }

    /// Push a const.
    pub fn push_const(&mut self, item: Const) -> &mut Self {
        self.items.push(Item::Const(item));
        self
    }

    /// Push a raw string to the scope.
    ///
    /// This string will be included verbatim in the formatted string.
    pub fn raw(&mut self, val: impl ToString) -> &mut Self {
        self.items.push(Item::Raw(val.to_string()));
        self
    }

    /// Push a new `TypeAlias`, returning a mutable reference to it.
    pub fn new_type_alias(&mut self, name: impl ToString, target: impl ToString) -> &mut TypeAlias {
        self.push_type_alias(TypeAlias::new(name, target));

        match *self.items.last_mut().unwrap() {
            Item::TypeAlias(ref mut v) => v,
            _ => unreachable!(),
        }
    }

    /// Push an `TypeAlias`.
    pub fn push_type_alias(&mut self, item: TypeAlias) -> &mut Self {
        self.items.push(Item::TypeAlias(item));
        self
    }

    /// Return a string representation of the scope.
    pub fn to_string(&self) -> String {
        let mut ret = String::new();

        self.fmt(&mut Formatter::new(&mut ret)).unwrap();

        // Remove the trailing newline
        if ret.as_bytes().last() == Some(&b'\n') {
            ret.pop();
        }

        ret
    }

    /// Formats the scope using the given formatter.
    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        if let Some(ref docs) = self.docs {
            docs.fmt(fmt)?;
        }

        self.fmt_imports(fmt)?;

        if !self.imports.is_empty() {
            writeln!(fmt)?;
        }

        for (i, item) in self.items.iter().enumerate() {
            if i != 0 {
                writeln!(fmt)?;
            }

            match *item {
                Item::Module(ref v) => v.fmt(fmt)?,
                Item::Struct(ref v) => v.fmt(fmt)?,
                Item::Function(ref v) => v.fmt(false, fmt)?,
                Item::Trait(ref v) => v.fmt(fmt)?,
                Item::Enum(ref v) => v.fmt(fmt)?,
                Item::Impl(ref v) => v.fmt(fmt)?,
                Item::Raw(ref v) => {
                    writeln!(fmt, "{}", v)?;
                }
                Item::TypeAlias(ref v) => v.fmt(fmt)?,
                Item::Const(ref v) => v.fmt(fmt)?,
            }
        }

        Ok(())
    }

    fn fmt_imports(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        // First, collect all visibilities
        let mut visibilities = vec![];

        for (_, imports) in &self.imports {
            for (_, import) in imports {
                if !visibilities.contains(&import.vis) {
                    visibilities.push(import.vis.clone());
                }
            }
        }

        let mut tys = vec![];

        // Loop over all visibilities and format the associated imports
        for vis in &visibilities {
            for (path, imports) in &self.imports {
                tys.clear();

                for (ty, import) in imports {
                    if *vis == import.vis {
                        tys.push(ty);
                    }
                }

                if !tys.is_empty() {
                    if let Some(ref vis) = *vis {
                        write!(fmt, "{} ", vis)?;
                    }

                    write!(fmt, "use {}::", path)?;

                    if tys.len() > 1 {
                        write!(fmt, "{{")?;

                        for (i, ty) in tys.iter().enumerate() {
                            if i != 0 {
                                write!(fmt, ", ")?;
                            }
                            write!(fmt, "{}", ty)?;
                        }

                        writeln!(fmt, "}};")?;
                    } else if tys.len() == 1 {
                        writeln!(fmt, "{};", tys[0])?;
                    }
                }
            }
        }

        Ok(())
    }
}
