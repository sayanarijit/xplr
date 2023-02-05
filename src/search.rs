use crate::node::Node;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use skim::prelude::{ExactOrFuzzyEngineFactory, RegexEngineFactory};
use skim::{MatchEngine, MatchEngineFactory, SkimItem};
use std::cmp::Ordering;
use std::sync::Arc;

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

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum SearchAlgorithm {
    #[default]
    Fuzzy,
    FuzzyUnranked,
    Regex,
    RegexUnranked,
}

impl SearchAlgorithm {
    pub fn is_ranked(&self) -> bool {
        match self {
            Self::Fuzzy | Self::Regex => true,
            Self::FuzzyUnranked | Self::RegexUnranked => false,
        }
    }

    pub fn cycle(self) -> Self {
        match self {
            Self::Fuzzy => Self::FuzzyUnranked,
            Self::FuzzyUnranked => Self::Regex,
            Self::Regex => Self::RegexUnranked,
            Self::RegexUnranked => Self::Fuzzy,
        }
    }

    pub fn enable_ranking(self) -> Self {
        match self {
            Self::FuzzyUnranked => Self::Fuzzy,
            Self::RegexUnranked => Self::Regex,
            Self::Fuzzy | Self::Regex => self,
        }
    }

    pub fn disable_ranking(self) -> Self {
        match self {
            Self::Fuzzy => Self::FuzzyUnranked,
            Self::Regex => Self::RegexUnranked,
            Self::FuzzyUnranked | Self::RegexUnranked => self,
        }
    }

    pub fn toggle_ranking(self) -> Self {
        match self {
            Self::Fuzzy => Self::FuzzyUnranked,
            Self::FuzzyUnranked => Self::Fuzzy,
            Self::Regex => Self::RegexUnranked,
            Self::RegexUnranked => Self::Regex,
        }
    }

    fn engine(&self, pattern: &str) -> Box<dyn MatchEngine> {
        match self {
            Self::Fuzzy | Self::FuzzyUnranked => FUZZY_FACTORY.create_engine(pattern),
            Self::Regex | Self::RegexUnranked => REGEX_FACTORY.create_engine(pattern),
        }
    }

    pub fn search<F>(&self, pattern: &str, nodes: Vec<Node>, sort: F) -> Vec<Node>
    where
        F: FnMut(&Node, &Node) -> Ordering,
    {
        let engine = self.engine(pattern);
        let mut ranked_nodes = nodes
            .into_iter()
            .filter_map(|n| {
                let item = Arc::new(PathItem::from(n.relative_path.clone()));
                engine.match_item(item).map(|res| (n, res.rank))
            })
            .collect::<Vec<(_, _)>>();

        if self.is_ranked() {
            ranked_nodes.sort_by(|(_, s1), (_, s2)| s2.cmp(s1));
            ranked_nodes.into_iter().map(|(n, _)| n).collect::<Vec<_>>()
        } else {
            let mut filtered_nodes =
                ranked_nodes.into_iter().map(|(n, _)| n).collect::<Vec<_>>();
            filtered_nodes.sort_by(sort);
            filtered_nodes
        }
    }
}
