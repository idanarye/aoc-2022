use std::collections::VecDeque;
use std::ops::Add;

struct BfsCell<P, C> {
    #[allow(unused)]
    parent: Option<P>,
    cost: C,
}

pub struct LinearBfs<C> {
    cells: Vec<Option<BfsCell<usize, C>>>,
    consider: VecDeque<usize>,
}

impl<C> LinearBfs<C>
where
    for<'a> &'a C: Add<&'a C, Output = C>,
    C: PartialOrd<C>,
{
    pub fn new(size: usize) -> Self {
        Self {
            cells: std::iter::repeat_with(|| None).take(size).collect(),
            consider: VecDeque::new(),
        }
    }

    pub fn cost(&self, key: usize) -> Option<&C> {
        self.cells[key].as_ref().map(|cell| &cell.cost)
    }

    pub fn add_root(&mut self, key: usize, cost: C) {
        self.cells[key] = Some(BfsCell { parent: None, cost });
        self.consider.push_back(key)
    }

    pub fn consider_next(&mut self) -> Option<usize> {
        self.consider.pop_front()
    }

    pub fn add_edge(&mut self, parent: usize, key: usize, additional_cost: C) -> bool {
        let new_cost = self.cost(parent).unwrap() + &additional_cost;
        if let Some(existing_cell) = &self.cells[key] {
            // TODO: this may fail because I'm using VecDeque instead of a BinaryHeap
            if new_cost < existing_cell.cost {
                assert!(self.consider.contains(&key), "out of order");
            } else {
                return false;
            }
        }
        self.consider.push_back(key);
        self.cells[key] = Some(BfsCell {
            parent: Some(parent),
            cost: new_cost,
        });
        true
    }
}
