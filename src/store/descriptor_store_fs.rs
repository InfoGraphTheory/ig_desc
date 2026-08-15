
use crate::{Descriptor, model::{space::Space, app::App, descriptor::{Point, DescId}}};
use std::{fs, path::Path};
use super::{descriptor_store::DescriptorStore, descriptor_facade::{DescIndex, self}};
use ig_tools::file_tools;

use confy;
use serde::{Deserialize, Serialize};
use dirs;
use std::path::PathBuf;


#[derive(Clone, Serialize, Deserialize)]
struct DescConfig {

    app_parent_path: PathBuf,
    app_folder_name: String,
    space_folder_name: String,
    desc_folder_name: String,
    index_folder_name: String,
    org_space: Space,
    tmp_space: Space,
}

impl ::std::default::Default for DescConfig {
    fn default() -> Self {
        Self {
            app_parent_path: dirs::data_local_dir().unwrap_or_else(||{PathBuf::new()}),
            app_folder_name: "infospace".to_string(),
            space_folder_name: "spaces".to_string(),
            desc_folder_name: "descs".to_string(),
            index_folder_name: "indexes".to_string(),
            org_space: Space::from("default".to_string()),
            // Not Space::Option(None): confy persists this default to TOML on first use of a
            // config_name, and TOML has no representation for null, which made
            // DescriptorStoreFS::new() panic on every fresh (never-before-seen) config_name.
            // Space::get_value() already treats an empty string the same as None, so this is
            // semantically identical while being serializable.
            tmp_space: Space::String("".to_string()),
        }
    }
}


/// A filesystem-based reference implementation of `DescriptorStore`. Descriptors are stored as
/// individual files under a per-space desc folder, named by their `desc_id`; each of the four
/// fields (point, name, label, description) has its own flat-file index under a per-space index
/// folder, mapping field values to `desc_id`s. Folder locations are derived from a per-app,
/// per-space configuration loaded via `confy`.
#[derive(Clone)]
pub struct DescriptorStoreFS {
    config: DescConfig,
    app_folder_path: PathBuf,
    space_folder_path: PathBuf,
    desc_folder_path: PathBuf,
    index_folder_path: PathBuf,
}

impl ::std::default::Default for DescriptorStoreFS {
    fn default() -> Self {
        Self {
            config: DescConfig::default(),
            app_folder_path: PathBuf::new(),
            desc_folder_path: PathBuf::new(),
            index_folder_path: PathBuf::new(),
            space_folder_path: PathBuf::new(),
        }
    }
}

impl DescriptorStoreFS {
///
/// Create a new DescriptorStoreFS. 
///
/// The parameter app_name is optionally the name of the
/// application which then will be the name of the app folder containing the data.
/// If no name is given "infospace" is used as a catch all folder.
/// 
/// Parameter space_id is an optional space that data is to be stored or retrieved from, per
/// default. If no space parameter is given the data will be stored in the root desc and index
/// folders of the app folder. 
///
/// Parameter desc_config is an optional name for a configuration file storing all path and folder
/// name variables used to setup the DescriptorStoreFS. If the desc_config parameter is set the
/// method will search for it in the standard configuration folder of your Operative System.
/// If no configuration file is given, a default naming will be used by calling the Default trait
/// for the DescConfig struct.
    pub fn new(app_name: App, space_id: Space, config_name: String) -> Self {

        let mut config: DescConfig = confy::load(config_name.as_str(), None).unwrap();
        
        if app_name.get_value().is_none() {
            config.app_folder_name = "infospace".to_string();    
        } else {
            config.app_folder_name = app_name.to_string();    
        }

        if space_id.get_value().is_none() {
            config.org_space = Space::from("default".to_string());    
        } else {
            config.org_space = space_id.clone();    
        }

        let mut instance = DescriptorStoreFS { config, ..DescriptorStoreFS::default() };
        Self::init_folders(&mut instance);

        instance
    }

