use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

impl Default for PaginationParams {
    fn default() -> Self {
        Self {
            page: Some(1),
            limit: Some(20),
        }
    }
}

impl PaginationParams {
    pub fn page(&self) -> u32 {
        self.page.unwrap_or(1)
    }

    pub fn limit(&self) -> u32 {
        self.limit.unwrap_or(20)
    }

    pub fn offset(&self) -> u32 {
        (self.page() - 1) * self.limit()
    }
}

pub fn generate_pagination_html(
    current_page: u32,
    limit: u32,
    total_items: usize,
    base_url: &str,
    query_params: &str,
) -> String {
    let total_pages = (total_items as f64 / limit as f64).ceil() as u32;

    if total_pages <= 1 {
        return String::new();
    }

    let mut pagination_html = String::new();

    // Previous page
    if current_page > 1 {
        let prev_page = current_page - 1;
        let prev_url = if query_params.is_empty() {
            format!("{base_url}?page={prev_page}")
        } else {
            format!("{base_url}?{query_params}&page={prev_page}")
        };
        pagination_html.push_str(&format!(
            r#"<a href="{prev_url}" class="pagination-link">Previous</a>"#
        ));
    }

    // Page numbers
    let start_page = if current_page > 3 {
        current_page - 2
    } else {
        1
    };
    let end_page = if current_page + 2 <= total_pages {
        current_page + 2
    } else {
        total_pages
    };

    // First page if not in range
    if start_page > 1 {
        let first_url = if query_params.is_empty() {
            format!("{base_url}?page=1")
        } else {
            format!("{base_url}?{query_params}&page=1")
        };
        pagination_html.push_str(&format!(
            r#"<a href="{first_url}" class="pagination-link">1</a>"#
        ));
        if start_page > 2 {
            pagination_html.push_str(r#"<span class="pagination-ellipsis">...</span>"#);
        }
    }

    // Page numbers in range
    for page in start_page..=end_page {
        if page == current_page {
            pagination_html.push_str(&format!(
                r#"<span class="pagination-current">{page}</span>"#
            ));
        } else {
            let page_url = if query_params.is_empty() {
                format!("{base_url}?page={page}")
            } else {
                format!("{base_url}?{query_params}&page={page}")
            };
            pagination_html.push_str(&format!(
                r#"<a href="{page_url}" class="pagination-link">{page}</a>"#
            ));
        }
    }

    // Last page if not in range
    if end_page < total_pages {
        if end_page < total_pages - 1 {
            pagination_html.push_str(r#"<span class="pagination-ellipsis">...</span>"#);
        }
        let last_url = if query_params.is_empty() {
            format!("{base_url}?page={total_pages}")
        } else {
            format!("{base_url}?{query_params}&page={total_pages}")
        };
        pagination_html.push_str(&format!(
            r#"<a href="{last_url}" class="pagination-link">{total_pages}</a>"#
        ));
    }

    // Next page
    if current_page < total_pages {
        let next_page = current_page + 1;
        let next_url = if query_params.is_empty() {
            format!("{base_url}?page={next_page}")
        } else {
            format!("{base_url}?{query_params}&page={next_page}")
        };
        pagination_html.push_str(&format!(
            r#"<a href="{next_url}" class="pagination-link">Next</a>"#
        ));
    }

    pagination_html
}

pub fn get_pagination_css() -> &'static str {
    r#"
    .pagination {
        margin-top: 20px;
        text-align: center;
        display: flex;
        justify-content: center;
        align-items: center;
        gap: 4px;
        flex-wrap: wrap;
    }

    .pagination-link {
        padding: 8px 12px;
        text-decoration: none;
        border: 1px solid #dee2e6;
        border-radius: 4px;
        color: #007bff;
        background-color: #fff;
        transition: all 0.2s ease;
        min-width: 40px;
        text-align: center;
        display: inline-block;
    }

    .pagination-link:hover {
        background-color: #e9ecef;
        border-color: #adb5bd;
        color: #0056b3;
    }

    .pagination-current {
        padding: 8px 12px;
        background-color: #007bff;
        color: white;
        border: 1px solid #007bff;
        border-radius: 4px;
        min-width: 40px;
        text-align: center;
        display: inline-block;
        font-weight: bold;
    }

    .pagination-ellipsis {
        padding: 8px 4px;
        color: #6c757d;
        font-weight: bold;
    }

    .pagination-info {
        margin-bottom: 15px;
        color: #6c757d;
        font-size: 14px;
        text-align: center;
    }

    .pagination-controls {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 20px;
        flex-wrap: wrap;
        gap: 10px;
    }

    .pagination-controls .form-group {
        margin: 0;
        display: flex;
        align-items: center;
        gap: 8px;
    }

    .pagination-controls label {
        margin: 0;
        font-size: 14px;
        color: #495057;
    }

    .pagination-controls select {
        padding: 4px 8px;
        border: 1px solid #ced4da;
        border-radius: 4px;
        font-size: 14px;
    }

    .pagination-controls button {
        padding: 6px 12px;
        background-color: #007bff;
        color: white;
        border: none;
        border-radius: 4px;
        cursor: pointer;
        font-size: 14px;
    }

    .pagination-controls button:hover {
        background-color: #0056b3;
    }

    @media (max-width: 768px) {
        .pagination {
            gap: 2px;
        }

        .pagination-link,
        .pagination-current {
            padding: 6px 8px;
            min-width: 32px;
            font-size: 12px;
        }

        .pagination-controls {
            flex-direction: column;
            align-items: stretch;
        }
    }
    "#
}

pub fn generate_pagination_info(current_page: u32, limit: u32, total_items: usize) -> String {
    let start_item = (current_page - 1) * limit + 1;
    let end_item = std::cmp::min(current_page * limit, total_items as u32);

    format!(
        "Showing {} to {} of {} entries (Page {} of {})",
        start_item,
        end_item,
        total_items,
        current_page,
        (total_items as f64 / limit as f64).ceil() as u32
    )
}

pub fn generate_pagination_controls(current_page: u32, limit: u32, total_items: usize) -> String {
    let total_pages = (total_items as f64 / limit as f64).ceil() as u32;

    format!(
        r#"
        <div class="pagination-controls">
            <div class="form-group">
                <label for="limit">Items per page:</label>
                <select id="limit" onchange="changeLimit(this.value)">
                    <option value="10" {}>10</option>
                    <option value="20" {}>20</option>
                    <option value="50" {}>50</option>
                    <option value="100" {}>100</option>
                </select>
            </div>

            <div class="form-group">
                <label for="page">Go to page:</label>
                <select id="page" onchange="goToPage(this.value)">
                    {}
                </select>
            </div>
        </div>

        <script>
            function changeLimit(newLimit) {{
                const url = new URL(window.location);
                url.searchParams.set('limit', newLimit);
                url.searchParams.set('page', '1');
                window.location.href = url.toString();
            }}

            function goToPage(page) {{
                const url = new URL(window.location);
                url.searchParams.set('page', page);
                window.location.href = url.toString();
            }}
        </script>
        "#,
        if limit == 10 { "selected" } else { "" },
        if limit == 20 { "selected" } else { "" },
        if limit == 50 { "selected" } else { "" },
        if limit == 100 { "selected" } else { "" },
        generate_page_options(current_page, total_pages)
    )
}

fn generate_page_options(current_page: u32, total_pages: u32) -> String {
    let mut options = String::new();

    for page in 1..=total_pages {
        let selected = if page == current_page { "selected" } else { "" };
        options.push_str(&format!(
            r#"<option value="{page}" {selected}>Page {page}</option>"#
        ));
    }

    options
}
