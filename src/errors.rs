use std::fmt::Display;

/// all possible things that the user could do wrong
pub enum UserError {
    NoRunnableFound,
}

impl UserError {
    pub fn message(&self) -> &'static str {
        match self {
            UserError::NoRunnableFound => "No runnable found",
        }
    }
}

impl Display for UserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.message())
    }
}
