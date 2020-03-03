use std::fmt;

#[derive(Debug)]
pub struct MatoranDictionaryEntry {
    entry: String,
    word_type: String,
    ix_kind: AffixKind,
    pronounciation: String,
    definition: String,
    etymology: String,
}

pub enum AffixKind {
    Prefix,
    Suffix,
    None,
}

impl std::fmt::Debug for AffixKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AffixKind::Prefix => write!(f, "Prefix"),
            AffixKind::Suffix => write!(f, "Suffix"),
            _ => write!(f, "Neither"),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
