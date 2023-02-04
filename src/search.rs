use std::sync::Arc;

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use skim::prelude::{ExactOrFuzzyEngineFactory, RegexEngineFactory};
use skim::{MatchEngine, MatchEngineFactory, SkimItem};

use crate::node::Node;

lazy_static! {
    static ref FUZZY_FACTORY: ExactOrFuzzyEngineFactory =
        ExactOrFuzzyEngineFactory::builder().build();
    static ref REGEX_FACTORY: RegexEngineFactory = RegexEngineFactory::builder().build();
}

struct PathItem {
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

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum SkimAlgorithm {
    #[default]
    Fuzzy,
    Regex,
}

impl SkimAlgorithm {
    fn engine(&self, pattern: &str) -> Box<dyn MatchEngine> {
        match self {
            Self::Fuzzy => FUZZY_FACTORY.create_engine(pattern),
            Self::Regex => REGEX_FACTORY.create_engine(pattern),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum SearchAlgorithm {
    Skim(SkimAlgorithm),
}

impl Default for SearchAlgorithm {
    fn default() -> Self {
        Self::Skim(SkimAlgorithm::default())
    }
}

impl SearchAlgorithm {
    pub fn search(&self, pattern: &str, nodes: Vec<Node>) -> Vec<(Node, [i32; 4])> {
        match self {
            Self::Skim(algorithm) => {
                let engine = algorithm.engine(pattern);
                nodes
                    .into_iter()
                    .filter_map(|n| {
                        let item = Arc::new(PathItem::from(n.relative_path.clone()));
                        engine.match_item(item).map(|res| (n, res.rank))
                    })
                    .collect::<Vec<(_, _)>>()
            }
        }
    }

    pub fn label(&self) -> String {
        match self {
            SearchAlgorithm::Skim(algorithm) => {
                let kind = match algorithm {
                    SkimAlgorithm::Fuzzy => "fuzzy",
                    SkimAlgorithm::Regex => "regex",
                };
                format!("skim ({kind})")
            }
        }
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum SearchOrder {
    #[default]
    Ranked,
    Sorted,
}

impl SearchOrder {
    pub fn label(&self) -> String {
        match self {
            SearchOrder::Ranked => "rank".to_string(),
            SearchOrder::Sorted => "sort".to_string(),
        }
    }
}