    ///
    /// Called when using a new space to make sure the folders for that space exists.
    /// Also sets all the folder path variables needed for read/write. 
    ///
    /// Creates the folders if not already there. The default parent folder is the default app data
    /// folder of the Operative System running the application.
    ///    
    fn init_folders(&mut self) {

        let desc_config = self.config.clone();

        let data_dir = desc_config.app_parent_path.clone();
        self.app_folder_path = data_dir.join(&desc_config.app_folder_name);

        self.space_folder_path = self.app_folder_path
            .join(&desc_config.space_folder_name)
            .join(self.get_space_id());
        let _ = fs::create_dir_all(self.space_folder_path.clone());

        self.create_desc_folder_in_folder(desc_config.clone(), self.space_folder_path.clone());

        self.create_index_folder_in_folder(desc_config.clone(), self.space_folder_path.clone());
        
    }

    ///
    /// Used to create a folder for descriptors.
    ///
    fn create_desc_folder_in_folder(&mut self, config: DescConfig, parent: PathBuf) {
        
        let desc_folder_dir: PathBuf = parent.join(config.desc_folder_name);
        self.desc_folder_path = desc_folder_dir.clone();
        let _ = fs::create_dir_all(desc_folder_dir);
    }

    ///
    /// Used to create the folder for indexes, along with the index files themselves
    /// (each index is a single flat file, not a sub folder).
    ///
    fn create_index_folder_in_folder(&mut self, config: DescConfig, parent: PathBuf) {

        let index_folder_dir = parent.join(config.index_folder_name.clone());
        self.index_folder_path = index_folder_dir.clone();
        let _ = fs::create_dir_all(index_folder_dir.clone());

        Self::create_file_if_not_there(DescIndex::DescPointIndex.to_string(), index_folder_dir.clone());
        Self::create_file_if_not_there(DescIndex::DescNameIndex.to_string(), index_folder_dir.clone());
        Self::create_file_if_not_there(DescIndex::DescLabelIndex.to_string(), index_folder_dir.clone());
        Self::create_file_if_not_there(DescIndex::DescDescIndex.to_string(), index_folder_dir.clone());
    }


    ///
    /// Creates a file if it does not already exist. 
    ///
    pub fn create_file_if_not_there(filename: String, folder: PathBuf) {
        

        let data_path: PathBuf = folder.clone().join(filename.clone());

        if !Path::new(&data_path).is_file() {
            let _ = fs::write(data_path, "");
        }
    }


    ///
    /// Composes the file system path for the index given as parameter.
    /// 
    ///
    pub fn get_index_path(&self, index: DescIndex) -> PathBuf {

        self.index_folder_path.clone().join(index.to_string())
    }

    ///
    /// As the name implies this method loads a descriptor note from the file system.
    /// It does so after composing the path to the file, based on its parameter desc_id.
    /// 
    pub fn load_desc(&self, desc_id: impl Into<String>) -> String {

        fs::read_to_string(
            self.desc_folder_path.clone()
            .join(desc_id.into())
        ).unwrap_or(String::from(""))
    }


    ///
    /// Create an index line and adds it to an index.
    /// This method is very general and therefore useful as helper method when appending to
    /// multiple indexes.
    ///
    fn append_index(id: &str, value: &str, index: String) -> String {
        let mut result = index;
        if !result.is_empty() {
            result.push('\n');
        }
        result.push_str(id);
        result.push(' ');
        result.push_str(value);
        result
    }

}


impl DescriptorStore for DescriptorStoreFS {

    // Following methods is for making it possible to change space temporary along the way.
    
    // |dynamic space handling begin|

    ///
    /// Sets a temporary space id. May be useful for smaller operations as a new DescriptorStore instance does not have to be made. 
    ///
    fn set_tmp_space_id(&mut self, space_id: String) {
        self.config.tmp_space = Space::from(space_id.clone());
        Self::init_folders(self);
    }

    ///
    /// After using this instance with a temporary space id, this function can be called to revert
    /// the used space id to the original one.
    ///
    fn revert_space_id(&mut self) {
        self.config.tmp_space = self.config.org_space.clone();
        Self::init_folders(self);
    }

