#[derive(Debug)]
pub enum Error {
    MissingBeginningLine,
    MissingEndingLine,
    SerdeYaml(serde_yaml::Error),
}

impl From<serde_yaml::Error> for Error {
    fn from(error: serde_yaml::Error) -> Self {
        Error::SerdeYaml(error)
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug)]
pub struct Parsed<'a, T: serde::de::DeserializeOwned> {
    pub body: &'a str,
    pub headers: T,
}

static LINE_PATTERN: &str = "---\n";

pub fn parse<T: serde::de::DeserializeOwned>(text: &str) -> Result<Parsed<T>> {
    if !text.starts_with(LINE_PATTERN) {
        return Err(Error::MissingBeginningLine);
    }

    let slice = &text[LINE_PATTERN.len()..];
    let index_of_ending_line = slice.find(LINE_PATTERN).ok_or(Error::MissingEndingLine)?;
    Ok(Parsed {
        body: &slice[(index_of_ending_line + LINE_PATTERN.len())..],
        headers: serde_yaml::from_str(&slice[..index_of_ending_line])?,
    })
}

#[cfg(test)]
mod tests {
    use super::{parse, Error};
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    struct Headers {
        title: String,
    }

    #[test]
    fn parse_with_missing_beginning_line() {
        let text = "";
        let result = parse::<Headers>(text);
        assert!(matches!(result, Err(Error::MissingBeginningLine)));
    }

    #[test]
    fn parse_with_missing_ending_line() {
        let text = "---\n";
        let result = parse::<Headers>(text);
        assert!(matches!(result, Err(Error::MissingEndingLine)));
    }

    #[test]
    fn parse_with_empty_frontmatter() {
        let text = "---\n---\n";
        let result = parse::<Headers>(text);
        assert!(matches!(result, Err(Error::SerdeYaml(_))));
    }

    #[test]
    fn parse_with_missing_known_field() {
        let text = "---\ndate: 2000-01-01\n---\n";
        let result = parse::<Headers>(text);
        assert!(matches!(result, Err(Error::SerdeYaml(_))));
    }

    #[test]
    fn parse_with_unknown_field() {
        let text = "---\ndate: 2000-01-01\ntitle: dummy_title\n---\n";
        let result = parse::<Headers>(text);
        dbg!(&result);
        assert!(matches!(result, Ok(_)));
    }

    #[test]
    fn parse_with_empty_known_field() {
        let text = "---\ntitle:\n---\n";
        let result = parse::<Headers>(text).unwrap();
        assert_eq!(result.headers.title, "~");
        assert_eq!(result.body, "");
    }

    #[test]
    fn parse_with_valid_frontmatter() {
        let text = "---\ntitle: dummy_title---\ndummy_body";
        let result = parse::<Headers>(text).unwrap();
        assert_eq!(result.headers.title, "dummy_title");
        assert_eq!(result.body, "dummy_body");
    }
}
