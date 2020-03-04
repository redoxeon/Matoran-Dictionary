use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
pub struct MatoranDictionary {
    entries: HashMap<String, MatoranDictionaryEntry>,
}

#[derive(Debug)]
pub struct MatoranDictionaryEntry {
    entry: String,
    typing: WordType,
    pronounciation: String,
    definition: String,
    etymology: String,
}

impl MatoranDictionaryEntry {
    pub fn new(
        entr: String,
        word_type: WordType,
        pronounce: String,
        def: String,
        etym: String,
    ) -> Self {
        MatoranDictionaryEntry {
            entry: entr,
            typing: word_type,
            pronounciation: pronounce,
            definition: def,
            etymology: etym,
        }
    }
}

pub enum WordType {
    Adjective,
    AdjectiveCompound,
    Adverb,
    Affix,
    Noun,
    NounCompound,
    Particle,
    ParticleCompound,
    Pronoun,
    Stem,
    Verb,
    VerbalNoun,
    VerbalNounCompound,
    None,
}

impl std::fmt::Debug for WordType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WordType::Adjective => write!(f, "Adjective"),
            WordType::AdjectiveCompound => write!(f, "AdjectiveCompound"),
            WordType::Adverb => write!(f, "Adverb"),
            WordType::Affix => write!(f, "Affix"),
            WordType::Noun => write!(f, "Noun"),
            WordType::NounCompound => write!(f, "NounCompound"),
            WordType::Particle => write!(f, "Particle"),
            WordType::ParticleCompound => write!(f, "ParticleCompound"),
            WordType::Pronoun => write!(f, "Pronoun"),
            WordType::Stem => write!(f, "Stem"),
            WordType::Verb => write!(f, "Verb"),
            WordType::VerbalNoun => write!(f, "VerbalNoun"),
            WordType::VerbalNounCompound => write!(f, "VerbalNounCompound"),
            _ => write!(f, "None"),
        }
    }
}

impl MatoranDictionary {
    pub fn new() -> Self {
        MatoranDictionary {
            entries: HashMap::new(),
        }
        // TODO: Either read in the whole thing when you start a new one or hard code the whole thing in here.
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
