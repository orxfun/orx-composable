// use orx_composable::{
//     compute_reduce::ReducibleComputation,
//     compute_reduce2::ReducibleComputation2,
//     compute_with_reduction::ComputeWithReduction,
//     type_sequence::{Many, One},
//     *,
// };
// use std::{any::type_name, collections::HashMap};

// // reduction

// #[derive(Default)]
// struct And;

// impl Reduce for And {
//     type Unit = bool;

//     fn identity(&self) -> Self::Unit {
//         true
//     }

//     fn reduce(&self, a: Self::Unit, b: Self::Unit) -> Self::Unit {
//         a && b
//     }
// }

// // rule #1: healthy stock levels

// struct SufficientStockLevels {
//     required_minimum_per_sku: HashMap<String, u64>,
// }

// impl Compute for SufficientStockLevels {
//     type In<'i> = &'i [(String, u64)];

//     type Out = bool;

//     fn compute(&self, stock_levels: Self::In<'_>) -> Self::Out {
//         stock_levels.iter().all(|(sku, current)| {
//             self.required_minimum_per_sku
//                 .get(sku)
//                 .map(|minimum| current >= minimum)
//                 .unwrap_or(true)
//         })
//     }
// }

// // rule #2: no backlogs

// struct BacklogAmount(u64);

// struct NoBacklogs;

// impl Compute for NoBacklogs {
//     type In<'i> = BacklogAmount;

//     type Out = bool;

//     fn compute(&self, total_backlogged_items: Self::In<'_>) -> Self::Out {
//         total_backlogged_items.0 == 0
//     }
// }

// // rule #3: no delayed orders

// #[derive(derive_new::new)]
// struct OrderStatus {
//     _sku: String,
//     days_to_due_date: u32,
//     expected_lead_time_days: u32,
// }

// struct NoDelayedOrders;

// impl Compute for NoDelayedOrders {
//     type In<'i> = &'i [OrderStatus];

//     type Out = bool;

//     fn compute(&self, orders: Self::In<'_>) -> Self::Out {
//         orders
//             .iter()
//             .all(|o| o.expected_lead_time_days <= o.days_to_due_date)
//     }
// }

// fn main() {
//     // define the rules for a healthy system

//     let sufficient_stock_levels = SufficientStockLevels {
//         required_minimum_per_sku: HashMap::from_iter([
//             (String::from("x"), 16),
//             (String::from("y"), 42),
//         ]),
//     };
//     let no_backlogs = NoBacklogs;
//     let no_delayed_orders = NoDelayedOrders;

//     let health_rules = Composable::new(And)
//         .compose(sufficient_stock_levels)
//         .compose(no_backlogs)
//         .compose(no_delayed_orders);

//     type C = ReducibleComputation2<
//         And,
//         ReducibleComputation2<
//             And,
//             ComputeWithReduction<And, SufficientStockLevels>,
//             ComputeWithReduction<And, NoBacklogs>,
//             Many<SufficientStockLevels, One<NoBacklogs>>,
//         >,
//         ComputeWithReduction<And, NoDelayedOrders>,
//         Many<SufficientStockLevels, Many<NoBacklogs, One<NoDelayedOrders>>>,
//     >;

//     type X<'i> = <C as ReducibleComputation>::ComputeSequence;
//     // type Y = usize;

//     println!("{:?}", type_name::<X>());

//     // use the defined health rules to get the health status with current state or inputs

//     let stock_levels = vec![(String::from("x"), 30), (String::from("y"), 42)];
//     let backlog_amount = BacklogAmount(0);
//     let orders = vec![
//         OrderStatus::new(String::from("x"), 5, 1),
//         OrderStatus::new(String::from("y"), 3, 2),
//         OrderStatus::new(String::from("x"), 7, 7),
//     ];
//     let input = health_rules
//         .input_builder()
//         .compose(stock_levels.as_slice())
//         .compose(backlog_amount)
//         .compose(orders.as_slice());
//     let health_status = health_rules.compute(input.value());
//     assert!(health_status);

//     // this time get the health status with the new state

//     let stock_levels = vec![(String::from("x"), 30), (String::from("y"), 42)];
//     let backlog_amount = BacklogAmount(0);
//     let orders = vec![
//         OrderStatus::new(String::from("x"), 4, 0),
//         OrderStatus::new(String::from("y"), 2, 1),
//         OrderStatus::new(String::from("x"), 6, 7),
//     ];
//     let input = health_rules
//         .input_builder()
//         .compose(stock_levels.as_slice())
//         .compose(backlog_amount)
//         .compose(orders.as_slice());
//     let health_status = health_rules.compute(input.value());
//     assert!(!health_status);
// }

// // generic health check

// // struct HealthCheck<C: Computation<And>>(Composable<And, C>);
fn main() {}
