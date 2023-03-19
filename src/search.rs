use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use skim::prelude::{ExactOrFuzzyEngineFactory, RegexEngineFactory};
use skim::{MatchEngine, MatchEngineFactory, SkimItem};

lazy_static! {
    static ref FUZZY_FACTORY: ExactOrFuzzyEngineFactory =
        ExactOrFuzzyEngineFactory::builder().build();
    static ref REGEX_FACTORY: RegexEngineFactory = RegexEngineFactory::builder().build();
}

pub struct PathItem {
    path: String,
}

impl From<String> for PathItem {
    fn from(value: String) -> Self {
        Self { path: value }
    }
}

impl SkimItem for PathItem {
    fn text(&self) -> std::borrow::Cow<str> {
        std::borrow::Cow::from(&self.path)
    }
}

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum SearchAlgorithm {
    #[default]
    Fuzzy,
    Regex,
}

impl SearchAlgorithm {
    pub fn toggle(self) -> Self {
        match self {
            Self::Fuzzy => Self::Regex,
            Self::Regex => Self::Fuzzy,
        }
    }

    pub fn engine(&self, pattern: &str) -> Box<dyn MatchEngine> {
        match self {
            Self::Fuzzy => FUZZY_FACTORY.create_engine(pattern),
            Self::Regex => REGEX_FACTORY.create_engine(pattern),
        }
    }
}
