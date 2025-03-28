
use crate::Descriptor;
use ig_tools::hashing_tools;

pub fn get_desc_id(desc: &Descriptor) -> String {
    create_desc_id(&desc.point, &desc.name, &desc.label, &desc.description)    
}

///
/// Returns SHA digest for the data of a Descriptor after a bigger concat procedure.
///
/// Existing newline characters in anything but the multiline description are removed doing the process as these would not be allowed anyways. 
///
/// Some may notice that the string concatenation here is not far from what is produced in one
/// Descriptor's to_string methods. 
/// The reason the creation of the desc_id is not done within Descriptor is that the methodology
/// for creating a Descriptor ID would be locked in the model.
///
/// The reason a similar to_string on Descriptor is not used here is because the generator of
/// unique IDs for Descriptors would then be dependent on the formatting of an existing to_string
/// method to never change. 
///
pub fn create_desc_id(point: &str, name: &str, label: &str, description: &str) -> String {

    let mut concat: String = String::from("");
    concat.push_str(&point.trim().replace("\n", "").replace("\r", ""));
    concat.push('\n');
    concat.push_str(&name.trim().replace("\n", "").replace("\r", ""));
    concat.push('\n');
    concat.push_str(&label.trim().replace("\n", "").replace("\r", ""));
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
