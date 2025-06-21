use std::collections::HashMap;

use orx_composable::*;

// reduction

struct And;

impl Reduction for And {
    type Out = bool;

    fn identity(&self) -> Self::Out {
        true
    }

    fn reduce(&self, a: Self::Out, b: Self::Out) -> Self::Out {
        a && b
    }
}

// rules for healthy stock levels

struct SufficientCurrentInventoryLevels {
    required_minimum_per_sku: HashMap<String, u64>,
}

// impl Computation<And> for SufficientCurrentInventoryLevels {
//     type In = &[(String, u64)];

//     fn compute(&self, reduction: &And, input: Self::In) -> <And as Reduction>::Out {
//         todo!()
//     }
// }

fn main() {}
