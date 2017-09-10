use num::{One, Num};
use std::collections::BinaryHeap;
use algo::edm_x::heap::{MaxHeap, MaxHeapItem, MinHeap, MinHeapItem};

use errors::*;

enum HeapItem<T: Ord> {
    MinHeap(MinHeapItem<T>),
    MaxHeap(MaxHeapItem<T>),
    FirstPush(T),
}

#[derive(Clone, Debug)]
enum HeapSizeInfo {
    MinHeapBigger,
    MaxHeapBigger,
    EqualSizes,
    Empty,
}

struct Heaps<T: Ord + Num + One + Clone> {
    min_heap: MinHeap<T>,
    max_heap: MaxHeap<T>,
    heap_size_info: HeapSizeInfo,
}

impl<T: Ord + Num + One + Clone> Heaps<T> {
    fn new() -> Self {
        let min_heap: MinHeap<T> = BinaryHeap::new();
        let max_heap: MaxHeap<T> = BinaryHeap::new();
        Heaps {
            min_heap: min_heap,
            max_heap: max_heap,
            heap_size_info: HeapSizeInfo::Empty,
        }
    }

    fn push_to_heap(&mut self, push_item: HeapItem<T>) -> () {
        self.heap_size_info = match (&self.heap_size_info, push_item) {
            (&HeapSizeInfo::Empty, HeapItem::FirstPush(item)) => {
                self.min_heap.push(MinHeapItem(item.clone()));
                self.max_heap.push(MaxHeapItem(item));
                HeapSizeInfo::EqualSizes
            }
            (&HeapSizeInfo::EqualSizes, HeapItem::MinHeap(item)) => {
                self.min_heap.push(item);
                HeapSizeInfo::MinHeapBigger
            }
            (&HeapSizeInfo::EqualSizes, HeapItem::MaxHeap(item)) => {
                self.max_heap.push(item);
                HeapSizeInfo::MaxHeapBigger
            }
            (&HeapSizeInfo::MaxHeapBigger, HeapItem::MinHeap(item)) => {
                self.min_heap.push(item);
                HeapSizeInfo::EqualSizes
            }
            (&HeapSizeInfo::MinHeapBigger, HeapItem::MaxHeap(item)) => {
                self.max_heap.push(item);
                HeapSizeInfo::EqualSizes
            }
            (&HeapSizeInfo::MinHeapBigger, HeapItem::MinHeap(item)) => {
                self.min_heap.push(item);
                if let Some(MinHeapItem(value)) = self.min_heap.pop() {
                    self.max_heap.push(MaxHeapItem(value));
                    HeapSizeInfo::EqualSizes
                } else {
                    unimplemented!("Impossible -- a value was just pushed to the min_heap.")
                }
            }
            (&HeapSizeInfo::MaxHeapBigger, HeapItem::MaxHeap(item)) => {
                self.max_heap.push(item);
                if let Some(MaxHeapItem(value)) = self.max_heap.pop() {
                    self.min_heap.push(MinHeapItem(value));
                    HeapSizeInfo::EqualSizes
                } else {
                    unimplemented!("Impossible -- a value was just pushed to the max_heap.")
                }
            }
            _ => {
                unimplemented!(
                    "Impossible -- BothHeaps is only called when heaps are empty, and vice-versa."
                )
            }
        };
    }

    fn add_to_heaps(&mut self, value: T) -> () {
        let heap_item_to_push = match &self.heap_size_info {
            &HeapSizeInfo::Empty => HeapItem::FirstPush(value),
            _ if &value <= &self.min_heap.peek()
                .expect("Must be Some -- previous pattern match with Empty ensures a value is present in both heaps.").0 =>
                HeapItem::MaxHeap(MaxHeapItem(value)),
            _ => HeapItem::MinHeap(MinHeapItem(value)),
        };
        self.push_to_heap(heap_item_to_push);
    }

