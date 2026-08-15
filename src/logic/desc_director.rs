
use std::collections::HashMap;

use crate::{Descriptor, descriptor_facade::DescriptorFacade, descriptor_store::DescriptorStore};
use crate::model::descriptor::{Point, Name, Label, Description};

/// Orchestrates descriptor creation, listing and lookup as sequences of steps for specific
/// tasks, abstracting away details like how storage and indexing are implemented — those are
/// delegated to a `DescriptorFacade<T>`.
#[derive(Clone)]
pub struct DescDirector <T:DescriptorStore> {
   descriptors: DescriptorFacade<T>,
}

impl<T:DescriptorStore> DescDirector<T> {

    /// Wraps a `DescriptorFacade<T>` in a new `DescDirector`.
    pub fn new(descriptors: DescriptorFacade<T>) -> Self {
        DescDirector{descriptors}
    }

    ///
    /// Creates and saves Descriptor and indexes.
    /// Newlines and surrounding white spaces in the single line fields are automatically filtered
    /// out.
    ///
    pub fn create_desc(&self, point: &str, name: &str, label: &str, description: &str) -> Descriptor {
        let mut desc = Descriptor {
            point: Point(point.trim().replace("\n", "").replace("\r", "")),
            name: Some(Name(name.trim().replace("\n", "").replace("\r", ""))),
            label: Some(Label(label.trim().replace("\n", "").replace("\r", ""))),
            description: Some(Description(description.trim().to_string())),
            desc_id: None,
        };
        let desc_id = self.descriptors.add_desc(desc.clone());
        desc.set_desc_id(&desc_id);
        self.descriptors.add_desc_index(desc.clone());
        desc
    }

    ///
    /// Returns a list with all descriptor notes.
    ///
    pub fn ls_descriptor_notes(&self) -> String {
        let descs = self.descriptors.get_all_descs();
        descs.iter().enumerate().map(|(c, d)| format!("{}: {} {} {} {}\n", c,
            d.point,
            d.name.as_deref().unwrap_or(""),
            d.label.as_deref().unwrap_or(""),
            d.description.as_deref().unwrap_or("")))
            .collect::<String>()
    }

    ///
    /// Returns a list with all descriptor notes annotated with line numbers.
    ///
    pub fn get_desc_ls_line_number(&self, line_number: &str) -> String {
        let descs: Vec<String> = self.descriptors.get_all_desc_ids();
        line_number.parse::<usize>().ok()
            .and_then(|i| descs.get(i).cloned())
            .unwrap_or_default()
    }

    ///
    /// Returns a HashMap where the entry values are Descriptor Notes and their points are the keys.
    ///
    pub fn get_descs_hashmap_for_list(&self, list: Vec<String>) -> HashMap<String, Descriptor> {
        self.descriptors.get_descs_hashmap_for_list(list)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::descriptor_store_fs::DescriptorStoreFS;
    use crate::model::{app::App, space::Space};

    fn new_director(space_id: &str) -> DescDirector<DescriptorStoreFS> {
        DescDirector::new(DescriptorFacade::new(DescriptorStoreFS::new(
            App::from("ig_desc_test_app".to_string()),
            Space::from(space_id.to_string()),
            "ig_desc_test_config".to_string(),
        )))
    }

    #[test]
    fn create_desc_trims_and_strips_newlines_from_single_line_fields() {
        let space_id = format!("ig_desc_test_director_create_{}", std::process::id());
        let director = new_director(&space_id);

        let desc = director.create_desc(" widget-1 \n", " Widget \nOne ", " lbl\r\n ", " multi\nline\ndescription ");

        assert_eq!(desc.point.0, "widget-1");
        assert_eq!(desc.name.as_deref(), Some("Widget One"));
        assert_eq!(desc.label.as_deref(), Some("lbl"));
        assert_eq!(desc.description.as_deref(), Some("multi\nline\ndescription"));
        assert!(desc.desc_id.is_some());
    }

    #[test]
    fn get_desc_ls_line_number_returns_empty_string_for_an_out_of_range_or_invalid_index() {
        let space_id = format!("ig_desc_test_director_line_invalid_{}", std::process::id());
        let director = new_director(&space_id);
        director.create_desc("widget-1", "Widget One", "", "");

        assert_eq!(director.get_desc_ls_line_number("not-a-number"), "");
        assert_eq!(director.get_desc_ls_line_number("999"), "");
    }

    #[test]
    fn get_desc_ls_line_number_returns_the_desc_id_at_that_line() {
        let space_id = format!("ig_desc_test_director_line_valid_{}", std::process::id());
        let director = new_director(&space_id);
        let desc = director.create_desc("widget-1", "Widget One", "", "");

        assert_eq!(director.get_desc_ls_line_number("0"), desc.desc_id.unwrap().0);
    }

    #[test]
    fn ls_descriptor_notes_does_not_panic_on_an_empty_space() {
        // Regression test: previously used .reduce(...).unwrap(), which panics when there are
        // no descriptors yet - same bug shape as edge_director::prettify, fixed the same way.
        let space_id = format!("ig_desc_test_director_ls_empty_{}", std::process::id());
        let director = new_director(&space_id);

        assert_eq!(director.ls_descriptor_notes(), "");
    }

    #[test]
    fn ls_descriptor_notes_lists_every_descriptor() {
        let space_id = format!("ig_desc_test_director_ls_{}", std::process::id());
        let director = new_director(&space_id);
        director.create_desc("widget-1", "Widget One", "", "");

        assert!(director.ls_descriptor_notes().contains("widget-1"));
    }
}
