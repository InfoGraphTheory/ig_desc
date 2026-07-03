
macro_rules! impl_str_newtype {
    ($t:ident) => {
        #[doc = concat!("Newtype wrapper around a `String` used for a `Descriptor`'s `", stringify!($t), "` field.")]
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


/// A descriptor note: a required `point`, with optional `name`, `label` and `description` fields.
///
/// `point` need not be unique — several descriptors may describe the same `point`. Uniqueness
/// comes from `desc_id`, a hash of the descriptor's fields that is computed and assigned once
/// the descriptor is persisted; it is `None` on a freshly constructed descriptor.
#[derive(Debug, PartialEq, Clone)]
pub struct Descriptor {
    /// Unique identifier assigned once the descriptor has been persisted. `None` until then.
    pub desc_id: Option<DescId>,
    /// The descriptor's point of reference. Not required to be unique.
    pub point: Point,
    /// Optional short name.
    pub name: Option<Name>,
    /// Optional short label.
    pub label: Option<Label>,
    /// Optional, possibly multi-line, description.
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
    /// Sets `desc_id`, trimming whitespace and stripping newlines. Sets to `None` if the result is empty.
    pub fn set_desc_id(&mut self, desc_id: &str) {
        let s = desc_id.trim().replace("\n", "").replace("\r", "");
        self.desc_id = if s.is_empty() { None } else { Some(DescId(s)) };
    }

    /// Sets `name`, trimming whitespace and stripping newlines. Sets to `None` if the result is empty.
    pub fn set_name(&mut self, name: &str) {
        let s = name.trim().replace("\n", "").replace("\r", "");
        self.name = if s.is_empty() { None } else { Some(Name(s)) };
    }

    /// Sets `label`, trimming whitespace and stripping newlines. Sets to `None` if the result is empty.
    pub fn set_label(&mut self, label: &str) {
        let s = label.trim().replace("\n", "").replace("\r", "");
        self.label = if s.is_empty() { None } else { Some(Label(s)) };
    }

    /// Sets `description`, trimming surrounding whitespace. Internal newlines are preserved since
    /// a description may span multiple lines. Sets to `None` if the result is empty.
    pub fn set_description(&mut self, description: &str) {
        self.description = if description.is_empty() { None } else { Some(Description(description.to_string())) };
    }
}

/// Serializes a `Descriptor` to its on-disk line format: `point`, `name`, `label`, then
/// `description` — one field per line, in that order. `point`, `name` and `label` are
/// trimmed and stripped of embedded newlines; `description` may itself span multiple lines.
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

/// Parses the line format produced by `From<Descriptor> for String`: the first three lines
/// are `point`, `name` and `label`; everything after that is joined back together (with `\n`)
/// as `description`. Missing trailing fields default to empty/`None`.
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
    /// Builds a `Descriptor` for tests where every field (`point`, `desc_id`, `name`, `label`,
    /// `description`) is derived from `identifier_label`, so distinct labels produce distinct,
    /// easily distinguishable descriptors.
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
