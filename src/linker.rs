use failure::Error;
use indexmap::IndexSet;
use reqwest::{get, Url};
use scraper::{Html, Selector};
use std::fmt::{self, Display, Formatter};
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::ops::{Deref, DerefMut};
use std::path::{Path, PathBuf};
use std::vec::IntoIter;

#[derive(Clone, Debug)]
pub struct RemoteScraper {
    pub path: Url,
}

impl RemoteScraper {
    pub fn new(path: &Url) -> RemoteScraper {
        RemoteScraper {
            path: path.to_owned(),
        }
    }

    pub fn from(path: &str) -> Result<RemoteScraper, Error> {
        Ok(RemoteScraper {
            path: Url::parse(path)?,
        })
    }

    pub fn scrape(self) -> Result<IndexSet<String>, Error> {
        let body = get(self.path.as_str())?.text()?;
        extract_links_from_document(&body)
    }
}

#[derive(Clone, Debug)]
pub struct LocalScraper {
    pub path: PathBuf,
}

impl LocalScraper {
    pub fn new(path: &Path) -> LocalScraper {
        LocalScraper {
            path: path.to_owned(),
        }
    }

    pub fn from(path: &str) -> LocalScraper {
        LocalScraper {
            path: PathBuf::from(path),
        }
    }

    pub fn scrape(self) -> Result<IndexSet<String>, Error> {
        let body = self.text_from_local_path()?;
        extract_links_from_document(&body)
    }

    fn text_from_local_path(&self) -> Result<String, Error> {
        match File::open(&self.path) {
            Ok(f) => {
                let mut buf_reader = BufReader::new(f);
                let mut contents = String::new();
                buf_reader
                    .read_to_string(&mut contents)
                    .expect("cannot read from file");

                Ok(contents)
            }
            Err(_) => Err(Error::from(LinkBroken)),
        }
    }
}

fn extract_links_from_document(body: &str) -> Result<IndexSet<String>, Error> {
    let document = Html::parse_document(&body);
    let picker = Selector::parse(".module-item").unwrap();

    if body.contains("<h1>The requested resource does not exist</h1>") {
        Err(Error::from(LinkBroken))
    } else {
        let mut text = IndexSet::default();
        for node in document.select(&picker) {
            let attributes = node.inner_html();
            let pred: &[_] = &['<', 't', 'd', '>', '/'];
            attributes
                .trim_matches(pred)
                .split(' ')
                .filter(|x| x.contains("href") && !x.contains("\"../"))
                .map(|x| {
                    let replace = x.replace("href=\"", "");
                    let link = replace.split(">").take(1).collect::<String>();
                    if link.is_empty() {
                        replace.to_string()
                    } else {
                        link
                    }
                }).map(|x| {
                    let replace = x.split("\"").take(1).collect::<String>();
                    if replace.is_empty() {
                        x.to_string()
                    } else {
                        replace
                    }
                }).map(|x| x.replace("\"", ""))
                .for_each(|x| {
                    text.insert(x);
                });
        }

        Ok(text)
    }
}

#[derive(Clone, Debug)]
pub struct LinksList(pub Vec<String>);

impl LinksList {
    pub fn parse(links: &[String]) -> LinksList {
        LinksList(links.to_vec())
    }
}

impl Deref for LinksList {
    type Target = Vec<String>;

    fn deref(&self) -> &Vec<String> {
        &self.0
    }
}

impl DerefMut for LinksList {
    fn deref_mut(&mut self) -> &mut Vec<String> {
        &mut self.0
    }
}

impl Display for LinksList {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for item in self.iter() {
            writeln!(f, "{}", item)?;
        }

        Ok(())
    }
}

impl IntoIterator for LinksList {
    type Item = String;
    type IntoIter = IntoIter<String>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[derive(Debug, Fail)]
#[fail(display = "link is broken or doesn't exist")]
struct LinkBroken;
