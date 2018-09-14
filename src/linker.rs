use reqwest::{Url, Response, get};
use failure::Error;
use scraper::{Html, Selector};

pub struct Scraper {
    pub path: Url
}

impl Scraper {
    pub fn from(path: &str) -> Result<Scraper, Error> {
        Ok(Scraper { path: Url::parse(path)? })
    }

    pub fn scrape(self, selector: &str) -> Result<Vec<String>, Error> {
        let body = get(self.path.as_str())?.text()?;
        let document = Html::parse_document(&body);
        let picker = Selector::parse(".module-item").unwrap();

        let mut text = vec![];
        for node in document.select(&picker) {
            let attributes = node.inner_html();
            let pred: &[_] = &['<', 't', 'd', '>', '/'];
            let attribute_pred: &[_] = &['h', 'r', 'e', 'f', '=', '"'];
            let links_list = attributes.trim_matches(pred).split(' ')
                .filter(|x| x. contains("href"))
                .map(|x| x.trim_matches(attribute_pred).to_owned())
                .for_each(|x| text.push(x));
        }
        Ok(text)
    }
}

#[cfg(test)]
mod tests {
    use super::Scraper;
    #[test]
    fn test_scrape() {
         let results = Scraper::from("https://docs.rs/uuid/0.7.0/uuid/").unwrap().scrape("");
    }
}