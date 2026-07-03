
use std::collections::HashMap;
use crate::logic::desc_director::DescDirector;
use crate::model::app::App;
use crate::model::space::Space;
use crate::{descriptor_facade::DescriptorFacade, descriptor_store_fs::DescriptorStoreFS, Descriptor};


/// A filesystem-backed facade for creating, listing and looking up descriptors, wrapping a
/// [`DescDirector<DescriptorStoreFS>`] together with the app/space context it was built with.
/// This is the API apps using the desc notes library are meant to use.
#[derive(Clone)]
pub struct DescServiceFS {
    /// Handles descriptor creation, listing and lookup, delegating storage to `DescriptorStoreFS`.
    pub descs: DescDirector<DescriptorStoreFS>,
    /// The space id this instance was originally created with.
    pub org_space: Space,
    /// A temporary space id, set via `set_tmp_space_id`, that overrides `org_space` until
    /// `revert_space_id` is called. `None` if no temporary space is in effect.
    pub tmp_space: Option<Space>,
    /// The app name this instance operates under.
    pub app_name: App,
}

impl DescServiceFS {
    /// Creates a new `DescServiceFS` for the given app and space, backed by a
    /// `DescriptorStoreFS` built from `config`.
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

    /// Sets a temporary space id. May be useful for smaller operations as a new
    /// `DescServiceFS` instance does not have to be made.
    pub fn set_tmp_space_id(&mut self, space_id: String) {
        self.tmp_space = Some(Space::from(space_id));
    }

    /// After using this instance with a temporary space id, reverts back to the original space id.
    pub fn revert_space_id(&mut self) {
        self.tmp_space = Some(self.org_space.clone());
    }

    /// Creates and saves a Descriptor and its indexes.
    pub fn create_desc(&self, point: &str, name: &str, label: &str, description: &str) -> Descriptor {
        self.descs.create_desc(point, name, label, description)
    }

    /// Returns a list with all descriptor notes.
    pub fn ls_descs(&self) -> String {
        self.descs.ls_descriptor_notes()
    }

    /// Returns a list with all descriptor notes annotated with line numbers.
    pub fn get_desc_ls_line_number(&self, line_number: &str) -> String {
        self.descs.get_desc_ls_line_number(line_number)
    }

    /// Returns a HashMap where the entry values are Descriptor Notes and their points are the keys.
    pub fn get_descs_hashmap_for_list(&self, list: Vec<String>) -> HashMap<String, Descriptor> {
        self.descs.get_descs_hashmap_for_list(list)
    }
}
