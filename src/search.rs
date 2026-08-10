use std::sync::Arc;

use serde::{Deserialize, Serialize};
use skim::item::RankBuilder;
use skim::prelude::{ExactOrFuzzyEngineFactory, RegexEngineFactory};
use skim::{MatchEngine, MatchEngineFactory};

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

impl From<RankCriteria> for skim::prelude::RankCriteria {
    fn from(criterion: RankCriteria) -> Self {
        match criterion {
            RankCriteria::Score => skim::prelude::RankCriteria::Score,
            RankCriteria::NegScore => skim::prelude::RankCriteria::NegScore,
            RankCriteria::Begin => skim::prelude::RankCriteria::Begin,
            RankCriteria::NegBegin => skim::prelude::RankCriteria::NegBegin,
            RankCriteria::End => skim::prelude::RankCriteria::End,
            RankCriteria::NegEnd => skim::prelude::RankCriteria::NegEnd,
            RankCriteria::Length => skim::prelude::RankCriteria::Length,
            RankCriteria::NegLength => skim::prelude::RankCriteria::NegLength,
            RankCriteria::Index => skim::prelude::RankCriteria::Index,
            RankCriteria::NegIndex => skim::prelude::RankCriteria::NegIndex,
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

    pub fn rank_builder_criteria(
        rank_criteria: Option<&[RankCriteria]>,
    ) -> Vec<skim::prelude::RankCriteria> {
        let criteria: Vec<skim::prelude::RankCriteria> = rank_criteria
            .map_or_else(
                || {
                    vec![
                        RankCriteria::Score,
                        RankCriteria::Begin,
                        RankCriteria::End,
                        RankCriteria::Length,
                    ]
                },
                |c| c.to_vec(),
            )
            .into_iter()
            .map(Into::into)
            .collect();

        criteria
    }

    pub fn engine(
        &self,
        pattern: &str,
        exact_mode: bool,
        rank_criteria: Option<Vec<RankCriteria>>,
    ) -> Box<dyn MatchEngine> {
        let criteria = Self::rank_builder_criteria(rank_criteria.as_deref());
        let rank_builder = RankBuilder::new(criteria);

        match self {
            Self::Fuzzy => ExactOrFuzzyEngineFactory::builder()
                .exact_mode(exact_mode)
                .rank_builder(Arc::new(rank_builder))
                .fuzzy_algorithm(skim::FuzzyAlgorithm::default())
                .build()
                .create_engine(pattern),

            Self::Regex => RegexEngineFactory::builder()
                .rank_builder(Arc::new(rank_builder))
                .build()
                .create_engine(pattern),
        }
    }
}
