//! FTMS — File/Text Management System
//!
//! Handles file upload, storage, text extraction, AI description,
//! and full-text search indexing.

pub mod schema;
pub mod storage;

pub use schema::{FileRecord, FileMetadata};
