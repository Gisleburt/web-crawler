mod url_summary;

pub use crate::url_summary::UrlSummary;

pub fn crawl<T>(url: T) -> UrlSummary
    where T: AsRef<str> {
    UrlSummary::new()
}
