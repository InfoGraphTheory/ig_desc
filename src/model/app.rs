use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum App {
    String(String),
    Option(Option<String>),
}

// Implementing the From trait for String
impl From<String> for App {
    fn from(s: String) -> Self {
        App::String(s)
    }
}

// Implementing the From trait for Option<String>
impl From<Option<String>> for App {
    fn from(opt: Option<String>) -> Self {
        App::Option(opt)
    }
}

impl App {
    // Method to retrieve the value as an Option<String>
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


// Implementing the ToString trait
impl ToString for App {
    fn to_string(&self) -> String {
        match self {
            App::String(s) => s.clone(),
            App::Option(opt) => opt.clone().unwrap_or_default(),
        }
    }
}

/*
fn process_string(input: App) {
    match input.get_value() {
        Some(s) => println!("Received a valid String: {}", s),
        None => println!("Received None or an empty String"),
    }
}

fn main() {
    let my_string: App = "Hello, World!".to_string().into();
    let my_option_some: App = Some("Hello, Option!".to_string()).into();
    let my_option_none: App = None.into();
    let empty_string: App = "".to_string().into();

    process_string(my_string);
    process_string(my_option_some);
    process_string(my_option_none);
    process_string(empty_string);

    // Demonstrating the get_value method
    println!("Value from my_string: {:?}", my_string.get_value());
    println!("Value from my_option_some: {:?}", my_option_some.get_value());
    println!("Value from my_option_none: {:?}", my_option_none.get_value());
    println!("Value from empty_string: {:?}", empty_string.get_value());

    // Demonstrating the to_string method
    println!("String representation of my_string: {}", my_string.to_string());
    println!("String representation of my_option_some: {}", my_option_some.to_string());
    println!("String representation of my_option_none: {}", my_option_none.to_string());
    println!("String representation of empty_string: {}", empty_string.to_string());
}
*/
