use serde::{Deserialize, Serialize};

/// Query parameters for paginated endpoints.
/// Usage: `Query(params): Query<PaginationParams>`
#[derive(Debug, Clone, Deserialize)]
pub struct PaginationParams {
    /// 1-based page number (defaults to 1)
    #[serde(default = "default_page")]
    pub page: u32,
    /// Items per page (defaults to 20, max 100)
    #[serde(default = "default_per_page")]
    pub per_page: u32,
}

fn default_page() -> u32 { 1 }
fn default_per_page() -> u32 { 20 }

impl PaginationParams {
    /// SQL LIMIT value (capped at 100)
    pub fn limit(&self) -> i64 {
        self.per_page.min(100) as i64
    }

    /// SQL OFFSET value
    pub fn offset(&self) -> i64 {
        ((self.page.saturating_sub(1)) * self.per_page.min(100)) as i64
    }
}

/// Generic paginated response envelope.
#[derive(Debug, Clone, Serialize)]
pub struct PaginatedResponse<T: Serialize> {
    pub items: Vec<T>,
    pub total: i64,
    pub page: u32,
    pub per_page: u32,
    pub total_pages: u32,
}

impl<T: Serialize> PaginatedResponse<T> {
    pub fn new(items: Vec<T>, total: i64, params: &PaginationParams) -> Self {
        let per_page = params.per_page.min(100);
        let total_pages = ((total as u32).saturating_add(per_page - 1)) / per_page;
        Self {
            items,
            total,
            page: params.page,
            per_page,
            total_pages,
        }
    }
}
