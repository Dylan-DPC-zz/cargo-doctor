use failure::Error;
use indexmap::IndexSet;
use linker::{LinksList, LocalScraper, RemoteScraper};
use reqwest::Url;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct Handler {
    base_url: String,
}

impl Handler {
    pub fn new(base_url: &str) -> Handler {
        Handler {
            base_url: base_url.to_owned(),
        }
    }

    pub fn remote_scrape(&self) -> Result<(), Error> {
        let base_url = Url::parse(self.base_url.as_str())?;
        let mut scraper = RemoteScraper::new(&base_url)
            .scrape()
            .expect("base url not found");

        scraper = scraper
            .iter()
            .map(|link| {
                base_url
                    .join(link)
                    .expect("cannot construct links from parent url")
                    .as_str()
                    .to_owned()
            }).collect::<IndexSet<String>>();

        let mut broken_links = vec![];

        let mut parent_url = base_url;
        let mut visited_urls = vec![];
        while let Some(link) = scraper.pop() {
            visited_urls.push(link.clone());
            let url = parent_url.join(&link).expect("cannot join links");
            if let Some(u) = url.parent() {
                parent_url = Url::parse(&u)?;
            }
            match RemoteScraper::new(&url).scrape() {
                Ok(contents) => {
                    let links = contents
                        .iter()
                        .filter(|item| !link.contains(item.as_str()))
                        .map(|item| {
                            parent_url
                                .join(item)
                                .expect("cannot construct links from parent url")
                                .as_str()
                                .to_string()
                        }).filter(|item| !visited_urls.contains(item))
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
        let mut scraper = LocalScraper::new(&index_url)
            .scrape()
            .expect("base url not found");

        scraper = scraper
            .iter()
            .map(|link| {
                base_url
                    .join(link)
                    .to_str()
                    .expect("cannot construct links from parent path")
                    .to_owned()
            }).collect::<IndexSet<String>>();
        let mut broken_links = vec![];
        let mut parent_path = base_url;
        let mut visited_urls = vec![];
        while let Some(link) = scraper.pop() {
            visited_urls.push(link.clone());
            let url = parent_path.join(&link);
            if let Some(u) = url.parent() {
                parent_path = PathBuf::from(&u);
            }
            match LocalScraper::new(&url).scrape() {
                Ok(contents) => {
                    let links = contents
                        .iter()
                        .filter(|item| !link.contains(item.as_str()))
                        .map(|link| {
                            parent_path
                                .join(link)
                                .to_str()
                                .expect("cannot join link")
                                .to_string()
                        }).filter(|item| !visited_urls.contains(item))
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
}

#[derive(Debug, Fail)]
#[fail(display = "broken links: {}", 0)]
pub struct BrokenLinks(pub LinksList);

impl BrokenLinks {
    fn new(links: &[String]) -> BrokenLinks {
        BrokenLinks(LinksList(links.to_vec()))
    }
}

pub trait Parent {
    fn parent(&self) -> Option<String>;
}

impl Parent for Url {
    fn parent(&self) -> Option<String> {
        let input = self.as_str();
        let length = if input.ends_with('/') { 3usize } else { 1usize };
        input
            .rsplitn(length, '/')
            .into_iter()
            .map(|x| x.to_string())
            .last()
    }
}