    ///
    /// Return the space id used currently. It may be the original space id from when this instance
    /// was created, or it may be a temporary space id set explicitly by a call to
    /// the function set_tmp_space_id.
    ///
    fn get_space_id(&mut self) -> String {
        if self.config.tmp_space.is_none() || self.config.org_space.get_value() == self.config.tmp_space.get_value() {
            return self.config.org_space.get_value().unwrap();
        }
        self.config.tmp_space.get_value().unwrap()
    }

    // |dynamic space handling end|



    /// Returns the Descriptor stored for each of the given points, in order.
    fn get_descs(&self, points: Vec<&str>) -> Vec<Descriptor> {
        points.iter().map(|x|self.get_desc(x)).collect()
    }

    /// Returns all stored Descriptors.
    fn get_all_descs(&self) -> Vec<Descriptor> {

        let binding = self.get_desc_point_indexes();
        let lines = binding.lines();
        let mut descs: Vec<Descriptor> = Vec::new();

        let filenames: Vec<&str> = lines.filter_map(|x| x.split_once(' ').map(|y| y.1)).collect();
        for filename in filenames {
            let mut desc = Descriptor::from(self.load_desc(filename));
            desc.desc_id = Some(DescId(filename.to_string()));
            descs.push(desc);
        };

        descs
   }


    /// Returns the Descriptor stored for each of the given points, in order, falling back to a
    /// point-only Descriptor for any point that isn't found.
    fn get_descs_or_else_ids(&self, points: Vec<String>) -> Vec<Descriptor> {
        points.iter().map(|x|self.get_desc_or_id(x)).collect()
    }

    /// Returns the Descriptor stored at the given point, falling back to a point-only Descriptor
    /// if not found.
    fn get_desc_or_id(&self, name: &str) -> Descriptor {
        let binding = self.get_desc_point_indexes();
        let mut lines = binding.lines();

        let point = lines.find_map(|x| x.split_once(' ').and_then(|y| if y.0 == name { Some(y.1) } else { None }));

        let content = if let Some(p) = point { self.load_desc(p) } else { "".to_string() };
        if content.is_empty() {
            return Descriptor { point: Point(name.to_string()), ..Default::default() };
        }
        let mut desc = Descriptor::from(content);
        desc.desc_id = point.map(|p| DescId(p.to_string()));
        desc
    }


    /// Returns the Descriptor stored at the given point.
    fn get_desc(&self, name: &str) -> Descriptor {
        let binding = self.get_desc_point_indexes();
        let mut lines = binding.lines();

        let point = lines
            .find_map(|x| x.split_once(' ').and_then(|y| if y.0 == name { Some(y.1) } else { None }));

        let content = if let Some(p) = point { self.load_desc(p) } else { "".to_string() };
        let mut desc = Descriptor::from(content);
        desc.desc_id = point.map(|p| DescId(p.to_string()));
        desc
    }
    

    ///
    /// Method used to persist a Descriptor. 
    ///
    fn add_desc(&self, desc: Descriptor, id: String) {
        let description = String::from(desc.clone());
        let file_path = self.desc_folder_path.join(id);
        let _ = fs::write(file_path, description);
    }


    ///
    /// Takes a descriptor note as argument and creates indexes for its variables. 
    /// It is important that the descriptor note has a desc_id. 
    ///
    fn index_desc(&self, desc: Descriptor) {
        // Index format is "{field_value} {desc_id}": search by field_value (y.0),
        // load file by desc_id (y.1). First column can be sorted for future binary search.
        let id = desc.desc_id.as_deref().unwrap_or("");

        let point_index = self.get_desc_point_indexes();
        let point_index = Self::append_index(&desc.point, id, point_index);
        self.set_desc_point_indexes(&point_index);

        let name_index = self.get_desc_name_indexes();
        let name_index = Self::append_index(desc.name.as_deref().unwrap_or(""), id, name_index);
        self.set_desc_name_indexes(&name_index);

        let label_index = self.get_desc_label_indexes();
        let label_index = Self::append_index(desc.label.as_deref().unwrap_or(""), id, label_index);
        self.set_desc_label_indexes(&label_index);

        let desc_index = self.get_desc_description_indexes();
        let desc_index = Self::append_index(desc.description.as_deref().unwrap_or(""), id, desc_index);
        self.set_desc_description_indexes(&desc_index);
    }

