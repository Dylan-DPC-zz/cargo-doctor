use linker::{RemoteScraper, LocalScraper, LinksList};
use failure::Error;
use reqwest::Url;
use std::str::FromStr;
use std::ops::{Deref, DerefMut};
use std::path::{PathBuf, Path};
use indexmap::IndexSet;

#[derive(Clone, Debug)]
pub struct Handler {
    base_url: String,
}

impl Handler {
    pub fn new(base_url: &str) -> Handler {
        Handler { base_url: base_url.to_owned() }
    }

    pub fn remote_scrape(&self) -> Result<(), Error> {
        let base_url = Url::parse(self.base_url.as_str())?;
        let mut scraper = RemoteScraper::new(&base_url).scrape().expect("base url not found");
        scraper = scraper.iter().map(|link| base_url
            .join(link)
            .expect("cannot construct links from parent url").as_str().to_owned())
            .collect::<IndexSet<String>>();

        let mut broken_links = vec![];

        let mut parent_url = base_url;
        while let Some(link) = scraper.pop() {
            let url = parent_url.join(&link).expect("cannot join links");
            if let Some(u) = url.parent() {
                parent_url = Url::parse(&u)?;
            }
            match RemoteScraper::new(&url).scrape() {
                Ok(contents) => {
                    let links = contents.iter()
                        .map(|link| parent_url
                            .join(link)
                        .expect("cannot construct links from parent url").as_str().to_string())
                        .collect::<IndexSet<String>>();
                    scraper.extend(links)
                }
                Err(_) => broken_links.push(link),
            }
        }


            if broken_links.is_empty() {
                Ok(())
            } else {
                Err(Error::from(BrokenLinks::new(&broken_links)))
            }

    }

    pub fn local_scrape(&self) -> Result<(), Error> {
        let base_url = PathBuf::from(self.base_url.as_str());
        let index_url = base_url.join("index.html");
        let mut scraper = LocalScraper::new(&index_url).scrape().expect("base url not found");
        let mut broken_links = vec![];

        while let Some(link) = scraper.pop() {
            let url = base_url.join(&link);
            match LocalScraper::new(&url).scrape() {
                Ok(contents) => scraper.extend(contents),
                Err(_) => broken_links.push(link),
            }
        }

            if broken_links.is_empty() {
                Ok(())
            } else {
                Err(Error::from(BrokenLinks::new(&broken_links)))
            }

    }
}

#[derive(Debug, Fail)]
#[fail(display = "link is broken or doesn't exist")]
pub struct BrokenLinks ( pub Vec<String>);

impl BrokenLinks {
    fn new(links: &[String]) -> BrokenLinks {
        BrokenLinks(links.to_vec())
    }
}

pub trait UrlParent {
    fn parent(&self) -> Option<String>;
}

impl UrlParent for Url {
    fn parent(&self) -> Option<String> {
        let input = self.as_str();
        let length = if input.ends_with('/') {
            3usize
        } else {
            1usize
        };
        input.rsplitn(length, '/').into_iter().map(|x| x.to_string()).last()
    }
}

#[cfg(test)]
mod tests {
    use reqwest::Url;
    use super::UrlParent;

    #[test]
    fn test_scrape() {
        let url = Url::parse("https://httpbin.org/anything/foo/").unwrap();
        let parent = url.parent();

        assert_eq!(parent, Some("https://httpbin.org/anything".to_owned()));
    }
}