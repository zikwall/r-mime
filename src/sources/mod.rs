use std::collections::HashMap;
use crate::mime::MimeType;

macro_rules! source_map {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map: HashMap<&str, MimeType> = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

pub fn get_sources<'a>() -> HashMap<&'a str, MimeType> {
    let sources: HashMap<&str, MimeType> = source_map!(
        "video/x-msvideo"   => MimeType::new(vec![String::from("avi")], String::from("")),
        "video/x-matroska"  => MimeType::new(vec![String::from("mkv"), String::from("mk3d"), String::from("mks")], String::from("")),
        "video/webm"        => MimeType::new(vec![String::from("webm")], String::from(""))
    );

    return sources;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn include() {
        let sources = get_sources();

        assert_eq!(true, sources.contains_key("video/x-matroska"));

        match sources.get("video/x-matroska") {
            Some(source) => {
                let a = vec![
                    "mkv".to_owned(),
                    "mk3d".to_owned(),
                    "mks".to_owned()
                ];

                assert_eq!(&a, source.extensions());
            },
           _ => panic!("Cause: source not exist"),
        }
    }
}