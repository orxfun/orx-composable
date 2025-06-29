use crate::{cost::Cost, tour::Tour};
use orx_composable::Computation;
use orx_iterable::Collection;
use std::collections::HashSet;

pub struct PrecedenceRelations(HashSet<(usize, usize)>);

impl PrecedenceRelations {
    fn number_of_violations(&self, tour: Tour) -> usize {
        let mut count = 0;
        for (i, a) in tour.iter().copied().enumerate() {
            for b in tour.iter().copied().skip(i + 1) {
                if self.0.contains(&(b, a)) {
                    count += 1;
                }
            }
        }
        count
    }
}

pub struct PrecedenceCost {
    cost_per_violation: usize,
}

impl Computation for PrecedenceCost {
    type In<'i> = &'i PrecedenceRelations;

    type Out = Cost;

    fn compute<'i>(&self, precedence_relations: Self::In<'i>) -> Self::Out {
        todo!()
    }
}
