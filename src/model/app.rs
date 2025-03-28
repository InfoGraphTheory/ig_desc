use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum App {
    String(String),
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
impl ToString for App {
    fn to_string(&self) -> String {
        match self {
            App::String(s) => s.clone(),
            App::Option(opt) => opt.clone().unwrap_or_default(),
        }
    }
}

