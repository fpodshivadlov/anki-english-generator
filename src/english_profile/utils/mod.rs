use url::Url;
use url::ParseError;

// const FETCH_DELAY: u32 = 1000;
// function pathify(string: string) {
//   return string.replace(/\W/g, '_');
// }

pub fn ensure_absolute_url(url_str: &str, base_url_str: &str) -> Option<String> {
  let url_result = Url::parse(url_str);
  match url_result {
    Ok(url) => {
      return Some(url.to_string());
    },
    Err(ParseError::RelativeUrlWithoutBase) => {
      let base_url = Url::parse(base_url_str).unwrap();
      let result = base_url.join(url_str).unwrap();
      //let mut result_url = url;
      //result_url.set_host(Some(another_url.host_str().unwrap()));
      return Some(result.to_string());
    },
    _ => None,
  }
}

#[cfg(test)]
mod tests;