    ///
    /// This method returns all indexing records of descriptors in current space, based on the point field. 
    ///
    fn get_desc_point_indexes(&self) -> String {
        let filename = self.get_index_path(DescIndex::DescPointIndex);
        fs::read_to_string(filename).expect("desc_point_index file missing - was init_folders() run?")
    }

    /// This method returns all indexing records of descriptors in current space, based on the name field.
    fn get_desc_name_indexes(&self) -> String  {

        let filename = self.get_index_path(DescIndex::DescNameIndex);
        fs::read_to_string(filename).expect("desc_name_index file missing - was init_folders() run?")
    }

    /// This method returns all indexing records of descriptors in current space, based on the label field.
    fn get_desc_label_indexes(&self) -> String  {

        let filename = self.get_index_path(DescIndex::DescLabelIndex);
        fs::read_to_string(filename).expect("desc_label_index file missing - was init_folders() run?")
    }

    /// This method returns all indexing records of descriptors in current space, based on the description field.
    fn get_desc_description_indexes(&self) -> String  {

        let filename = self.get_index_path(DescIndex::DescDescIndex);
        fs::read_to_string(filename).expect("desc_description_index file missing - was init_folders() run?")
    }

    ///
    /// This method returns all indexing records of descriptors based on the point field, for the
    /// space specified with the method parameter space_id. 
    /// The method makes a temporary switch to the new space_id and then reverts the object back to
    /// its original space_id.
    ///
    fn get_tmp_space_desc_point_indexes(&mut self, space_id: String) -> String {

        self.set_tmp_space_id(space_id);
        let filename = self.get_index_path(DescIndex::DescPointIndex);
        self.revert_space_id();
        fs::read_to_string(filename).expect("desc_point_index file missing - was init_folders() run?")
    }


    /// Overwrites the point index with the given contents.
    fn set_desc_point_indexes(&self, lines: &str) {

        file_tools::write(self.get_index_path(descriptor_facade::DescIndex::DescPointIndex), lines);
    }

    /// Overwrites the name index with the given contents.
    fn set_desc_name_indexes(&self, lines: &str){
        file_tools::write(self.get_index_path(descriptor_facade::DescIndex::DescNameIndex), lines);
    }

    /// Overwrites the label index with the given contents.
    fn set_desc_label_indexes(&self, lines: &str) {
        file_tools::write(self.get_index_path(descriptor_facade::DescIndex::DescLabelIndex), lines);
    }

