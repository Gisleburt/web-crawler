use serde::Serialize;
use std::collections::hash_map::IntoIter;
use std::collections::HashMap;

/// Our Urls are stored in a newtype structure to provide a simple API around a HashMap
#[derive(Debug, Serialize)]
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
    where
        T: AsRef<str>,
    {
        let url = url.as_ref();
        match self.0.get_mut(url) {
            Some(count) => {
                *count = *count + 1;
            }
            None => {
                self.0.insert(url.to_string(), 1);
            }
        }
    }

    /// Returns the current count of a url in the page.
    pub fn get_count<T>(&mut self, url: T) -> u64
    where
        T: AsRef<str>,
    {
        let url = url.as_ref();
        self.0.get(url).map(|count| count.to_owned()).unwrap_or(0)
    }

    pub fn contains<T>(&self, url: T) -> bool
    where
        T: AsRef<str>,
    {
        self.0.contains_key(url.as_ref())
    }
}

/// We can return a struct to make the iterator nice and easy to use
pub struct UrlSummaryItem {
    pub url: String,
    pub count: u64,
}

/// This structure allows us to wrap the iterator types we get from HashMap so we can provide our
/// own interface
pub struct UrlSummaryIterator(IntoIter<String, u64>);

/// Implement Iterator for our Iterator type
impl Iterator for UrlSummaryIterator {
    type Item = UrlSummaryItem;

    fn next(&mut self) -> Option<Self::Item> {
        // Map the tuple into a more ergonomic struct
        self.0.next().map(|v| Self::Item {
            url: v.0.to_owned(),
            count: v.1,
        })
    }
}

/// Implement IntoIterator for UrlSummary
/// ```
/// use web_crawler::UrlSummary;
///
/// let mut summary = UrlSummary::new();
/// summary.add("https://example.com");
/// summary.add("https://danielmason.com");
/// summary.add("https://example.com");
/// for item in summary.into_iter() {
///   println!("Url: {}, count: {}", item.url, item.count)
/// }
/// ```
impl IntoIterator for UrlSummary {
    type Item = UrlSummaryItem;
    type IntoIter = UrlSummaryIterator;

    fn into_iter(self) -> Self::IntoIter {
        UrlSummaryIterator(self.0.into_iter())
    }
}
