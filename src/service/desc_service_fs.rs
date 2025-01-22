
use std::collections::HashMap;
use crate::logic::desc_director::DescDirector;
use crate::{descriptor_facade::DescriptorFacade, descriptor_store_fs::DescriptorStoreFS, Descriptor};


//TODO: test that there isn't an error where indexes files get overwritten rather than appended also that files produced get
//their own directory...
//TODO: If not done already: merge the store_fs "pair" functions so that only the store trait functions are present. 
//TODO: If not done already: Change all index reads and writes to use getIndexPath and all file reads and writes to use getFilePath methods.


#[derive(Clone)]
pub struct DescServiceFS {
    pub descs: DescDirector<DescriptorStoreFS>,
    pub org_space: String,
    pub tmp_space: Option<String>,
}

impl DescServiceFS {
    pub fn new(space_id: String) -> Self {
    
        let descriptors = DescriptorStoreFS::new(space_id.clone());
        let desc_facade = DescriptorFacade::new(descriptors);

        DescServiceFS { 
            descs: DescDirector::new(desc_facade),
            org_space: space_id.clone(),
            tmp_space: Option::None,
        }    
    }

    pub fn set_tmp_space_id(&mut self, space_id: String) {
        self.tmp_space = Some(space_id);
    }

    pub fn revert_space_id(&mut self) {
        self.tmp_space = Some(self.org_space.clone());
    }

    pub fn create_desc(&self, point: String, name: String, label: String, description: String) -> Descriptor {
        
        self.descs.create_desc( point, name, label, description)
    }

    pub fn ls_descs(&self) -> String {
        self.descs.ls_descriptor_notes()
    }

    pub fn get_desc_ls_line_number(&self, line_number: String) -> String {
        self.descs.get_desc_ls_line_number(line_number)
    }

    pub fn get_descs_hashmap_for_list(&self, list: Vec<String>) -> HashMap<String, Descriptor> {
        self.descs.get_descs_hashmap_for_list(list)
    }
}
