use orx_composable::Reduction;

pub struct Cost(usize);

pub struct AdditiveCost;

// impl Reduction for AdditiveCost {
//     type Unit = Cost;

//     fn identity(&self) -> Self::Unit {
//         Cost(0)
//     }

//     fn reduce(&self, a: Self::Unit, b: Self::Unit) -> Self::Unit {
//         Cost(a.0 + b.0)
//     }
// }
