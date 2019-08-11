mod url_summary;

pub use crate::url_summary::UrlSummary;
use reqwest::{get, Url, UrlError};
use scraper::{Html, Selector};

// Helper function for comparing domains.
fn is_url_on_host<T>(url: &Url, host: T) -> bool
where
    T: AsRef<str>,
{
    !url.has_host() || url.host_str().unwrap() == host.as_ref()
}

pub fn crawl<T>(url: T) -> Result<UrlSummary, UrlError>
where
    T: AsRef<str>,
{
    let initial_url = Url::parse(url.as_ref())?;
    let mut summary = UrlSummary::new();
    crawl_page(
        initial_url.clone().host_str().unwrap(),
        initial_url,
        &mut summary,
    );
    Ok(summary)
}

fn crawl_page<T>(host: T, url: Url, summary: &mut UrlSummary)
where
    T: AsRef<str>,
{
    // Download the html document
    let result = get(url.clone())
        .and_then(|mut response| response.text());
    if let Err(e) = result {
        dbg!(e);
        return;
    }
    if let Ok(body) = result {

        // Parse the document with scraper
        let html = Html::parse_document(&body);

        // Get all the anchor tags
        let anchor = Selector::parse("a").unwrap(); // Should be safe, we know a is ok

        html.select(&anchor)
            .into_iter()
            .filter_map(|a| a.value().attr("href"))
            .filter_map(|href| url.join(href).ok()) // Parse with join in case its a relative url
            .for_each(|next_url| {
                // Have we come across the url before, if we have we should crawl it
                let new = summary.contains(&next_url);
                // Lets add the url to make sure we don't create a loop
                summary.add(&next_url);
                if new && is_url_on_host(&next_url, host.as_ref()) {
                     crawl_page(host.as_ref(), next_url, summary);
                }
            });
    }
}

#[cfg(test)]
mod tests {
    use super::is_url_on_host;
    use reqwest::Url;

    #[test]
    fn test_is_url_on_host() {
        let url = Url::parse("https://example.com/on-example").unwrap();
        assert!(is_url_on_host(&url, "example.com"));
        let url = Url::parse("https://danielmason.com/not-on-example").unwrap();
        assert!(!is_url_on_host(&url, "example.com"));
    }
}
