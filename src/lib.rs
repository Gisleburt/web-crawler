mod url_summary;

pub use crate::url_summary::UrlSummary;
use reqwest::{get, Url};
use scraper::{Html, Selector};
use std::error::Error;

pub fn crawl<T>(url: T) -> Result<UrlSummary, Box<dyn Error>>
where
    T: AsRef<str>,
{
    let initial_url = Url::parse(url.as_ref())?;
    let mut url_summary = UrlSummary::new();
    crawl_page(initial_url, url_summary)
}

fn crawl_page(url: Url, mut summary: UrlSummary) -> Result<UrlSummary, Box<dyn Error>> {
    // Download the html document
    let body = get(url)?.text()?;

    // Parse the document with scraper
    let html = Html::parse_document(&body);

    // Get all the anchor tags
    let anchor = Selector::parse("a").unwrap(); // Should be safe, we know a is ok

    for a in html.select(&anchor) {
        if let Some(href) = a.value().attr("href") {
            // Lets make sure the url is real
            if let Ok(next_url) = Url::parse(href) {
                // Have we come across the url before, if we have we should crawl it
                let new = summary.contains(&next_url);
                // Lets add the url to make sure we don't create a loop
                summary.add(&next_url);
            }
        }
    }

    Ok(summary)
}
