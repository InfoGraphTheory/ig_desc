//! An implementation of descriptor notes.
//!
//! A [`Descriptor`] is a small note with a `point`, and optional `name`, `label` and
//! `description` fields. Its `desc_id` — a hash of those fields, assigned once persisted —
//! is the descriptor's real unique identifier; multiple descriptors may share the same
//! `point`. This crate provides the model ([`Descriptor`] and its field newtypes), a general
//! [`descriptor_store`] trait for persisting and querying descriptors, with a filesystem-based
//! reference implementation ([`descriptor_store_fs`]), and a facade/service layer
//! ([`descriptor_facade`], [`desc_service_fs`]) for creating, indexing and retrieving
//! descriptors.

mod logic;
mod misc;
mod model;
mod service;
mod store;

pub use logic::desc_director::DescDirector;
pub use misc::descriptor_tools;
pub use model::descriptor::{Descriptor, DescId, Point, Name, Label, Description};
pub use model::space::Space;
pub use model::app::App;
pub use service::desc_service_fs;
pub use store::descriptor_facade;
pub use store::descriptor_store;
pub use store::descriptor_store_fs;

