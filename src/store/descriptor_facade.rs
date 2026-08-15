use crate::{Descriptor, model::descriptor::DescId};
use delve::{EnumFromStr, EnumToStr};
use std::collections::HashMap;
use super::descriptor_store::DescriptorStore;
use crate::misc::descriptor_tools;


/// Identifies which of a Descriptor's four indexed fields (point, name, label, description) an
/// index operation targets.
#[derive(EnumFromStr, EnumToStr, Clone)]
pub enum DescIndex {
    /// The `point` index.
    DescPointIndex,
    /// The `name` index.
    DescNameIndex,
    /// The `label` index.
    DescLabelIndex,
    /// The `description` index.
    DescDescIndex,
}

impl std::fmt::Display for DescIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            DescIndex::DescPointIndex => "Desc_point_index",
            DescIndex::DescNameIndex => "Desc_name_index",
            DescIndex::DescLabelIndex => "Desc_label_index",
            DescIndex::DescDescIndex => "Desc_desc_index",
        };
        write!(f, "{}", s)
    }
}


/// A facade over a `DescriptorStore`, providing higher-level descriptor creation, indexing and
/// retrieval operations built on top of the store's lower-level methods.
#[derive(Clone)]
pub struct DescriptorFacade<T:DescriptorStore> {
    storage: T,
}

impl<T:DescriptorStore> DescriptorFacade<T> {

    /// Wraps a `DescriptorStore` implementation in a new `DescriptorFacade`.
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
        result.desc_id = Some(DescId(desc_id));
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

    /// Returns the Descriptor stored for each of the given points, in order.
    pub fn get_descs(&self, points: Vec<&str>) -> Vec<Descriptor> {
        self.storage.get_descs(points)
    }

    /// Returns the Descriptor stored for each of the given points, in order, falling back to a
    /// point-only Descriptor for any point that isn't found.
    pub fn get_descs_or_else_ids(&self, points: Vec<String>) -> Vec<Descriptor> {

        self.storage.get_descs_or_else_ids(points)
    }

    /// Returns a HashMap where the entry values are Descriptor Notes and their points are the keys.
    pub fn get_descs_hashmap_for_list(&self, list: Vec<String>) -> HashMap<String, Descriptor> {
        let mut descs: HashMap<String, Descriptor> = HashMap::new();
        for x in self.get_descs_or_else_ids(list) {
            descs.insert(x.point.to_string(), x);
        }
        descs
    }

    /// Returns all stored Descriptors.
    pub fn get_all_descs(&self) -> Vec<Descriptor> {
        self.storage.get_all_descs()
    }

    /// Returns the `desc_id` of every stored Descriptor, read from the point index.
    pub fn get_all_desc_ids(&self) -> Vec<String> {
        let point_indexes = self.storage.get_desc_point_indexes();
        let lines = point_indexes.lines();
        let ids: Vec<String> = lines.map(|x|{x.split_once(' ').unwrap().1.to_string()}).collect();
        ids
    }

    /// Returns the Descriptor stored at the given point.
    pub fn get_desc(&self, name: &str) -> Descriptor {
        self.storage.get_desc(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::descriptor_store_fs::DescriptorStoreFS;
    use crate::model::{app::App, space::Space};
    use crate::model::descriptor::{Point, Name};

    fn new_facade(space_id: &str) -> DescriptorFacade<DescriptorStoreFS> {
        DescriptorFacade::new(DescriptorStoreFS::new(
            App::from("ig_desc_test_app".to_string()),
            Space::from(space_id.to_string()),
            "ig_desc_test_config".to_string(),
        ))
    }

    #[test]
    fn add_desc_n_index_sets_the_desc_id_and_the_descriptor_is_retrievable_by_point() {
        let space_id = format!("ig_desc_test_facade_add_{}", std::process::id());
        let facade = new_facade(&space_id);

        let saved = facade.add_desc_n_index(Descriptor {
            point: Point("widget-1".to_string()),
            name: Some(Name("Widget One".to_string())),
            ..Default::default()
        });
        assert!(saved.desc_id.is_some());

        let fetched = facade.get_desc("widget-1");
        assert_eq!(fetched.point.0, "widget-1");
        assert_eq!(fetched.name.as_deref(), Some("Widget One"));
    }

    #[test]
    fn get_descs_or_else_ids_falls_back_to_a_point_only_descriptor_when_not_found() {
        let space_id = format!("ig_desc_test_facade_fallback_{}", std::process::id());
        let facade = new_facade(&space_id);
        facade.add_desc_n_index(Descriptor {
            point: Point("known".to_string()),
            name: Some(Name("Known".to_string())),
            ..Default::default()
        });

        let descs = facade.get_descs_or_else_ids(vec!["known".to_string(), "unknown".to_string()]);

        assert_eq!(descs.len(), 2);
        assert_eq!(descs[0].name.as_deref(), Some("Known"));
        assert_eq!(descs[1].point.0, "unknown");
        assert!(descs[1].name.is_none());
    }

    #[test]
    fn get_descs_hashmap_for_list_keys_by_point() {
        let space_id = format!("ig_desc_test_facade_hashmap_{}", std::process::id());
        let facade = new_facade(&space_id);
        facade.add_desc_n_index(Descriptor {
            point: Point("a".to_string()),
            name: Some(Name("A".to_string())),
            ..Default::default()
        });

        let map = facade.get_descs_hashmap_for_list(vec!["a".to_string()]);

        assert_eq!(map.get("a").and_then(|d| d.name.as_deref()), Some("A"));
    }

    #[test]
    fn get_all_descs_returns_every_descriptor_added_in_the_space() {
        let space_id = format!("ig_desc_test_facade_all_{}", std::process::id());
        let facade = new_facade(&space_id);
        facade.add_desc_n_index(Descriptor { point: Point("a".to_string()), ..Default::default() });
        facade.add_desc_n_index(Descriptor { point: Point("b".to_string()), ..Default::default() });

        let all = facade.get_all_descs();
        let mut points: Vec<String> = all.iter().map(|d| d.point.to_string()).collect();
        points.sort();
        assert_eq!(points, vec!["a".to_string(), "b".to_string()]);
    }

    #[test]
    fn get_all_desc_ids_reads_ids_back_from_the_point_index() {
        let space_id = format!("ig_desc_test_facade_ids_{}", std::process::id());
        let facade = new_facade(&space_id);
        let saved = facade.add_desc_n_index(Descriptor { point: Point("a".to_string()), ..Default::default() });

        let ids = facade.get_all_desc_ids();
        assert!(ids.contains(&saved.desc_id.unwrap().0));
    }
}
