use orx_composable::*;
use std::collections::HashMap;

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

// rule #1: healthy stock levels

struct HealthyStockLevels {
    required_minimum_per_sku: HashMap<String, u64>,
}

impl Computation<And> for HealthyStockLevels {
    type In<'i>
        = &'i [(String, u64)]
    where
        Self: 'i,
        And: 'i;

    fn compute_reduce<'i>(&self, _: &And, stock_levels: Self::In<'i>) -> <And as Reduction>::Out
    where
        And: 'i,
    {
        stock_levels.iter().all(|(sku, current)| {
            self.required_minimum_per_sku
                .get(sku)
                .map(|minimum| current >= minimum)
                .unwrap_or(true)
        })
    }
}

// rule #2: no backlogs

struct BacklogAmount(u64);

struct NoBacklogs;

impl Computation<And> for NoBacklogs {
    type In<'i>
        = BacklogAmount
    where
        Self: 'i,
        And: 'i;

    fn compute_reduce<'i>(
        &self,
        _: &And,
        total_backlogged_items: Self::In<'i>,
    ) -> <And as Reduction>::Out
    where
        And: 'i,
    {
        total_backlogged_items.0 == 0
    }
}

// rule #3: no delayed orders

struct OrderStatus {
    sku: String,
    days_to_due_date: u32,
    expected_lead_time_days: u32,
}

struct NoDelayedOrders;

impl Computation<And> for NoDelayedOrders {
    type In<'i>
        = &'i [OrderStatus]
    where
        Self: 'i,
        And: 'i;

    fn compute_reduce<'i>(&self, _: &And, input: Self::In<'i>) -> <And as Reduction>::Out
    where
        And: 'i,
    {
        todo!()
    }
}

fn main() {}
