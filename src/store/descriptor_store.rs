use crate::Descriptor;

pub trait DescriptorStore {

fn get_desc(&self, name: &str) -> Descriptor;    

fn get_descs(&self, points: Vec<&str>) -> Vec<Descriptor>;

fn get_desc_or_id(&self, name: &str) -> Descriptor;

fn get_descs_or_else_ids(&self, points: Vec<String>) -> Vec<Descriptor>;

fn get_all_descs(&self) -> Vec<Descriptor>;

fn add_desc(&self, desc: Descriptor, id: String);

fn get_desc_point_indexes(&self) -> String;

fn get_tmp_space_desc_point_indexes(&mut self, space_id: String) -> String;

fn get_desc_name_indexes(&self) -> String;

fn get_desc_label_indexes(&self) -> String;

fn get_desc_description_indexes(&self) -> String;


fn set_desc_point_indexes(&self, indexes: &str);

fn set_desc_name_indexes(&self, indexes: &str);

fn set_desc_label_indexes(&self, indexes: &str);

fn set_desc_description_indexes(&self, indexes:&str);


fn set_tmp_space_id(&mut self, space_id: String);

fn revert_space_id(&mut self);

fn get_space_id(&mut self) -> String;

///
/// Root function for adding indexes for a Descriptor.
///
fn index_desc(&self, desc: Descriptor);
}

