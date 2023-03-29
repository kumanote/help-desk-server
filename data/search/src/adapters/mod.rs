mod health_check;
pub use health_check::*;

pub mod faq_item;
pub mod inquiry_contact;
pub mod inquiry_message;
pub mod public_faq_item;

use crate::entities::SearchResults;

fn create_empty_results<T>() -> SearchResults<T> {
    SearchResults {
        hits: vec![],
        offset: None,
        limit: None,
        estimated_total_hits: None,
        page: None,
        hits_per_page: None,
        total_hits: None,
        total_pages: None,
        facet_distribution: None,
        processing_time_ms: 0,
        query: String::default(),
    }
}
