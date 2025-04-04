
use std::collections::HashMap;
use crate::logic::desc_director::DescDirector;
use crate::model::app::App;
use crate::model::space::Space;
use crate::{descriptor_facade::DescriptorFacade, descriptor_store_fs::DescriptorStoreFS, Descriptor};


#[derive(Clone)]
pub struct DescServiceFS {
    pub descs: DescDirector<DescriptorStoreFS>,
    pub org_space: Space,
    pub tmp_space: Option<Space>,
    pub app_name: App,
}

impl DescServiceFS {
    pub fn new(app_name: App, space_id: Space, config: String) -> Self {
    
        let descriptors = DescriptorStoreFS::new(app_name.clone(), space_id.clone(), config);
        let desc_facade = DescriptorFacade::new(descriptors);

        DescServiceFS { 
            descs: DescDirector::new(desc_facade),
            org_space: space_id.clone(),
            tmp_space: Option::None,
            app_name, 
        }    
    }

    pub fn set_tmp_space_id(&mut self, space_id: String) {
        self.tmp_space = Some(Space::from(space_id));
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
