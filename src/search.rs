use std::sync::Arc;

use serde::{Deserialize, Serialize};
use skim::item::RankBuilder;
use skim::prelude::{ExactOrFuzzyEngineFactory, RegexEngineFactory};
use skim::{MatchEngine, MatchEngineFactory, SkimItem};

pub struct PathItem {
    path: String,
}

impl From<String> for PathItem {
    fn from(value: String) -> Self {
        Self { path: value }
    }
}

impl SkimItem for PathItem {
    fn text(&self) -> std::borrow::Cow<'_, str> {
        std::borrow::Cow::from(&self.path)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum RankCriteria {
    Score,
    NegScore,
    Begin,
    NegBegin,
    End,
    NegEnd,
    Length,
    NegLength,
    Index,
    NegIndex,
}

impl Into<skim::prelude::RankCriteria> for RankCriteria {
    fn into(self) -> skim::prelude::RankCriteria {
        match self {
            Self::Score => skim::prelude::RankCriteria::Score,
            Self::NegScore => skim::prelude::RankCriteria::NegScore,
            Self::Begin => skim::prelude::RankCriteria::Begin,
            Self::NegBegin => skim::prelude::RankCriteria::NegBegin,
            Self::End => skim::prelude::RankCriteria::End,
            Self::NegEnd => skim::prelude::RankCriteria::NegEnd,
            Self::Length => skim::prelude::RankCriteria::Length,
            Self::NegLength => skim::prelude::RankCriteria::NegLength,
            Self::Index => skim::prelude::RankCriteria::Index,
            Self::NegIndex => skim::prelude::RankCriteria::NegIndex,
        }
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

    pub fn engine(
        &self,
        pattern: &str,
        exact_mode: bool,
        rank_criteria: Option<Vec<RankCriteria>>,
    ) -> Box<dyn MatchEngine> {
        let criteria = rank_criteria.map_or_else(
            || {
                vec![
                    RankCriteria::Score,
                    RankCriteria::Begin,
                    RankCriteria::End,
                    RankCriteria::Length,
                ]
            },
            Into::into,
        );

        let rank_builder =
            RankBuilder::new(criteria.into_iter().map(Into::into).collect());

        match self {
            Self::Fuzzy => ExactOrFuzzyEngineFactory::builder()
                .exact_mode(exact_mode)
                .rank_builder(Arc::new(rank_builder))
                .build()
                .create_engine(pattern),

            Self::Regex => RegexEngineFactory::builder()
                .rank_builder(Arc::new(rank_builder))
                .build()
                .create_engine(pattern),
        }
    }
}
