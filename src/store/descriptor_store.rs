use crate::Descriptor;

/// A general interface for persisting and querying descriptors and their field indexes.
/// `DescriptorStoreFS` is one filesystem-based reference implementation; other storage
/// backends can implement this trait as well.
pub trait DescriptorStore {

/// Returns the Descriptor stored at the given point.
fn get_desc(&self, name: &str) -> Descriptor;

/// Returns the Descriptor stored for each of the given points, in order.
fn get_descs(&self, points: Vec<&str>) -> Vec<Descriptor>;

/// Returns the Descriptor stored at the given point, falling back to a point-only Descriptor
/// if not found.
fn get_desc_or_id(&self, name: &str) -> Descriptor;

/// Returns the Descriptor stored for each of the given points, in order, falling back to a
/// point-only Descriptor for any point that isn't found.
fn get_descs_or_else_ids(&self, points: Vec<String>) -> Vec<Descriptor>;

/// Returns all stored Descriptors.
fn get_all_descs(&self) -> Vec<Descriptor>;

/// Persists a Descriptor under the given id.
fn add_desc(&self, desc: Descriptor, id: String);

/// Returns all indexing records of descriptors in the current space, based on the point field.
fn get_desc_point_indexes(&self) -> String;

/// Returns the point-field indexing records for the given space id, temporarily switching to
/// it and reverting back afterwards.
fn get_tmp_space_desc_point_indexes(&mut self, space_id: String) -> String;

/// Returns all indexing records of descriptors in the current space, based on the name field.
fn get_desc_name_indexes(&self) -> String;

/// Returns all indexing records of descriptors in the current space, based on the label field.
fn get_desc_label_indexes(&self) -> String;

/// Returns all indexing records of descriptors in the current space, based on the description field.
fn get_desc_description_indexes(&self) -> String;


/// Overwrites the point index with the given contents.
fn set_desc_point_indexes(&self, indexes: &str);

/// Overwrites the name index with the given contents.
fn set_desc_name_indexes(&self, indexes: &str);

/// Overwrites the label index with the given contents.
fn set_desc_label_indexes(&self, indexes: &str);

/// Overwrites the description index with the given contents.
fn set_desc_description_indexes(&self, indexes:&str);


/// Sets a temporary space id. May be useful for smaller operations as a new store instance
/// does not have to be made.
fn set_tmp_space_id(&mut self, space_id: String);

/// After using this instance with a temporary space id, reverts back to the original space id.
fn revert_space_id(&mut self);

/// Returns the space id currently in use — the original one, or a temporary one set via
/// `set_tmp_space_id`.
fn get_space_id(&mut self) -> String;

///
/// Root function for adding indexes for a Descriptor.
///
fn index_desc(&self, desc: Descriptor);
}

