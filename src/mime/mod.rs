pub struct MimeType {
    extension: String,
    charset: String,
}

impl MimeType {
    pub fn new(extension: String, charset: String) -> MimeType {
        return MimeType { extension, charset };
    }

    pub fn extension(&self) -> &String {
        return &self.extension;
    }

    pub fn charset(&self) -> &String {
        return &self.charset;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn is() {
        let m = MimeType::new(
            "test".to_owned(),
            "UTF-8".to_owned(),
        );

        assert_eq!("UTF-8", m.charset());
        assert_eq!("test", m.extension());
    }
}