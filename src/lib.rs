use rand::{seq::SliceRandom, thread_rng};

macro_rules! include_word_list {
    ($filename:expr) => {
        include_str!(concat!(env!("OUT_DIR"), "/", $filename))
    };
}

pub enum WordList {
    GfycatAdjectives,
    GfycatAnimals,
    EffShortWordlist20,
    EffShortWordlist1,
    EffLargeWordlist,
    UniqueNamesGeneratorAdjectives,
    UniqueNamesGeneratorAnimals,
    UniqueNamesGeneratorColors,
    UniqueNamesGeneratorCountries,
    UniqueNamesGeneratorNames,
    UniqueNamesGeneratorStarWars,
    AdjectiveAdjectiveAnimalAdjectives,
    AdjectiveAdjectiveAnimalAnimals,
    WittyPhraseGeneratorAdjectives,
    WittyPhraseGeneratorIntensifiers,
    WittyPhraseGeneratorNouns,
    MemorableWordlistWords,
}

impl WordList {
    fn get_word_list(&self) -> &'static str {
        match self {
            WordList::GfycatAdjectives => include_word_list!("gfycat_adjectives.txt"),
            WordList::GfycatAnimals => include_word_list!("gfycat_animals.txt"),
            WordList::EffShortWordlist20 => include_word_list!("eff_short_wordlist_2_0.txt"),
            WordList::EffShortWordlist1 => include_word_list!("eff_short_wordlist_1.txt"),
            WordList::EffLargeWordlist => include_word_list!("eff_large_wordlist.txt"),
            WordList::UniqueNamesGeneratorAdjectives => {
                include_word_list!("unique_names_generator_adjectives.txt")
            }
            WordList::UniqueNamesGeneratorAnimals => {
                include_word_list!("unique_names_generator_animals.txt")
            }
            WordList::UniqueNamesGeneratorColors => {
                include_word_list!("unique_names_generator_colors.txt")
            }
            WordList::UniqueNamesGeneratorCountries => {
                include_word_list!("unique_names_generator_countries.txt")
            }
            WordList::UniqueNamesGeneratorNames => {
                include_word_list!("unique_names_generator_names.txt")
            }
            WordList::UniqueNamesGeneratorStarWars => {
                include_word_list!("unique_names_generator_star-wars.txt")
            }
            WordList::AdjectiveAdjectiveAnimalAdjectives => {
                include_word_list!("adjective_adjective_animal_adjectives.txt")
            }
            WordList::AdjectiveAdjectiveAnimalAnimals => {
                include_word_list!("adjective_adjective_animal_animals.txt")
            }
            WordList::WittyPhraseGeneratorAdjectives => {
                include_word_list!("witty_phrase_generator_adjectives.txt")
            }
            WordList::WittyPhraseGeneratorIntensifiers => {
                include_word_list!("witty_phrase_generator_intensifiers.txt")
            }
            WordList::WittyPhraseGeneratorNouns => {
                include_word_list!("witty_phrase_generator_nouns.txt")
            }
            WordList::MemorableWordlistWords => include_word_list!("memorable_wordlist_words.txt"),
        }
    }

    /// Get a random word from the word list
    fn get_random_word(&self) -> Option<&'static str> {
        // this is slooooow
        // and not secure
        let mut rng = thread_rng();
        let word_list = self.get_word_list();
        let words: Vec<&'static str> = word_list.lines().collect();
        // except is meh
        // but it's a failure to get random
        let random_word = words.choose(&mut rng).copied();
        random_word
    }
}

pub struct RandomSequence {
    words: Vec<&'static str>,
}

impl RandomSequence {
    /// Create sequence
    pub fn new(word_list: &[WordList]) -> Option<Self> {
        let words: Option<Vec<&str>> = word_list
            .iter()
            .map(|word_list| word_list.get_random_word())
            .collect();
        words.map(|words| Self { words })
    }

    pub fn as_space_delimited_string(&self) -> String {
        self.words.join(" ")
    }

    pub fn as_snake_case_string(&self) -> String {
        self.words.join("_")
    }

    pub fn as_kebab_case_string(&self) -> String {
        self.words.join("-")
    }
}

pub fn simple_adjective_adjective_animal() -> String {
    let sequence = RandomSequence::new(&[
        WordList::GfycatAdjectives,
        WordList::GfycatAdjectives,
        WordList::GfycatAnimals,
    ])
    .unwrap();
    sequence.as_snake_case_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_word_list() {
        let _ = WordList::GfycatAnimals.get_word_list();
    }

    #[test]
    fn test_simple_adjective_adjective_animal() {
        let _ = simple_adjective_adjective_animal();
    }
}
