pub struct Cost(usize);

impl From<usize> for Cost {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

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
