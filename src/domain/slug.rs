#[derive(Debug, PartialEq, Eq)]
pub struct Slug(String);

#[derive(PartialEq, Debug, Eq)]
pub enum SlugError {
    Empty,
    EdgeHyphen,
    InvalidCharacter,
}

impl Slug {
    pub fn new(value: &str) -> Result<Self, SlugError> {
        if value.is_empty() {
            return Err(SlugError::Empty);
        }
        if value.starts_with('-') || value.ends_with('-') {
            return Err(SlugError::EdgeHyphen);
        }
        if value
            .chars()
            .any(|ch| !ch.is_ascii_lowercase() && !ch.is_ascii_digit() && ch != '-')
        {
            return Err(SlugError::InvalidCharacter);
        }

        Ok(Self(value.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn value(self) -> String {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rejects_empty_slug() {
        let slug = Slug::new("");
        assert_eq!(slug, Err(SlugError::Empty));
    }

    #[test]
    fn rejects_edge_hyphen() {
        let leading = Slug::new("-edge-hyphen");
        let trailing = Slug::new("edge-hyphen-");

        assert_eq!(leading, Err(SlugError::EdgeHyphen));
        assert_eq!(trailing, Err(SlugError::EdgeHyphen));
    }

    #[test]
    fn rejects_invalid_character() {
        let slug = Slug::new("☺");
        assert_eq!(slug, Err(SlugError::InvalidCharacter));
    }

    #[test]
    fn accepts_valid_slug() {
        let valid_inp = "best-post-ever";
        let slug = Slug::new(valid_inp);

        assert_eq!(slug, Ok(Slug(valid_inp.to_string())));
    }
}