    /// Overwrites the description index with the given contents.
    fn set_desc_description_indexes(&self, lines:&str) {
        file_tools::write(self.get_index_path(descriptor_facade::DescIndex::DescDescIndex), lines);
    }

}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::store::descriptor_facade::DescriptorFacade;
    use crate::logic::desc_director::DescDirector;
    use crate::model::descriptor::{Name, Label, Description};
    use crate::DescId;

    fn temp_dir(tag: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("ig_desc_test_{}_{}", std::process::id(), tag));
        let _ = fs::remove_dir_all(&dir);
        dir
    }

    fn new_store_in(temp: &PathBuf) -> DescriptorStoreFS {
        let mut store = DescriptorStoreFS::default();
        store.config = DescConfig { app_parent_path: temp.clone(), ..DescConfig::default() };
        DescriptorStoreFS::init_folders(&mut store);
        store
    }

    fn new_director_in(temp: &PathBuf) -> DescDirector<DescriptorStoreFS> {
        DescDirector::new(DescriptorFacade::new(new_store_in(temp)))
    }

    #[test]
    fn init_folders_can_be_called_repeatedly_and_index_files_stay_readable() {
        let temp = temp_dir("init");
        let mut store = new_store_in(&temp);
        // init_folders is also called by set_tmp_space_id/revert_space_id, so it must be safe
        // to call more than once without breaking the index files it created.
        DescriptorStoreFS::init_folders(&mut store);

        assert_eq!(store.get_desc_point_indexes(), "");
        assert_eq!(store.get_desc_name_indexes(), "");
        assert_eq!(store.get_desc_label_indexes(), "");
        assert_eq!(store.get_desc_description_indexes(), "");

        let _ = fs::remove_dir_all(&temp);
    }

    #[test]
    fn create_desc_persists_and_is_retrievable_by_point() {
        let temp = temp_dir("create_retrieve");
        let director = new_director_in(&temp);

        let created = director.create_desc("my-point", "my name", "my label", "my description");

        assert!(created.desc_id.is_some(), "desc_id should be set after create");
        assert_eq!(&*created.point, "my-point");

        let facade = DescriptorFacade::new(new_store_in(&temp));
        let retrieved = facade.get_desc("my-point");
        assert_eq!(&*retrieved.point, "my-point");
        assert_eq!(retrieved.name.as_deref(), Some("my name"));
        assert_eq!(retrieved.label.as_deref(), Some("my label"));
        assert_eq!(retrieved.description.as_deref(), Some("my description"));
        // Regression: get_desc used to always return desc_id: None, even for a descriptor it
        // just found and loaded successfully, because Descriptor::from(String) has no way to
        // know its own filename and get_desc never patched it back in.
        assert_eq!(retrieved.desc_id, created.desc_id);

        let _ = fs::remove_dir_all(&temp);
    }

    #[test]
    fn get_all_descs_returns_all_created_descriptors() {
        let temp = temp_dir("get_all");
        let director = new_director_in(&temp);

        let created_a = director.create_desc("point-a", "name a", "label a", "desc a");
        director.create_desc("point-b", "name b", "label b", "desc b");

        let facade = DescriptorFacade::new(new_store_in(&temp));
        let all = facade.get_all_descs();
        assert_eq!(all.len(), 2);

        let points: Vec<&str> = all.iter().map(|d| d.point.as_ref()).collect();
        assert!(points.contains(&"point-a"));
        assert!(points.contains(&"point-b"));

        // Regression: get_all_descs used to always return desc_id: None (same root cause as
        // get_desc above) even though it reads the id straight off the index it just parsed.
        let found_a = all.iter().find(|d| &*d.point == "point-a").unwrap();
        assert_eq!(found_a.desc_id, created_a.desc_id);

        let _ = fs::remove_dir_all(&temp);
    }

    #[test]
    fn get_desc_or_id_returns_point_only_descriptor_when_not_found() {
        let temp = temp_dir("or_id_fallback");
        let store = new_store_in(&temp);

        let result = store.get_desc_or_id("unknown-point");
        assert_eq!(&*result.point, "unknown-point");
        assert!(result.desc_id.is_none());
        assert!(result.name.is_none());

        let _ = fs::remove_dir_all(&temp);
    }

    #[test]
    fn index_desc_writes_field_value_then_desc_id() {
        let temp = temp_dir("index_format");
        let store = new_store_in(&temp);

        let desc = Descriptor {
            point: Point("my-point".to_string()),
            desc_id: Some(DescId("abc123".to_string())),
            name: Some(Name("my-name".to_string())),
            label: Some(Label("my-label".to_string())),
            description: Some(Description("my-desc".to_string())),
        };
        store.index_desc(desc);

        assert_eq!(store.get_desc_point_indexes(), "my-point abc123");
        assert_eq!(store.get_desc_name_indexes(), "my-name abc123");
        assert_eq!(store.get_desc_label_indexes(), "my-label abc123");
        assert_eq!(store.get_desc_description_indexes(), "my-desc abc123");

        let _ = fs::remove_dir_all(&temp);
    }
}

