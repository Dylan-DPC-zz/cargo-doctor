use linker::Scraper;
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
        let base_url = Url::parse(self.base_url.as_str())?;
        let mut scraper = Scraper::new(&base_url).scrape().expect("base url not found");
        let mut broken_links = vec![];

        while let Some(link) = scraper.pop() {
            let url = base_url.join(&link).expect("cannot join links");
            match Scraper::new(&url).scrape() {
                Ok(contents) => scraper.0.extend(contents),
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


#[cfg(test)]
mod tests {
    use super::Handler;
    #[test]
    fn test_scrape() {
        let results = Handler::new(&"https://docs.rs/uuid/0.7.0/uuid/").scrape();
        println!("{:?}", results );
    }
}