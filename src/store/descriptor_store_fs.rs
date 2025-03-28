
use crate::{Descriptor, model::{space::Space, app::App}};
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
            tmp_space: Space::Option(None),
        }
    }
}


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

        let mut instance: DescriptorStoreFS = DescriptorStoreFS::default();
        instance.config = config;
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

        let desc_config: DescConfig = self.clone().config;

        let data_dir = desc_config.app_parent_path.clone();
        let _ = data_dir.join(desc_config.app_folder_name.clone());
        self.app_folder_path = data_dir.clone();

        let _ = data_dir.join(desc_config.space_folder_name.clone());
        let _ = data_dir.join(self.get_space_id());
        self.space_folder_path = data_dir.clone();
        let _ = fs::create_dir_all(data_dir.clone());
        
        self.create_desc_folder_in_folder(desc_config.clone(), data_dir.clone());

        self.create_index_folder_in_folder(desc_config.clone(), data_dir.clone());
        
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
    /// Used to create a folder for indexes, along with sub folders for specific indexes.
    ///
    fn create_index_folder_in_folder(&mut self, config: DescConfig, parent: PathBuf) {
        
        let index_folder_dir = parent.join(config.index_folder_name.clone());
        self.index_folder_path = index_folder_dir.clone();
        let _ = fs::create_dir_all(index_folder_dir.clone());
    
        let index_dir = index_folder_dir.join(DescIndex::DescPointIndex.to_string());
        let _ = fs::create_dir_all(index_dir);

        let index_dir = index_folder_dir.join(DescIndex::DescNameIndex.to_string());
        let _ = fs::create_dir_all(index_dir);

        let index_dir = index_folder_dir.join(DescIndex::DescLabelIndex.to_string());
        let _ = fs::create_dir_all(index_dir);

        let index_dir = index_folder_dir.join(DescIndex::DescDescIndex.to_string());
        let _ = fs::create_dir_all(index_dir);

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
    fn append_index(id: String, value: String, index: String) -> String {
        let mut line: String = id;
        line.push(' ');
        line.push_str(value.as_str());
        let mut result: String = index.clone();
        if !result.is_empty() {    
            result.push('\n');
        }
        result.push_str(line.as_str());
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
        self.config.tmp_space = Space::from(self.config.org_space.clone());
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



    fn get_descs(&self, points: Vec<&str>) -> Vec<Descriptor> {
        points.iter().map(|x|self.get_desc(x)).collect()
    }

    fn get_all_descs(&self) -> Vec<Descriptor> {

        let binding = self.get_desc_point_indexes();
        let lines = binding.lines();
        let mut descs: Vec<Descriptor> = Vec::new();

        let filenames: Vec<&str> = lines.map(|x|{x.split_once(' ').unwrap().1}).collect();
        for filename in filenames {
            descs.push(Descriptor::from(self.load_desc(filename)));
        };

        descs
   }


    fn get_descs_or_else_ids(&self, points: Vec<String>) -> Vec<Descriptor> {
        points.iter().map(|x|self.get_desc_or_id(x)).collect()
    }

    fn get_desc_or_id(&self, name: &str) -> Descriptor {
        let binding = self.get_desc_point_indexes();
        let mut lines = binding.lines();

        let point = lines.find_map(|x|{ let y = x.split_once(" ").unwrap(); if y.0 == name {return Some(y.1)}else{ return None}});

        let mut content = "".to_string();
        if point.is_some(){
            content = self.load_desc(point.unwrap());
        }
        if content.is_empty() {
            return Descriptor{
                point: name.to_string(), 
                desc_id: "".to_string(),
                description: "".to_string(),
                label: "".to_string(),
                name: "".to_string(),
            }
        }
        Descriptor::from(content)
    }


    fn get_desc(&self, name: &str) -> Descriptor {
        let binding = self.get_desc_point_indexes();
        let mut lines = binding.lines();

        let point = lines
            .find_map(|x|{ 
                let y = x.split_once(" ").unwrap(); 
                if y.0 == name { Some(y.1) }
                else{ None }
            });

        let mut content = "".to_string();
        if point.is_some(){
            content = self.load_desc(point.unwrap());
        }
        Descriptor::from(content)
    }
    

    ///
    /// Method used to persist a Descriptor. 
    ///
    fn add_desc(&self, desc: Descriptor, id: String) {
        let description = String::from(desc.clone());
        let path = self.desc_folder_path.clone();
        let _ = path.join(id);

        let _ = fs::write(path, description);
    }


    ///
    /// Takes a descriptor note as argument and creates indexes for its variables. 
    /// It is important that the descriptor note has a desc_id. 
    ///
    fn index_desc(&self, desc: Descriptor) {

        let mut point_index = self.get_desc_point_indexes();
        point_index = Self::append_index(desc.desc_id.clone(), desc.point.clone(), point_index.clone());
        self.set_desc_point_indexes(&point_index);

        let mut name_index = self.get_desc_name_indexes();
        name_index = Self::append_index(desc.desc_id.clone(), desc.name.clone(), name_index.clone());
        self.set_desc_name_indexes(&name_index);

        let mut label_index = self.get_desc_label_indexes();
        label_index = Self::append_index(desc.desc_id.clone(), desc.label.clone(), label_index.clone());
        self.set_desc_label_indexes(&label_index);

        let mut desc_index = self.get_desc_description_indexes();
        desc_index = Self::append_index(desc.desc_id.clone(), desc.description.clone(), desc_index.clone());
        self.set_desc_description_indexes(&desc_index);

    }

    ///
    /// This method returns all indexing records of descriptors in current space, based on the point field. 
    ///
    fn get_desc_point_indexes(&self) -> String {
        let filename = self.get_index_path(DescIndex::DescPointIndex);    
        fs::read_to_string(filename).expect("something wetn wrong reading the desc_point_index_file file")
    }

    fn get_desc_name_indexes(&self) -> String  {

        let filename = self.get_index_path(DescIndex::DescNameIndex);    
        fs::read_to_string(filename).expect("something wetn wrong reading the desc_name_index_file file")
    }

    fn get_desc_label_indexes(&self) -> String  {
   
        let filename = self.get_index_path(DescIndex::DescLabelIndex);    
        fs::read_to_string(filename).expect("something wetn wrong reading the desc_label_index_file file")
    }

    fn get_desc_description_indexes(&self) -> String  {
    
        let filename = self.get_index_path(DescIndex::DescDescIndex);    
        fs::read_to_string(filename).expect("something wetn wrong reading the desc_description_index_file file")
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
        fs::read_to_string(filename).expect("something wetn wrong reading the desc_point_index_file file")
    }


    fn set_desc_point_indexes(&self, lines: &str) { 
    
        file_tools::write(self.get_index_path(descriptor_facade::DescIndex::DescPointIndex), lines);
    }

    fn set_desc_name_indexes(&self, lines: &str){
        file_tools::write(self.get_index_path(descriptor_facade::DescIndex::DescNameIndex), lines);
    }

    fn set_desc_label_indexes(&self, lines: &str) { 
        file_tools::write(self.get_index_path(descriptor_facade::DescIndex::DescLabelIndex), lines);
    }

    fn set_desc_description_indexes(&self, lines:&str) {
        file_tools::write(self.get_index_path(descriptor_facade::DescIndex::DescDescIndex), lines);
    }

}

