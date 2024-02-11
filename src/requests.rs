use serde::Deserialize;

#[derive(Deserialize)]
pub struct SearchTermRequest {
    pub search_term: String,
}
