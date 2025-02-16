
use crate::Descriptor;
use ig_tools::hashing_tools;

pub fn get_desc_id(desc: &Descriptor) -> String {
    create_desc_id(&desc.point, &desc.name, &desc.label, &desc.description)    
}


//return digest after a bigger concat procedure
//This approach to id creation dictates that fields has to be trimmed and that a newline char
//has to be used as delimitter and that no other fields than the description can have newline
//chars inside their value. In that way we can easily split the fields but at the expense of
//newline dictatorship. 
pub fn create_desc_id(point: &str, name: &str, label: &str, description: &str) -> String {

    let mut concat: String = String::from("");
    concat.push_str(point.trim());
    concat.push('\n');
    concat.push_str(name.trim());
    concat.push('\n');
    concat.push_str(label.trim());
    concat.push('\n');
    concat.push_str(description.trim());
    hashing_tools::hash_text(&concat)
}


pub fn create_desc_point_index_line(desc: &Descriptor) -> String {
    create_desc_index_line(desc, &desc.point)
}

pub fn create_desc_name_index_line(desc: &Descriptor) -> String {
    create_desc_index_line(desc, &desc.name)
}

pub fn create_desc_label_index_line(desc: &Descriptor) -> String {
    create_desc_index_line(desc, &desc.label)
}

pub fn create_desc_description_index_line(desc: &Descriptor) -> String {
    create_desc_index_line(desc, &desc.description)
}


pub fn create_desc_index_line(desc: &Descriptor, field :&str) -> String {
    let id = self::get_desc_id(desc);
    let mut addition = field.trim().to_string();
    addition.push(' ');
    addition.push_str(&id);
    addition.to_string()
}
