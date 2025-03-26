use crate::Descriptor;
use delve::{EnumFromStr, EnumToStr};
use std::collections::HashMap;
use super::descriptor_store::DescriptorStore;
use crate::misc::descriptor_tools;


#[derive(EnumFromStr, EnumToStr, Clone)]
pub enum DescIndex {
    DescPointIndex,
    DescNameIndex,
    DescLabelIndex,
    DescDescIndex,
}

impl ToString for DescIndex {
    fn to_string(&self) -> String {
        match self {
            DescIndex::DescPointIndex => String::from("Desc_point_index"),
            DescIndex::DescNameIndex => String::from("Desc_name_index"),
            DescIndex::DescLabelIndex => String::from("Desc_label_index"),
            DescIndex::DescDescIndex => String::from("Desc_desc_index"),
        }
    }
}


#[derive(Clone)]
pub struct DescriptorFacade<T:DescriptorStore> {
    storage: T, 
}

impl<T:DescriptorStore> DescriptorFacade<T> {

    pub fn new(storage: T) -> Self{
        DescriptorFacade {storage}
    }

    ///
    /// This method stores a Descriptor after creating its desc_id.
    /// Before returning the Descriptor, indexes are creates also.
    ///
    pub fn add_desc_n_index(&self, desc: Descriptor) -> Descriptor {
        let desc_id = self.add_desc(desc.clone());
        let mut result = desc.clone();
        result.desc_id = desc_id;
        self.add_desc_index(result.clone());
        result
    }

    ///
    /// Stores a Descriptor after creating its desc_id.
    /// Consider using add_desc_n_index instead as it creates indexes also.
    ///
    pub fn add_desc(&self, desc: Descriptor) -> String {
        let id = descriptor_tools::get_desc_id(&desc);
        self.storage.add_desc(desc, id.clone());
        id
    }

    ///        
    /// Helper method that adds indexes to a Descriptor.
    /// Consider using add_desc_n_index as it calls this method and stores the Descriptor as well.
    ///
    pub fn add_desc_index(&self, desc: Descriptor) {
        self.storage.index_desc(desc);
//        self.add_to_desc_point_index(desc.clone());
//        self.add_to_desc_name_index(desc.clone());
//        self.add_to_desc_label_index(desc.clone());
//        self.add_to_desc_description_index(desc.clone());

    }

    pub fn get_descs(&self, points: Vec<&str>) -> Vec<Descriptor> {
    
        let content = self.storage.get_descs(points);
        content
    }

    pub fn get_descs_or_else_ids(&self, points: Vec<String>) -> Vec<Descriptor> {
    
        let content = self.storage.get_descs_or_else_ids(points);
        content
    }
/*
    pub fn get_space_descs_or_else_ids(&self, points: Vec<String>, space_id: String) -> Vec<Descriptor> {
    
        let content = self.storage.get_space_descs_or_else_ids(points, space_id);
        content
    }
*/
    pub fn get_descs_hashmap_for_list(&self, list: Vec<String>) -> HashMap<String, Descriptor> {
        let mut descs: HashMap<String, Descriptor> = HashMap::new();
        self.get_descs_or_else_ids(list)
            .iter()
            .for_each(|x|{
                descs.insert(x.point.clone(), x.clone());
            });
         descs   
    }
/*
    pub fn get_descs_hashmap_for_space_list(&self, list: Vec<String>, space_id: String) -> HashMap<String, Descriptor> {
        let mut descs: HashMap<String, Descriptor> = HashMap::new();
        self.get_space_descs_or_else_ids(list, space_id)
            .iter()
            .for_each(|x|{
                descs.insert(x.point.clone(), x.clone());
            });
         descs   
    }
*/
    pub fn get_all_descs(&self) -> Vec<Descriptor> {
        self.storage.get_all_descs()
    }

    pub fn get_all_desc_ids(&self) -> Vec<String> {
        let point_indexes = self.storage.get_desc_point_indexes();
        let lines = point_indexes.lines(); 
        let ids: Vec<String> = lines.map(|x|{x.split_once(' ').unwrap().1.to_string()}).collect();
        ids
    }

    pub fn get_desc(&self, name: &str) -> Descriptor {
    
        let content = self.storage.get_desc(name);
        Descriptor::from(content)
    }
/*
    fn add_to_desc_point_index(&self, desc: Descriptor) {

        let binding = self.storage.get_desc_point_indexes();
        let line = descriptor_tools::create_desc_point_index_line(&desc); //no good since the line
                                                                          //is medium dependent.
        let lines = list_tools::append_ln_n_sort(&line, &binding);
        self.storage.set_desc_point_indexes(&lines);
    }

    fn add_to_desc_name_index(&self, desc: Descriptor) {
        let binding = self.storage.get_desc_name_indexes();
        let line = descriptor_tools::create_desc_name_index_line(&desc);
        let lines = list_tools::append_ln_n_sort(&line, &binding);
        self.storage.set_desc_name_indexes(&lines);
    }

    fn add_to_desc_label_index(&self, desc: Descriptor) {
        let binding = self.storage.get_desc_label_indexes();
        let line = descriptor_tools::create_desc_label_index_line(&desc);
        let lines = list_tools::append_ln_n_sort(&line, &binding);
        self.storage.set_desc_label_indexes(&lines);
    }

    fn add_to_desc_description_index(&self, desc: Descriptor) {
        let binding = self.storage.get_desc_description_indexes();
        let line = descriptor_tools::create_desc_description_index_line(&desc);
        let lines = list_tools::append_ln_n_sort(&line, &binding);
        self.storage.set_desc_description_indexes(&lines);
    }
*/

}

