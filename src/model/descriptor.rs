
#[derive(Debug)]
#[derive(PartialEq)]
pub struct Descriptor {
    pub desc_id: String, 
    pub point: String, 
    pub name: String, 
    pub label: String, 
    pub description: String, 
}

impl Descriptor {

    pub fn set_desc_id(&mut self, desc_id:&str){
        self.desc_id = desc_id.to_string();
    }

    pub fn set_name(&mut self, name:&str){
        self.name = name.to_string();
    }

    pub fn set_label(&mut self, label:&str){
        self.label = label.to_string();
    }

    pub fn set_description(&mut self, description: &str) {
        self.description = description.to_string();
    }
}

impl Descriptor {
    fn new(point: &str, name: &str, label: &str, description: &str) -> Descriptor{
        Descriptor{
            point: point.to_string(), 
            desc_id: "".to_string(), 
            name: name.to_string(), 
            label: label.to_string(), 
            description: description.to_string()
        }
    }
}

impl Default for Descriptor {
    fn default() -> Self {
        Descriptor {
            point: "".to_string(),
            desc_id: "".to_string(),
            name: "".to_string(),
            label: "".to_string(),
            description: "".to_string(),
        }
   }
}

impl Clone for Descriptor{
    fn clone(&self) -> Self {
        Descriptor{
            point: self.point.clone(),
            desc_id: self.desc_id.clone(),
            name: self.name.clone(),
            label: self.label.clone(),
            description: self.description.clone(),
        } 
    }
}

impl From<Descriptor> for String {
    fn from(desc: Descriptor) -> String {
        let mut one_string = String::new();
        one_string.push_str(&desc.point);
        one_string.push('\n');
        one_string.push_str(&desc.name);
        one_string.push('\n');
        one_string.push_str(&desc.label);
        one_string.push('\n');
        one_string.push_str(&desc.description);
        one_string 
    }
}

//todo: new::impl from one String
//question if it should be from, into trait implementations  
impl From<String> for Descriptor {
    fn from(string: String) -> Descriptor {
        if string.is_empty() {
            let id = "";
            let name = "";
            let label = "";
            let description = "";
            Descriptor::new(id, name, label, description)
 
        } else {
            let mut lines = string.lines();
            let id = lines.next().unwrap();
            let name = lines.next().unwrap_or("");
            let label = lines.next().unwrap_or("");
            let description = lines.next().unwrap_or("");
            Descriptor::new(id, name, label, description)
        }
    }
}

#[allow(dead_code)]
pub(crate) fn mock() -> Descriptor {
    Descriptor {
        point: "point".to_string(),
        desc_id: "".to_string(),
        name: "name".to_string(),
        label: "label".to_string(),
        description: "description\nWhich may be \nmultiple lines \nlong.".to_string(),
    }
}

impl  Descriptor {
    #[allow(dead_code)]
    pub fn mock_with_id(identifier_label: &str) -> Descriptor {
        let  mut point = "".to_string();
        point.push_str(identifier_label);
        let  mut desc_id = "".to_string();
        desc_id.push_str(identifier_label);
        let mut name = "".to_string();
        name.push_str(identifier_label);
        let mut label = "".to_string();
        label.push_str(identifier_label);
        let mut description = "Description\nWhich may be \nmultiple lines \nlong".to_string();
        description.push_str(identifier_label);
        Descriptor {point, desc_id, name, label, description}
    }
}

#[test]
fn to_one_string_test(){
    let descriptor = Descriptor {
        point: "point".to_string(),
        desc_id:"desc_id".to_string(),
        name: "name".to_string(),
        label: "label".to_string(),
        description: "description\nWhich may be \nmultiple lines \nlong.".to_string(),
    };
    let ideal: String = String::from(
"point
name
label
description\nWhich may be \nmultiple lines \nlong.");

print!("{}", String::from(descriptor.clone()));
   assert_eq!(String::from(descriptor), ideal);
}

