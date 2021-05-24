//!
//! Various model types that are not part of the GraphQL schema
//!

use std::ops::Range;

#[derive(Debug)]
pub struct TodoFilter {
    pub ids: Option<Vec<uuid::Uuid>>,
    pub range: Range<u32>,
}