    fn get_median(&self) -> T {
        match &self.heap_size_info {
            &HeapSizeInfo::Empty => {
                unimplemented!(
                    "get_median is never called in the EDM-X algorithm before a value is pushed to the heaps."
                )
            }
            &HeapSizeInfo::MinHeapBigger => {
                self.min_heap
                    .peek()
                    .expect("Item must be Some because HeapSizeInfo is not Empty")
                    .0
                    .clone()
            }
            &HeapSizeInfo::MaxHeapBigger => {
                self.max_heap
                    .peek()
                    .expect("Item must be Some because HeapSizeInfo is not Empty")
                    .0
                    .clone()
            }
            &HeapSizeInfo::EqualSizes => {
                let min_heap_value = self.min_heap
                    .peek()
                    .expect("Item must be Some because HeapSizeInfo is not Empty")
                    .0
                    .clone();
                let max_heap_value = self.max_heap
                    .peek()
                    .expect("Item must be Some because HeapSizeInfo is not Empty")
                    .0
                    .clone();
                (min_heap_value + max_heap_value) / (T::one() + T::one())
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct BestCandidate<T: Ord> {
    pub statistic: T,
    pub location: usize,
}

fn inner_edm_x_loop<'a, T, I>(
    left_median: T,
    delta: usize,
    z_from_i: I,
    i: usize,
) -> BestCandidate<T>
where
    T: Ord + Clone + Num + One + From<f64> + 'a,
    I: Iterator<Item = &'a T>,
{
    let mut right_heaps: Heaps<T> = Heaps::new();
    let mut best_candidate: Option<BestCandidate<T>> = None;
    let mut starting_state = (&mut right_heaps, &mut best_candidate);
    z_from_i
        .enumerate()
        .scan(starting_state, move |next_state, (jmi, next_item)| {
            let &mut (ref mut right_heaps, ref mut best_candidate) = next_state;
            right_heaps.add_to_heaps(next_item.clone());
            if jmi < delta {
                Some(None)
            } else {
                let j = jmi + i;
                let j_float = j as f64;
                let i_float = i as f64;
                let right_median = right_heaps.get_median();
                let median_diff = left_median.clone() - right_median;
                let median_diff_squared = median_diff.clone() * median_diff;
                let stat_weight = (i_float * (j_float - i_float)) / j_float;
                Some(Some(BestCandidate {
                    statistic: T::from(stat_weight) * median_diff_squared,
                    location: j,
                }))
            }
        })
        .filter_map(|result| result)
        .max()
        .expect("filter_map ensures result is Some")
}

pub fn edm_x<T>(z: &[T], delta: usize) -> BestCandidate<T>
where
    T: Ord + Clone + Num + One + From<f64>
{
    let mut left_heaps: Heaps<T> = Heaps::new();
    let mut best_candidate: Option<BestCandidate<T>> = None;
    let mut starting_state = (&mut left_heaps, &mut best_candidate);
    z.iter().take(z.len() - delta)
        .enumerate()
        .scan(starting_state, move |next_state, (i, next_item)| {
            let &mut (ref mut left_heaps, ref mut best_candidate) = next_state;
            left_heaps.add_to_heaps(next_item.clone());
            if i < delta {
                Some(None)
            } else {
                let left_median = left_heaps.get_median();
                Some(Some(inner_edm_x_loop(left_median, delta, z.iter().skip(i), i)))
            }
        })
        .filter_map(|result| result)
        .max()
        .expect("filter_map ensures result is Some")
}

#[cfg(test)]
mod tests {
    use super::*;
    use algo::non_nan::{to_non_nans, NonNaN};

    #[test]
    fn heaps_find_the_median() {
        let initial_number: NonNaN<f32> = NonNaN::new(1.0).unwrap();
        let mut heaps: Heaps<NonNaN<f32>> = Heaps::new();
        heaps.add_to_heaps(initial_number.clone());
        assert_eq!(heaps.get_median(), initial_number.clone());
        heaps.add_to_heaps(NonNaN::new(2.0).unwrap());
        heaps.add_to_heaps(NonNaN::new(3.0).unwrap());
        heaps.add_to_heaps(NonNaN::new(4.0).unwrap());
        heaps.add_to_heaps(NonNaN::new(5.0).unwrap());
        heaps.add_to_heaps(NonNaN::new(6.0).unwrap());
        heaps.add_to_heaps(NonNaN::new(7.0).unwrap());
        heaps.add_to_heaps(NonNaN::new(8.0).unwrap());
        assert_eq!(heaps.get_median(), NonNaN::new(4.0).unwrap());
    }
}
