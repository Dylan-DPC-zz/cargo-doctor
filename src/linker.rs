use reqwest::{Url, get};
use failure::Error;
use scraper::{Html, Selector};
use std::ops::{Deref, DerefMut};
use indexmap::{IndexSet, set::IntoIter};

pub struct Scraper {
    pub path: Url
}

impl Scraper {
    pub fn new(path: &Url) -> Scraper {
        Scraper { path: path.to_owned() }
    }

    pub fn from(path: &str) -> Result<Scraper, Error> {
        Ok(Scraper { path: Url::parse(path)? })
    }

    pub fn scrape(self) -> Result<LinksList, Error> {
        let body = get(self.path.as_str())?.text()?;
        let document = Html::parse_document(&body);
        let picker = Selector::parse(".module-item").unwrap();

        if body.contains("<h1>The requested resource does not exist</h1>") {
            Err(Error::from(LinkBroken))
        } else {
            let mut text = IndexSet::default();
            for node in document.select(&picker) {
                let attributes = node.inner_html();
                let pred: &[_] = &['<', 't', 'd', '>', '/'];
                let attribute_pred: &[_] = &['h', 'r', 'e', 'f', '=', '"'];
                attributes.trim_matches(pred).split(' ')
                    .filter(|x| x.contains("href") && !x.contains("\"../"))
                    .map(|x| x.trim_matches(attribute_pred).splitn(2, '\"').take(1).collect::<String>())
                    .for_each(|x| { text.insert(x); });
            }
            Ok(LinksList::parse(text))
        }
    }

}



#[derive(Clone, Debug)]
pub struct LinksList(pub IndexSet<String>);

impl LinksList {
    fn parse<T: Into<IndexSet<String>>>(links: T) -> LinksList {
        LinksList(links.into())
    }
}

impl Deref for LinksList {
    type Target = IndexSet<String>;

    fn deref(&self) -> &IndexSet<String> {
        &self.0
    }

}

impl DerefMut for LinksList {
    fn deref_mut(&mut self) -> &mut IndexSet<String> {
        &mut self.0
    }

}

impl IntoIterator for LinksList
{
    type Item = String;
    type IntoIter = IntoIter<String>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[derive(Debug, Fail)]
#[fail(display = "link is broken or doesn't exist")]
struct LinkBroken;

