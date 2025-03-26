use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Space {
    String(String),
    Option(Option<String>),
}

// Implementing the From trait for String
impl From<String> for Space {
    fn from(s: String) -> Self {
        Space::String(s)
    }
}

// Implementing the From trait for Option<String>
impl From<Option<String>> for Space {
    fn from(opt: Option<String>) -> Self {
        Space::Option(opt)
    }
}

impl Space {
    // Method to retrieve the value as an Option<String>
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

// Implementing the ToString trait
impl ToString for Space {
    fn to_string(&self) -> String {
        match self {
            Space::String(s) => s.clone(),
            Space::Option(opt) => opt.clone().unwrap_or_default(),
        }
    }
}

//fn process_string(input: Space) {
//    match input.get_value() {
//        Some(s) => println!("Received a valid String: {}", s),
//        None => println!("Received None or an empty String"),
//    }
//}
/*
fn main() {
    let my_string: Space = "Hello, World!".to_string().into();
    let my_option_some: Space = Some("Hello, Option!".to_string()).into();
    let my_option_none: Space = None.into();
    let empty_string: Space = "".to_string().into();

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
