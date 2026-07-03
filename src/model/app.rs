use serde::{Serialize, Deserialize};

/// Used to hold information about the app such as app name and project folder.
///
/// Wraps either a bare `String` or an `Option<String>` so it can be constructed from either
/// (via the `From` impls below); `get_value` normalizes both forms to `Option<String>`,
/// treating an empty string the same as `None`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum App {
    /// An app name given directly.
    String(String),
    /// An app name given as an already-optional value.
    Option(Option<String>),
}

///
/// Implementing the From trait for String
///
impl From<String> for App {
    fn from(s: String) -> Self {
        App::String(s)
    }
}

/// 
/// Implementing the From trait for Option<String>
///
impl From<Option<String>> for App {
    fn from(opt: Option<String>) -> Self {
        App::Option(opt)
    }
}

impl App {
    ///
    ///Method to retrieve the value as an Option<String>
    ///
    pub fn get_value(&self) -> Option<String> {
        match self {
            App::String(s) if !s.is_empty() => Some(s.clone()),
            App::Option(opt) => opt.clone(),
            _ => None,
        }
    }

    /// Returns `true` if `get_value()` is `None` (i.e. no app name, or an empty string).
    pub fn is_none(&self) -> bool {
        self.get_value().is_none()
    }

    /// Returns `true` if `get_value()` is `Some` (i.e. a non-empty app name is set).
    pub fn is_some(&self) -> bool {
        self.get_value().is_some()
    }
}


///
/// Implementing the Display trait
///
impl std::fmt::Display for App {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            App::String(s) => write!(f, "{}", s),
            App::Option(opt) => write!(f, "{}", opt.clone().unwrap_or_default()),
        }
    }
}

