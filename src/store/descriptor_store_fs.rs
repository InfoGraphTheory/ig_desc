
//
// Lots of logic here. The biggest refactoring needed in the future is to split space storage from
// shared storage.
// Right now the user feed a space when creating a Store instance and can from here choose to save
// in that space or in the shared space.
// It would be prettier if the you could initiate an instance for shared space use only without
// putting in an empty space (or random dummy space).
// Also it may be a source of errors that you are abel to use both individual and shared space once
// initiated. Ultimately we might want two versions of DescriptorStoreFS (DescriptorStoreFSShared
// and DescriptorStoreFSSpace).
//

use crate::Descriptor;
use std::{fs, path::Path};
use super::{descriptor_store::DescriptorStore, descriptor_facade::{DescIndex, self}};
use crate::descriptor_tools;
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
    org_space: String,
    tmp_space: Option<String>,
}

impl ::std::default::Default for DescConfig {
    fn default() -> Self {
        Self {
            app_parent_path: dirs::data_local_dir().unwrap_or_else(||PathBuf::new()),
            app_folder_name: "infospace".to_string(),
            space_folder_name: "spaces".to_string(),
            desc_folder_name: "descs".to_string(),
            index_folder_name: "indexes".to_string(),
            org_space: "default".to_string(),
            tmp_space: None,
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
    pub fn new(app_name: String, space_id: String, config_name: String) -> Self {

        let mut config: DescConfig = confy::load(config_name.as_str(), None).unwrap();
        if app_name.eq("") {
            config.app_folder_name = "infospace".to_string();    
        } else {
            config.app_folder_name = app_name.clone();    
        }
        if space_id.eq("") {
            config.org_space = "infospace".to_string();    
        } else {
            config.org_space = space_id.clone();    
        }

        let mut instance: DescriptorStoreFS = DescriptorStoreFS::default();
        Self::create_folders_if_not_there(&mut instance, space_id);

        instance
    }

    ///
    /// Called when using a new space to make sure the space that is, its folders exists.
    ///
    ///
    /// Creates the folders if not already there. The default parent folder is the default app data
    /// folder of the Operative System running the applications.
    ///    
    fn create_folders_if_not_there(instance: &mut DescriptorStoreFS, space: String) {

        let desc_config: DescConfig = instance.clone().config;

        let data_dir = desc_config.app_parent_path.clone();
        data_dir.join(desc_config.app_folder_name.clone());
        instance.app_folder_path = data_dir.clone();

        data_dir.join(desc_config.space_folder_name.clone());
        data_dir.join(space.clone());
        instance.space_folder_path = data_dir.clone();
        let _ = fs::create_dir_all(data_dir.clone());
        
        Self::create_desc_folder_in_folder(desc_config.clone(), data_dir.clone(), instance);

        Self::create_index_folder_in_folder(desc_config.clone(), data_dir.clone(), instance);
        
    }

    ///
    /// Used to create a folder for descriptors.
    ///
    fn create_desc_folder_in_folder(config: DescConfig, parent: PathBuf, instance: &mut DescriptorStoreFS) {
        
        let desc_folder_dir = parent.join(config.desc_folder_name);
        instance.desc_folder_path = desc_folder_dir.clone();
        let _ = fs::create_dir_all(desc_folder_dir);
    }

    //
    // Used to create a folder for indexes, along with sub folders for specific indexes.
    ///
    fn create_index_folder_in_folder(config: DescConfig, parent: PathBuf, instance: &mut DescriptorStoreFS) {
        
        let index_folder_dir = parent.join(config.index_folder_name.clone());
        instance.index_folder_path = index_folder_dir.clone();
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











    //nxt::todo::find out when and from where to call this fn
    pub fn create_space_files_and_dirs_if_not_there(&self, space_id: String){

        let index_folder = self.create_space_dir_if_not_there(space_id.clone(), self.index_folder_name.clone(), self.config);
        let _ = self.create_space_dir_if_not_there(space_id.clone(), self.desc_folder_name.clone(), self.config);

        Self::create_indexfile_if_not_there(DescIndex::DescPointIndex.to_string(), index_folder.to_string());
        Self::create_indexfile_if_not_there(DescIndex::DescNameIndex.to_string(), index_folder.to_string());
        Self::create_indexfile_if_not_there(DescIndex::DescLabelIndex.to_string(), index_folder.to_string());
        Self::create_indexfile_if_not_there(DescIndex::DescDescIndex.to_string(), index_folder.to_string());
    }
        
    pub fn create_space_dir_if_not_there(&self, space_id: String, sub_folder_name: String, desc_config: DescConfig) -> PathBuf {

        let data_path: PathBuf = Self::get_data_home(desc_config);
        let data_path: PathBuf = data_path.join(self.space_folder);
        let data_path: PathBuf = data_path.join(self.spaces_id);
        let data_path: PathBuf = data_path.join(self.sub_folder_name);

        let _ = fs::create_dir_all(data_path.clone());
        data_path
    }

    pub fn get_data_home(desc_config: DescConfig) -> PathBuf {

        let data_dir = desc_config.app_parent_path.clone();
        data_dir.join(desc_config.app_folder_name);
        data_dir
    }

    //TODO:: make a fn like get_data_home for all relevant paths 

    pub fn create_indexfile_if_not_there(filename: String, indexfolder: PathBuf) {
        

        let data_path: PathBuf = indexfolder.join(filename);

        if !Path::new(&data_path).is_file() {
            let _ = fs::write(data_path, "");
        }
    }

    pub fn get_index_path(&self, name: String) -> String {

        let mut path = self.index_folder.clone();
        path.push_str(name.trim());
        
        path
    }   

    pub fn get_space_index_path(&self, name: String, space_id: String) -> String {

        let mut path = self.space_folder.clone();
        if !path.ends_with('/') {
            path.push('/');
        }
        path.push_str(&space_id);
        path.push('/');
        path.push_str(&self.index_folder_name);
        path.push('/');
        path.push_str(name.trim());

        path
    }   

    pub fn get_file_path(&self, name: String) -> String {

        let mut path = self.file_folder.clone();
        path.push_str(name.trim());
        
        path
    }   

    pub fn get_space_descs_path(&self, name: String, space_id: String) -> String {

        let mut path = self.space_folder.clone();
        if !path.ends_with('/') {
            path.push('/');
        }
        path.push_str(&space_id);
        path.push('/');
        path.push_str(&self.desc_folder_name);
        path.push('/');
        path.push_str(name.trim());
        
        path
    }   

//TODO: For better search in the future we want to be able to search indexes. Ultimately through a
//cache. 
    pub fn desc_to_index_files(desc: Descriptor) {
        let line = String::new();    
        let id = descriptor_tools::get_desc_id(&desc);
    
        Self::add_to_desc_point_index(&id, desc);
    //add for each index file.
    //we probably want to generalize so that we don't go directly to and from files, but just to
    //and from index methods which then in turn decides if ram, files or db is to be used.
    //But, keep file operations for buffer seperated from descriptors and triples. 
    }

    fn add_to_desc_point_index(id: &str, desc: Descriptor) {
//storage::get_desc_point_indexes

//desc_tools::create_desc_point_index_line

//list_tools::add

//storage::update_desc_point_indexes    
    }
}


impl DescriptorStore for DescriptorStoreFS {

    // Following methods is for making it possible to change space temporary along the way.
    //dynamic space handling begin
    fn set_tmp_space_id(&mut self, space_id: String) {
       self.tmp_space = Option::Some(space_id);
    }

    fn revert_space_id(&mut self) {
        self.tmp_space = Option::Some(self.org_space.clone());
    }

    fn get_space_id(&mut self) -> String {
        if self.tmp_space.is_none() || self.org_space == self.tmp_space.clone().unwrap_or("".to_string()) {
            return self.org_space.clone();
        }
        self.tmp_space.clone().unwrap()
    }
    //dynamic space handling end

    fn get_descs(&self, points: Vec<&str>) -> Vec<Descriptor> {
        points.iter().map(|x|self.get_desc(x)).collect()
    }

    fn get_all_descs(&self) -> Vec<Descriptor> {

        let binding = self.get_desc_point_indexes();
        let lines = binding.lines();
        let mut descs: Vec<Descriptor> = Vec::new();

        let filenames: Vec<&str> = lines.map(|x|{x.split_once(' ').unwrap().1}).collect();
        for filename in filenames {

            let path_name  = self.get_file_path(filename.to_string());
            let content = fs::read_to_string(path_name).unwrap_or("".to_string());
            descs.push(Descriptor::from(content));
        };

        descs
   }

    fn get_descs_or_else_ids(&self, points: Vec<String>) -> Vec<Descriptor> {
        points.iter().map(|x|self.get_desc_or_id(x)).collect()
    }

    fn get_space_descs_or_else_ids(&self, points: Vec<String>, space_id: String) -> Vec<Descriptor> {
        points
            .iter()
            .map(|x|
                self.get_space_desc_or_id(x, space_id.clone())
            )
            .collect()
    }

//Consider if name is the right name for the argument. Also consider if the method "maybe" could be split
//up so the facade part figures out the details in a cross medium generalized way. 
    fn get_desc_or_id(&self, name: &str) -> Descriptor {
        let binding = self.get_desc_point_indexes();
        let mut lines = binding.lines();

        let point = lines.find_map(|x|{ let y = x.split_once(" ").unwrap(); if y.0 == name {return Some(y.1)}else{ return None}});

        let mut content = "".to_string();
        if point.is_some(){
        let path_name  = self.get_file_path(point.unwrap().to_string());
            content = fs::read_to_string(path_name).unwrap_or("".to_string());
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

    fn get_space_desc_or_id(&self, name: &str, space_id: String) -> Descriptor {

        let binding = self.get_space_desc_point_indexes(space_id.clone());
        let mut lines = binding.lines();
//println!("LINES: {:?}", lines.clone());

        let point = lines
            .find_map(|x| { 
                let y = x.split_once(" ").unwrap(); 
                if y.0 == name { Some(y.1) }
                else { None }
            });

        let mut content = "".to_string();
        if point.is_some() {
            let path_name = self.get_space_descs_path(point.unwrap().to_string(), space_id.clone());
            content = fs::read_to_string(path_name.clone()).unwrap_or("".to_string());
//println!("content {:?} found for path {:?}", content.clone(), path_name.clone());
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
            let path_name  = self.get_file_path(point.unwrap().to_string());
            content = fs::read_to_string(path_name).unwrap_or("".to_string());
        }
        Descriptor::from(content)
    }
    
    fn add_desc(&self, desc: Descriptor, id: String) {
        let description = String::from(desc.clone());
        let path = self.get_file_path(id);

        let _ = fs::write(path, description);
    }


    fn get_desc_point_indexes(&self) -> String {
        let filename = DescIndex::DescPointIndex.to_string();
        let filename = self.get_index_path(filename);    
        fs::read_to_string(filename).expect("something wetn wrong reading the desc_point_index_file file")
    }

    fn get_space_desc_point_indexes(&self, space_id: String) -> String {
        self.create_space_files_and_dirs_if_not_there(space_id.clone());

        let filename = DescIndex::DescPointIndex.to_string();
        let filename = self.get_space_index_path(filename, space_id);    
//println!("Reading point indexes from {}", filename);
        fs::read_to_string(filename).expect("something wetn wrong reading the desc_point_index_file file")
    }

    fn get_desc_name_indexes(&self) -> String  {
        let filename = DescIndex::DescNameIndex.to_string();
        let filename = self.get_index_path(filename);    
        fs::read_to_string(filename).expect("something wetn wrong reading the desc_name_index_file file")
    }

    fn get_desc_label_indexes(&self) -> String  {
        let filename = DescIndex::DescLabelIndex.to_string();
        let filename = self.get_index_path(filename);    
        fs::read_to_string(filename).expect("something wetn wrong reading the desc_label_index_file file")
    }

    fn get_desc_description_indexes(&self) -> String  {
        let filename = DescIndex::DescDescIndex.to_string();
        let filename = self.get_index_path(filename);    
        fs::read_to_string(filename).expect("something wetn wrong reading the desc_label_index_file file")
    }


    fn set_desc_point_indexes(&self, lines: &str) { 
    
        file_tools::write(self.get_index_path(descriptor_facade::DescIndex::DescPointIndex.to_string()), lines);
    }

    fn set_desc_name_indexes(&self, lines: &str){
        file_tools::write(self.get_index_path(descriptor_facade::DescIndex::DescNameIndex.to_string()), lines);
    }

    fn set_desc_label_indexes(&self, lines: &str) { 
        file_tools::write(self.get_index_path(descriptor_facade::DescIndex::DescLabelIndex.to_string()), lines);
    }

    fn set_desc_description_indexes(&self, lines:&str) {
        file_tools::write(self.get_index_path(descriptor_facade::DescIndex::DescDescIndex.to_string()), lines);
    }

}


