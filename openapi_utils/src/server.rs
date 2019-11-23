use openapiv3::*;
use http::Uri;

pub trait ServerExt {
    fn base_path(&self) -> String;
}

impl ServerExt for Server {
    fn base_path(&self) -> String {
        if let Some(variables) = &self.variables {
            match variables.get("basePath") {
                Some(base_path) => {
                    let mut base_str = base_path.default.clone();
                    let last_character = base_str.chars().last().unwrap();
                    if last_character == '/' {
                        base_str.pop();
                    }
                    base_str.clone()
                }
                None => "".to_string(),
            }
        } else {
            let url_parse = &self.url.parse::<Uri>();
            match url_parse {
                Ok(url) => url.path().to_string(),
                Err(_) => "".to_string(),
            }
        }
    }
}