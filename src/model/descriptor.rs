
macro_rules! impl_str_newtype {
    ($t:ident) => {
        #[derive(Debug, Clone, PartialEq, Default)]
        pub struct $t(pub String);

        impl std::fmt::Display for $t {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl std::ops::Deref for $t {
            type Target = str;
            fn deref(&self) -> &str { &self.0 }
        }

        impl From<&str> for $t {
            fn from(s: &str) -> Self { Self(s.to_string()) }
        }

        impl From<String> for $t {
            fn from(s: String) -> Self { Self(s) }
        }
    }
}

impl_str_newtype!(DescId);
impl_str_newtype!(Point);
impl_str_newtype!(Name);
impl_str_newtype!(Label);
impl_str_newtype!(Description);


#[derive(Debug, PartialEq, Clone)]
pub struct Descriptor {
    pub desc_id: Option<DescId>,
    pub point: Point,
    pub name: Option<Name>,
    pub label: Option<Label>,
    pub description: Option<Description>,
}

impl Default for Descriptor {
    fn default() -> Self {
        Descriptor {
            desc_id: None,
            point: Point::default(),
            name: None,
            label: None,
            description: None,
        }
    }
}

impl Descriptor {
    pub fn set_desc_id(&mut self, desc_id: &str) {
        let s = desc_id.trim().replace("\n", "").replace("\r", "");
        self.desc_id = if s.is_empty() { None } else { Some(DescId(s)) };
    }

    pub fn set_name(&mut self, name: &str) {
        let s = name.trim().replace("\n", "").replace("\r", "");
        self.name = if s.is_empty() { None } else { Some(Name(s)) };
    }

    pub fn set_label(&mut self, label: &str) {
        let s = label.trim().replace("\n", "").replace("\r", "");
        self.label = if s.is_empty() { None } else { Some(Label(s)) };
    }

    pub fn set_description(&mut self, description: &str) {
        self.description = if description.is_empty() { None } else { Some(Description(description.to_string())) };
    }
}

impl From<Descriptor> for String {
    fn from(desc: Descriptor) -> String {
        let mut s = String::new();
        s.push_str(&desc.point.trim().replace("\n", "").replace("\r", ""));
        s.push('\n');
        s.push_str(&desc.name.as_deref().unwrap_or("").trim().replace("\n", "").replace("\r", ""));
        s.push('\n');
        s.push_str(&desc.label.as_deref().unwrap_or("").trim().replace("\n", "").replace("\r", ""));
        s.push('\n');
        s.push_str(desc.description.as_deref().unwrap_or("").trim());
        s
    }
}

impl From<String> for Descriptor {
    fn from(string: String) -> Descriptor {
        if string.is_empty() {
            return Descriptor::default();
        }
        let mut lines = string.lines();
        let point = lines.next().unwrap_or("").to_string();
        let name = lines.next().unwrap_or("").to_string();
        let label = lines.next().unwrap_or("").to_string();
        let description = lines.collect::<Vec<_>>().join("\n");
        Descriptor {
            point: Point(point),
            name: if name.is_empty() { None } else { Some(Name(name)) },
            label: if label.is_empty() { None } else { Some(Label(label)) },
            description: if description.is_empty() { None } else { Some(Description(description)) },
            ..Default::default()
        }
    }
}

#[allow(dead_code)]
pub(crate) fn mock() -> Descriptor {
    Descriptor {
        point: Point("point".to_string()),
        desc_id: None,
        name: Some(Name("name".to_string())),
        label: Some(Label("label".to_string())),
        description: Some(Description("description\nWhich may be \nmultiple lines \nlong.".to_string())),
    }
}

impl Descriptor {
    #[allow(dead_code)]
    pub fn mock_with_id(identifier_label: &str) -> Descriptor {
        Descriptor {
            point: Point(identifier_label.to_string()),
            desc_id: Some(DescId(identifier_label.to_string())),
            name: Some(Name(identifier_label.to_string())),
            label: Some(Label(identifier_label.to_string())),
            description: Some(Description(format!("Description\nWhich may be \nmultiple lines \nlong{}", identifier_label))),
        }
    }
}

#[test]
fn to_one_string_test() {
    let descriptor = Descriptor {
        point: Point("point".to_string()),
        desc_id: Some(DescId("desc_id".to_string())),
        name: Some(Name("name".to_string())),
        label: Some(Label("label".to_string())),
        description: Some(Description("description\nWhich may be \nmultiple lines \nlong.".to_string())),
    };
    let ideal = "point\nname\nlabel\ndescription\nWhich may be \nmultiple lines \nlong.".to_string();
    print!("{}", String::from(descriptor.clone()));
    assert_eq!(String::from(descriptor), ideal);
}
