
use std::collections::HashMap;

use crate::{Descriptor, descriptor_facade::DescriptorFacade, descriptor_store::DescriptorStore};

#[derive(Clone)]
pub struct DescDirector <T:DescriptorStore> {
   descriptors: DescriptorFacade<T>,
}

impl<T:DescriptorStore> DescDirector<T> {

    pub fn new(descriptors: DescriptorFacade<T>) -> Self {
        DescDirector{descriptors}
    }

    /// 
    /// Creates and saves Descriptor and indexes. 
    /// Newlines and surrounding white spaces in the single line fields are automatically filtered
    /// out.
    ///
    pub fn create_desc(&self, point: &str, name: &str, label: &str, description: &str) -> Descriptor{
        let mut desc = Descriptor{
            point: point.trim().replace("\n", "").replace("\r", "").to_string(),
            desc_id: "".trim().to_string(),
            name: name.trim().replace("\n", "").replace("\r", "").to_string(),
            label: label.trim().replace("\n", "").replace("\r", "").to_string(),
            description: description.trim().to_string(),
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
        descs.iter().enumerate().map(|(c,d)| format!("{}: {} {} {} {}\n",c, d.point, d.name, d.label, d.description))
            .reduce(|mut result, var| { result.push_str(&var); result}).unwrap()
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
