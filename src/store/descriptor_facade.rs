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
    }

    pub fn get_descs(&self, points: Vec<&str>) -> Vec<Descriptor> {
        self.storage.get_descs(points)
    }

    pub fn get_descs_or_else_ids(&self, points: Vec<String>) -> Vec<Descriptor> {
    
        self.storage.get_descs_or_else_ids(points)
    }

    pub fn get_descs_hashmap_for_list(&self, list: Vec<String>) -> HashMap<String, Descriptor> {
        let mut descs: HashMap<String, Descriptor> = HashMap::new();
        self.get_descs_or_else_ids(list)
            .iter()
            .for_each(|x|{
                descs.insert(x.point.clone(), x.clone());
            });
         descs   
    }

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
        self.storage.get_desc(name)
    }
}

