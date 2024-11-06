/// Defines an import (`use` statement).
#[derive(Debug, Clone)]
pub struct Import {
    /// Function visibility
    pub vis: Option<String>,
}

impl Default for Import {
    fn default() -> Self {
        Self::new()
    }
}

impl Import {
    /// Return a new import.
    pub fn new() -> Self {
        Import {
            vis: None,
        }
    }

    /// Set the import visibility.
    pub fn vis(&mut self, vis: impl ToString) -> &mut Self {
        self.vis = Some(vis.to_string());
        self
    }
}
