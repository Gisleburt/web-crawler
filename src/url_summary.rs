use std::collections::HashMap;

/// Our Urls are stored in a newtype structure to provide a simple API around a HashMap
pub struct UrlSummary(HashMap<String, u64>);

/// The default value for a UrlSummary should contain no entries. This method is used by
/// `UrlSummary::new()`
impl Default for UrlSummary {
    fn default() -> Self {
        UrlSummary(HashMap::new())
    }
}

/// An interface for tracking uses of urls
///
/// ```
/// use web_crawler::UrlSummary;
///
/// let mut summary = UrlSummary::new();
/// assert_eq!(summary.get_count("https://example.com"), 0);
/// summary.add("https://example.com");
/// assert_eq!(summary.get_count("https://example.com"), 1);
/// summary.add("https://example.com");
/// assert_eq!(summary.get_count("https://example.com"), 2);
/// ```
impl UrlSummary {
    /// Create an empty summary object
    pub fn new() -> UrlSummary {
        UrlSummary::default()
    }

    /// Adds a url to the summary. If the url already exists it will increase the count.
    pub fn add<T>(&mut self, url: T)
        where T: AsRef<str> {
        let url = url.as_ref();
        match self.0.get_mut(url) {
            Some(count) => { *count = *count + 1; },
            None => { self.0.insert(url.to_string(), 1); },
        }
    }

    /// Returns the current count of a url in the page.
    pub fn get_count<T>(&mut self, url: T) -> u64
        where T: AsRef<str> {
        let url = url.as_ref();
        self.0.get(url).map(|count| count.to_owned()).unwrap_or(0)
    }
}
