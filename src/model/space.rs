use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Space {
    String(String),
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
/// Implementing the From trait for Option<String>
///
impl From<Option<String>> for Space {
    fn from(opt: Option<String>) -> Self {
        Space::Option(opt)
    }
}

impl Space {
    /// 
    /// Method to retrieve the value as an Option<String>
    ///
    pub fn get_value(&self) -> Option<String> {
        match self {
            Space::String(s) if !s.is_empty() => Some(s.clone()),
            Space::Option(opt) => opt.clone(),
            _ => None,
        }
    }

    pub fn is_none(&self) -> bool {
        self.get_value().is_none()
    }

    pub fn is_some(&self) -> bool {
        self.get_value().is_some()
    }
}

///
/// Implementing the ToString trait
///
impl ToString for Space {
    fn to_string(&self) -> String {
        match self {
            Space::String(s) => s.clone(),
            Space::Option(opt) => opt.clone().unwrap_or_default(),
        }
    }
}

