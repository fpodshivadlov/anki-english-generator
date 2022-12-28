
use std::io::{ Error, ErrorKind};

use base_url::Url;

pub fn set_base_url(page_url: &str, url: &str) -> String {
    let page_url = Url::parse(page_url).unwrap();
    let base_url = base_url(page_url)
        .expect("base url parsing ");

    let options = Url::options();
    let options = options.base_url(Some(&base_url));
    
    let result_url = options.parse(url).unwrap();
    result_url.to_string()
}

fn base_url(mut url: Url) -> Result<Url, Error> {
    match url.path_segments_mut() {
        Ok(mut path) => {
            path.clear();
        }
        Err(_) => {
            return Err(Error::new(ErrorKind::Other, "Cannot be a base"));
        }
    }

    url.set_query(None);

    Ok(url)
}