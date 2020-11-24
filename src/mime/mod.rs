use std::collections::HashMap;
use crate::sources::get_sources;

pub struct Mime<'a> {
    db: HashMap<&'a str, MimeType>
}

fn is_mime(a: &str) -> bool {
    let split: Vec<&str> = a.split("/").collect();

    return split.len() == 2
}

impl<'a> Mime<'a> {
    pub fn load() -> Self {
        return Mime{ db: get_sources() }
    }
}

pub struct MimeType {
    extensions: Vec<String>,
    charset: String,
}

impl MimeType {
    pub fn new(extensions: Vec<String>, charset: String) -> MimeType {
        return MimeType { extensions, charset };
    }

    pub fn extensions(&self) -> &Vec<String> {
        return &self.extensions;
    }

    pub fn charset(&self) -> &String {
        return &self.charset;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn is_mime_test() {
        assert_eq!(true, is_mime("path/file.txt"));
        assert_eq!(false, is_mime(&""));
    }

    #[test]
    fn is() {
        let m = MimeType::new(
            vec![
                "txt".to_owned(),
                "doc".to_owned()
            ],
            "UTF-8".to_owned(),
        );

        assert_eq!("UTF-8", m.charset());

        let a = vec![
            "txt".to_owned(),
            "doc".to_owned()
        ];

        assert_eq!(&a, m.extensions());
    }
}