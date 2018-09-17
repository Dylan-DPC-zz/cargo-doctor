use linker::{Scraper, LinksList};
use failure::Error;
use reqwest::Url;

#[derive(Clone, Debug)]
pub struct Handler {
    base_url: String
}

impl Handler {
    pub fn new(base_url: &str) -> Handler {
        Handler { base_url: base_url.to_owned() }
    }

    pub fn scrape(&self) -> Result<(), Error> {
        let mut base_url = Url::parse(self.base_url.as_str())?;
        let mut scraper = Scraper::new(&base_url).scrape().unwrap();

        while let Some(link) = scraper.pop() {
            let url = base_url.join(&link).expect("cannot join links");
            let inner_scraper = Scraper::new(&url).scrape()?;
            scraper.extend(inner_scraper);
        }

        Ok(())

    }
}

#[cfg(test)]
mod tests {
    use super::Handler;
    use reqwest::Url;
    #[test]
    fn test_scrape() {
        let results = Handler::new(&"https://docs.rs/uuid/0.7.0/uuid/").scrape();
        println!("{:?}", results );
    }
}