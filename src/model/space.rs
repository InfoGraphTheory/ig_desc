use serde::{Serialize, Deserialize};

/// Used to hold information about the space such as the space id and project folder.
///
/// Wraps either a bare `String` or an `Option<String>` so it can be constructed from either
/// (via the `From` impls below); `get_value` normalizes both forms to `Option<String>`,
/// treating an empty string the same as `None`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Space {
    /// A space id given directly.
    String(String),
    /// A space id given as an already-optional value.
    Option(Option<String>),
}

/// 
/// Implementing the From trait for String
///
impl From<String> for Space {
    fn from(s: String) -> Self {
        Space::String(s)
    }
}

///
/// Implementing the From trait for `Option<String>`
///
impl From<Option<String>> for Space {
    fn from(opt: Option<String>) -> Self {
        Space::Option(opt)
    }
}

impl Space {
    /// 
    /// Method to retrieve the value as an `Option<String>`
    ///
    pub fn get_value(&self) -> Option<String> {
        match self {
            Space::String(s) if !s.is_empty() => Some(s.clone()),
            Space::Option(opt) => opt.clone(),
            _ => None,
        }
    }

    /// Returns `true` if `get_value()` is `None` (i.e. no space id, or an empty string).
    pub fn is_none(&self) -> bool {
        self.get_value().is_none()
    }

    /// Returns `true` if `get_value()` is `Some` (i.e. a non-empty space id is set).
    pub fn is_some(&self) -> bool {
        self.get_value().is_some()
    }
}

///
/// Implementing the Display trait
///
impl std::fmt::Display for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Space::String(s) => write!(f, "{}", s),
            Space::Option(opt) => write!(f, "{}", opt.clone().unwrap_or_default()),
        }
    }
}

