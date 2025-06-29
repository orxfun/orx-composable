use crate::swap::Swap;
use std::cmp::Ordering;

pub struct SortedIntersectingSwaps<Iter1, Iter2> {
    iter1: Iter1,
    iter2: Iter2,
}

enum IterationResult {
    OneIteratorConsumed,
    BothYieldedSameValue(Swap),
    FirstIteratorYieldedSmaller(Swap),
    FirstIteratorYieldedGreater(Swap),
}

fn iter_result(value1: Option<Swap>, value2: Option<Swap>) -> IterationResult {
    match value1.and_then(|x| value2.map(|y| (x, y))) {
        Some((x, y)) => match x.cmp(&y) {
            Ordering::Equal => IterationResult::BothYieldedSameValue(x),
            Ordering::Greater => IterationResult::FirstIteratorYieldedGreater(x),
            Ordering::Less => IterationResult::FirstIteratorYieldedSmaller(y),
        },
        None => IterationResult::OneIteratorConsumed,
    }
}

impl<Iter1, Iter2> SortedIntersectingSwaps<Iter1, Iter2>
where
    Iter1: Iterator<Item = Swap>,
    Iter2: Iterator<Item = Swap>,
{
    fn iterate_first(&mut self, value2: Swap) -> IterationResult {
        let value1 = self.iter1.next();
        iter_result(value1, Some(value2))
    }

    fn iterate_second(&mut self, value1: Swap) -> IterationResult {
        let value2 = self.iter2.next();
        iter_result(Some(value1), value2)
    }

    fn handle_iteration_result(&mut self, mut iteration_result: IterationResult) -> Option<Swap> {
        loop {
            match iteration_result {
                IterationResult::OneIteratorConsumed => return None,
                IterationResult::BothYieldedSameValue(swap) => return Some(swap),
                IterationResult::FirstIteratorYieldedGreater(swap1) => {
                    iteration_result = self.iterate_second(swap1)
                }
                IterationResult::FirstIteratorYieldedSmaller(swap2) => {
                    iteration_result = self.iterate_first(swap2)
                }
            }
        }
    }
}

impl<Iter1, Iter2> Iterator for SortedIntersectingSwaps<Iter1, Iter2>
where
    Iter1: Iterator<Item = Swap>,
    Iter2: Iterator<Item = Swap>,
{
    type Item = Swap;

    fn next(&mut self) -> Option<Self::Item> {
        let (x, y) = (self.iter1.next(), self.iter2.next());
        self.handle_iteration_result(iter_result(x, y))
    }
}
