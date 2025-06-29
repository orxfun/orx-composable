use crate::swap::Swap;
use std::{ops::Index, vec};

pub struct Tour {
    sequence: Vec<usize>,
}

impl Index<usize> for Tour {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        &self.sequence[index]
    }
}

impl IntoIterator for Tour {
    type Item = usize;

    type IntoIter = vec::IntoIter<usize>;

    fn into_iter(self) -> Self::IntoIter {
        self.sequence.into_iter()
    }
}

impl<'a> IntoIterator for &'a Tour {
    type Item = &'a usize;

    type IntoIter = core::slice::Iter<'a, usize>;

    fn into_iter(self) -> Self::IntoIter {
        self.sequence.iter()
    }
}

impl Tour {
    pub fn apply_swap(&mut self, swap: Swap) {
        self.sequence.swap(swap.pos1, swap.pos2);
    }
}
