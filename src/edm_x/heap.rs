use std::collections::BinaryHeap;
use std::cmp::{Ordering, Ord};

#[derive(PartialEq, Eq, Debug)]
pub struct MinHeapItem<T: PartialEq + Eq + PartialOrd + Ord>(T);
#[derive(PartialEq, Eq, Debug)]
pub struct MaxHeapItem<T: PartialEq + Eq + PartialOrd + Ord>(T);

fn min_heap_cmp<T>(this: &MinHeapItem<T>, other: &MinHeapItem<T>) -> Ordering
    where
    T: PartialEq + Eq + PartialOrd + Ord
{
    let &MinHeapItem(ref this) = this;
    let &MinHeapItem(ref other) = other;
    this.cmp(other).reverse()
}

impl<T: Ord> PartialOrd for MinHeapItem<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(min_heap_cmp(self, other))
    }
}

impl<T: Ord> Ord for MinHeapItem<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        min_heap_cmp(self, other)
    }
}

fn max_heap_cmp<T>(this: &MaxHeapItem<T>, other: &MaxHeapItem<T>) -> Ordering
    where
    T: PartialEq + Eq + PartialOrd + Ord
{
    let &MaxHeapItem(ref this) = this;
    let &MaxHeapItem(ref other) = other;
    this.cmp(other)
}

impl<T: Ord> PartialOrd for MaxHeapItem<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(max_heap_cmp(self, other))
    }
}

impl<T: Ord> Ord for MaxHeapItem<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        max_heap_cmp(self, other)
    }
}

impl<T: Ord> From<T> for MaxHeapItem<T> {
    fn from(source: T) -> Self {
        MaxHeapItem(source)
    }
}

impl<T: Ord> From<T> for MinHeapItem<T> {
    fn from(source: T) -> Self {
        MinHeapItem(source)
    }
}

pub type MaxHeap<T> = BinaryHeap<MaxHeapItem<T>>;
pub type MinHeap<T> = BinaryHeap<MinHeapItem<T>>;

#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::RangeFull;

    #[test]
    fn heap_sort_works_on_min_heap() {
        let mut source_numbers = vec![4, 2, 3, 9, 1];
        let mut sorted_numbers = vec![1, 2, 3, 4, 9];
        let mut result_numbers = Vec::new();
        let mut min_heap: MinHeap<u32> = BinaryHeap::new();
        for source_number in source_numbers.drain(RangeFull) {
            min_heap.push(source_number.into());
        };
        while let Some(MinHeapItem(result_number)) = min_heap.pop() {
            result_numbers.push(result_number);
        };
        for (result_number, expected_number) in result_numbers.drain(RangeFull).zip(sorted_numbers.drain(RangeFull)) {
            assert_eq!(result_number, expected_number);
        };
    }

    #[test]
    fn max_heap_does_reverse_sort() {
        let mut source_numbers = vec![4, 2, 3, 9, 1];
        let mut sorted_numbers = vec![9, 4, 3, 2, 1];
        let mut result_numbers = Vec::new();
        let mut min_heap: MaxHeap<u32> = BinaryHeap::new();
        for source_number in source_numbers.drain(RangeFull) {
            min_heap.push(source_number.into());
        };
        while let Some(MaxHeapItem(result_number)) = min_heap.pop() {
            result_numbers.push(result_number);
        };
        for (result_number, expected_number) in result_numbers.drain(RangeFull).zip(sorted_numbers.drain(RangeFull)) {
            assert_eq!(result_number, expected_number);
        };
    }
}
