/// Game data related to the classic version of the game
pub mod classic {
    use std::collections::HashMap;

    use serde::{Deserialize, Serialize};
    use serde_json::from_slice;

    pub(crate) const LEVELS: &'static [u8] = include_bytes!("../data/classic/levels.json");
    pub(crate) const NAMES: &'static [u8] = include_bytes!("../data/classic/names.json");
    pub(crate) const CHAPTERS: &'static [u8] = include_bytes!("../data/classic/chapters.json");

    /// A list of all chapters in this version of the game
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
    pub enum Chapter {
        #[serde(rename = "Chapter 1")]
        Chapter1,
        #[serde(rename = "Chapter 2")]
        Chapter2,
        #[serde(rename = "Chapter 3")]
        Chapter3,
        #[serde(rename = "Chapter 4")]
        Chapter4,
        #[serde(rename = "Chapter 5")]
        Chapter5,
        #[serde(rename = "Chapter 6")]
        Chapter6,
    }

    /// Used to generate the rust data from the source files.  
    ///
    /// ```
    /// use miu::data::classic;
    ///
    /// let data = classic::Data::new().unwrap();
    ///
    /// println!("levels: {:#?}", data.levels());
    /// ```
    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Data {
        levels: Vec<String>,
        names: HashMap<String, String>,
        chapters: HashMap<Chapter, Vec<String>>,
    }

    impl Data {
        /// Create a new source of data
        ///
        /// Prefer to only call this once as you can't own the inner data and this parses all of the json only once.  
        pub fn new() -> Result<Self, serde_json::Error> {
            Ok(Self {
                levels: from_slice(LEVELS)?,
                names: from_slice(NAMES)?,
                chapters: from_slice(CHAPTERS)?,
            })
        }

        /// A list of all level ids.  
        ///
        /// Note that some Ids will have `SP_` at the start, this one **doesn't**
        pub fn levels(&self) -> &Vec<String> {
            &self.levels
        }

        /// A map of `level id` => `human readable name`.  
        pub fn names(&self) -> &HashMap<String, String> {
            &self.names
        }

        /// A map of `chapter` => `list of level ids`
        pub fn chapters(&self) -> &HashMap<Chapter, Vec<String>> {
            &self.chapters
        }
    }
}

/// Game data related to the ultra version of the game
pub mod ultra {
    use std::collections::HashMap;

    use serde::{Deserialize, Serialize};
    use serde_json::from_slice;

    pub(crate) const LEVELS: &'static [u8] = include_bytes!("../data/ultra/levels.json");
    pub(crate) const NAMES: &'static [u8] = include_bytes!("../data/ultra/names.json");
    pub(crate) const CHAPTERS: &'static [u8] = include_bytes!("../data/ultra/chapters.json");

    /// A list of all chapters in this version of the game
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
    pub enum Chapter {
        #[serde(rename = "Chapter 1")]
        Chapter1,
        #[serde(rename = "Chapter 2")]
        Chapter2,
        #[serde(rename = "Chapter 3")]
        Chapter3,
        #[serde(rename = "Chapter 4")]
        Chapter4,
        #[serde(rename = "Chapter 5")]
        Chapter5,
        #[serde(rename = "Chapter 6")]
        Chapter6,
        #[serde(rename = "Bonus 1")]
        Bonus1,
        #[serde(rename = "Bonus 2")]
        Bonus2,
        #[serde(rename = "Bonus 3")]
        Bonus3,
        #[serde(rename = "Bonus 4")]
        Bonus4,
    }

    /// Used to generate the rust data from the source files.  
    ///
    /// ```
    /// use miu::data::ultra;
    ///
    /// let data = ultra::Data::new().unwrap();
    ///
    /// println!("levels: {:#?}", data.levels());
    /// ```
    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Data {
        levels: Vec<String>,
        names: HashMap<String, String>,
        chapters: HashMap<Chapter, Vec<String>>,
    }

    impl Data {
        /// Create a new source of data.  
        ///
        /// Prefer to only call this once as you can't own the inner data and this parses all of the json only once.  
        pub fn new() -> Result<Self, serde_json::Error> {
            Ok(Self {
                levels: from_slice(LEVELS)?,
                names: from_slice(NAMES)?,
                chapters: from_slice(CHAPTERS)?,
            })
        }

        /// A list of all level ids.  
        ///
        /// Note that some Ids will have `SP_` at the start, this one **doesn't**
        pub fn levels(&self) -> &Vec<String> {
            &self.levels
        }

        /// A map of `level id` => `human readable name`.  
        pub fn names(&self) -> &HashMap<String, String> {
            &self.names
        }

        /// A map of `chapter` => `list of level ids`
        pub fn chapters(&self) -> &HashMap<Chapter, Vec<String>> {
            &self.chapters
        }
    }
}

#[cfg(test)]
mod test {
    use crate::data::{classic, ultra};

    #[test]
    fn classic_data() {
        let data = classic::Data::new().unwrap();

        assert!(!data.levels().is_empty());
        assert!(!data.names().is_empty());
        assert!(!data.chapters().is_empty());

        assert!(data.chapters().get(&classic::Chapter::Chapter1).is_some());
        assert!(data.chapters().get(&classic::Chapter::Chapter6).is_some());

        for level in data.levels() {
            assert!(data.names().get(level).is_some());
        }
    }

    #[test]
    fn ultra_data() {
        let data = ultra::Data::new().unwrap();

        assert!(!data.levels().is_empty());
        assert!(!data.names().is_empty());
        assert!(!data.chapters().is_empty());

        assert!(data.chapters().get(&ultra::Chapter::Chapter1).is_some());
        assert!(data.chapters().get(&ultra::Chapter::Chapter6).is_some());
        assert!(data.chapters().get(&ultra::Chapter::Bonus1).is_some());
        assert!(data.chapters().get(&ultra::Chapter::Bonus4).is_some());

        for level in data.levels() {
            assert!(data.names().get(level).is_some());
        }
    }
}
