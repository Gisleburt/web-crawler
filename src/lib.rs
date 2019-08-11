mod url_summary;

pub use crate::url_summary::UrlSummary;
use reqwest::get;
use scraper::{Html, Selector};
use std::error::Error;

pub fn crawl<T>(url: T) -> Result<UrlSummary, Box<dyn Error>>
where
    T: AsRef<str>,
{
    // Download the html document
    let body = get(url.as_ref())?.text()?;

    // Parse the document with scraper
    let html = Html::parse_document(&body);

    // Get all the anchor tags
    let anchor = Selector::parse("a").unwrap();

    // prep the summary
    let mut summary = UrlSummary::new();

    for a in html.select(&anchor) {
        if let Some(href) = a.value().attr("href") {
            summary.add(href);
        }
    }

    Ok(summary)
}
