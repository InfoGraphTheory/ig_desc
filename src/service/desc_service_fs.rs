
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_sets_org_space_and_leaves_tmp_space_unset() {
        let space_id = format!("ig_desc_test_service_new_{}", std::process::id());
        let service = DescServiceFS::new(
            App::from("ig_desc_test_app".to_string()),
            Space::from(space_id.clone()),
            "ig_desc_test_config".to_string(),
        );
        assert_eq!(service.org_space.get_value(), Some(space_id));
        assert!(service.tmp_space.is_none());
    }

    #[test]
    fn create_desc_round_trips_through_the_real_store() {
        let space_id = format!("ig_desc_test_service_create_{}", std::process::id());
        let service = DescServiceFS::new(
            App::from("ig_desc_test_app".to_string()),
            Space::from(space_id),
            "ig_desc_test_config".to_string(),
        );

        let desc = service.create_desc("widget-1", "Widget One", "", "");

        let map = service.get_descs_hashmap_for_list(vec!["widget-1".to_string()]);
        assert_eq!(map.get("widget-1").and_then(|d| d.desc_id.clone()), desc.desc_id);
    }

    // Note: tmp_space here is plain bookkeeping on DescServiceFS itself - DescDirector /
    // DescriptorFacade / DescriptorStoreFS never read it, so setting it has no effect on where
    // create_desc/get_descs_hashmap_for_list actually read or write. This documents current
    // behavior rather than a routing guarantee (same finding as TrServiceFS - see REVIEW_ig_tr.md).
    #[test]
    fn set_tmp_space_id_and_revert_space_id_update_the_field_but_not_storage_routing() {
        let space_id = format!("ig_desc_test_service_revert_{}", std::process::id());
        let mut service = DescServiceFS::new(
            App::from("ig_desc_test_app".to_string()),
            Space::from(space_id.clone()),
            "ig_desc_test_config".to_string(),
        );

        service.set_tmp_space_id("some-other-space".to_string());
        assert_eq!(service.tmp_space.as_ref().and_then(|s| s.get_value()), Some("some-other-space".to_string()));

        service.revert_space_id();
        assert_eq!(service.tmp_space.as_ref().and_then(|s| s.get_value()), Some(space_id));
    }
}
