
use crate::Descriptor;
use ig_tools::hashing_tools;

/// Computes a Descriptor's `desc_id` from its current field values (see `create_desc_id`).
pub fn get_desc_id(desc: &Descriptor) -> String {
    create_desc_id(
        &desc.point,
        desc.name.as_deref().unwrap_or(""),
        desc.label.as_deref().unwrap_or(""),
        desc.description.as_deref().unwrap_or(""),
    )
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


/// Builds a `point` index line (`"{point} {desc_id}"`) for a Descriptor.
pub fn create_desc_point_index_line(desc: &Descriptor) -> String {
    create_desc_index_line(desc, &desc.point)
}

/// Builds a `name` index line (`"{name} {desc_id}"`) for a Descriptor.
pub fn create_desc_name_index_line(desc: &Descriptor) -> String {
    create_desc_index_line(desc, desc.name.as_deref().unwrap_or(""))
}

/// Builds a `label` index line (`"{label} {desc_id}"`) for a Descriptor.
pub fn create_desc_label_index_line(desc: &Descriptor) -> String {
    create_desc_index_line(desc, desc.label.as_deref().unwrap_or(""))
}

/// Builds a `description` index line (`"{description} {desc_id}"`) for a Descriptor.
pub fn create_desc_description_index_line(desc: &Descriptor) -> String {
    create_desc_index_line(desc, desc.description.as_deref().unwrap_or(""))
}


/// Builds an index line pairing `field` with the Descriptor's `desc_id`, in the
/// `"{field} {desc_id}"` format all lookup functions expect.
pub fn create_desc_index_line(desc: &Descriptor, field: &str) -> String {
    let id = self::get_desc_id(desc);
    let mut addition = field.trim().to_string();
    addition.push(' ');
    addition.push_str(&id);
    addition.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::descriptor::{Point, Name, Label, Description};

    fn desc(point: &str, name: &str, label: &str, description: &str) -> Descriptor {
        Descriptor {
            desc_id: None,
            point: Point(point.to_string()),
            name: Some(Name(name.to_string())),
            label: Some(Label(label.to_string())),
            description: Some(Description(description.to_string())),
        }
    }

    #[test]
    fn create_desc_id_is_deterministic_for_the_same_fields() {
        let id1 = create_desc_id("p", "n", "l", "d");
        let id2 = create_desc_id("p", "n", "l", "d");
        assert_eq!(id1, id2);
    }

    #[test]
    fn create_desc_id_differs_when_any_field_differs() {
        let base = create_desc_id("p", "n", "l", "d");
        assert_ne!(base, create_desc_id("p2", "n", "l", "d"));
        assert_ne!(base, create_desc_id("p", "n2", "l", "d"));
        assert_ne!(base, create_desc_id("p", "n", "l2", "d"));
        assert_ne!(base, create_desc_id("p", "n", "l", "d2"));
    }

    #[test]
    fn create_desc_id_strips_embedded_newlines_from_point_name_and_label() {
        let clean = create_desc_id("p", "n", "l", "d");
        let with_newlines = create_desc_id("p\n", "n\r\n", "l\n", "d");
        assert_eq!(clean, with_newlines);
    }

    #[test]
    fn get_desc_id_matches_create_desc_id_for_the_descriptors_fields() {
        let d = desc("p", "n", "l", "d");
        assert_eq!(get_desc_id(&d), create_desc_id("p", "n", "l", "d"));
    }

    #[test]
    fn index_line_helpers_pair_the_field_with_the_desc_id() {
        let d = desc("p", "n", "l", "d");
        let id = get_desc_id(&d);

        assert_eq!(create_desc_point_index_line(&d), format!("p {}", id));
        assert_eq!(create_desc_name_index_line(&d), format!("n {}", id));
        assert_eq!(create_desc_label_index_line(&d), format!("l {}", id));
        assert_eq!(create_desc_description_index_line(&d), format!("d {}", id));
    }
}
