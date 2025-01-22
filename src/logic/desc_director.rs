
use std::collections::HashMap;

use crate::{Descriptor, descriptor_facade::DescriptorFacade, descriptor_store::DescriptorStore};
#[derive(Clone)]
pub struct DescDirector <T:DescriptorStore>{
   descriptors: DescriptorFacade<T>,
}

impl<T:DescriptorStore> DescDirector<T> {

    pub fn new(descriptors: DescriptorFacade<T>) -> Self {
        DescDirector{descriptors}
    }

    pub fn create_desc(&self, point: String, name: String, label: String, description: String) -> Descriptor{
        let mut desc = Descriptor{
            point: point.trim().to_string(),
            desc_id: "".trim().to_string(),
            name: name.trim().to_string(),
            label: label.trim().to_string(),
            description: description.trim().to_string(),
        };
        let desc_id = self.descriptors.add_desc(desc.clone());
        self.descriptors.add_desc_index(desc.clone());
        desc.set_desc_id(&desc_id);
        desc
    }

    pub fn ls_descriptor_notes(&self) -> String {
        let descs = self.descriptors.get_all_descs();
        descs.iter().enumerate().map(|(c,d)| format!("{}: {} {} {} {}\n",c, d.point, d.name, d.label, d.description))
            .reduce(|mut result, var| { result.push_str(&var); result}).unwrap()
    }

    pub fn get_desc_ls_line_number(&self, line_number: String) -> String {
        let descs: Vec<String> = self.descriptors.get_all_desc_ids();
        let desc: Option<String> = descs.get(line_number.parse::<usize>().unwrap()).cloned();
        desc.unwrap_or("".to_string()).to_string()
    }

    pub fn get_descs_hashmap_for_list(&self, list: Vec<String>) -> HashMap<String, Descriptor> {
        self.descriptors.get_descs_hashmap_for_list(list)
    }
}
