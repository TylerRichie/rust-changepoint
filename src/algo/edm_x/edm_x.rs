use num::{One, Num};
use std::collections::BinaryHeap;
use std::ops::{Index, RangeFrom};
use algo::edm_x::heap::{MaxHeap, MaxHeapItem, MinHeap, MinHeapItem};
use algo::best_candidate::BestCandidate;
use algo::changepoint::ChangePointDetector;

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

pub trait HeapNum: Ord + Num + One + Clone {}

impl<T: Ord + Num + One + Clone> HeapNum for T {}

struct Heaps<T: HeapNum> {
    min_heap: MinHeap<T>,
    max_heap: MaxHeap<T>,
    heap_size_info: HeapSizeInfo,
}

impl<T: HeapNum> Heaps<T> {
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

fn inner_edm_x_loop<'a, T, I>(
    left_median: T,
    delta: usize,
    z_from_i: I,
    i: usize,
) -> BestCandidate<T>
where
    T: HeapNum + From<f64> + 'a,
    I: Iterator<Item = &'a T>,
{
    let right_heaps: Heaps<T> = Heaps::new();
    z_from_i
        .enumerate()
        .scan(right_heaps, move |right_heaps, (jmi, next_item)| {
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
                let candidate = BestCandidate {
                    statistic: T::from(stat_weight) * median_diff_squared,
                    location: i,
                };
                Some(Some(candidate))
            }
        })
        .filter_map(|result| result)
        .max()
        .expect("filter_map ensures result is Some")
}

fn edm_x<T>(z: &[T], delta: usize) -> BestCandidate<T>
where
    T: HeapNum + From<f64>,
{
    let left_heaps: Heaps<T> = Heaps::new();
    z.iter()
        .take(z.len() - delta)
        .enumerate()
        .scan(left_heaps, move |left_heaps, (i, next_item)| {
            left_heaps.add_to_heaps(next_item.clone());
            if i < delta {
                Some(None)
            } else {
                let left_median = left_heaps.get_median();
                let inner_best_candidate = inner_edm_x_loop(
                    left_median,
                    delta,
                    z.index(RangeFrom { start: i }).iter(),
                    i,
                );
                Some(Some(inner_best_candidate))
            }
        })
        .filter_map(|result| result)
        .max()
        .expect("filter_map ensures result is Some")
}

#[derive(Clone, Debug)]
pub struct EDMX {
    delta: usize,
}

impl EDMX {
    pub fn new(delta: usize) -> Self {
        EDMX { delta: delta }
    }
}

impl<T: HeapNum + From<f64>> ChangePointDetector<T> for EDMX {
    fn find_candidate(&self, observations: &[T]) -> Result<BestCandidate<T>> {
        if observations.len() < self.delta * 2 {
            Err(
                ErrorKind::NotEnoughValues(observations.len(), self.delta).into(),
            )
        } else {
            Ok(edm_x(observations, self.delta))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use algo::non_nan::NonNaN;
    use rand::SeedableRng;
    use rand::distributions::{IndependentSample, Normal};
    use mersenne_twister::MersenneTwister;
    use num::abs;

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

    #[test]
    fn edm_x_on_central_tendency() {
        let before_change_count = 100;
        let after_change_count = 400;
        let delta = 10;
        let tolerance = 50;
        let mut rng: MersenneTwister = SeedableRng::from_seed(0x1234);
        let mut input: Vec<NonNaN<f64>> = Vec::new();
        let before_change_dist = Normal::new(10.0, 5.0);
        for _ in 0..before_change_count {
            input.push(
                NonNaN::new(before_change_dist.ind_sample(&mut rng)).unwrap(),
            );
        }
        let after_change_dist = Normal::new(30.0, 5.0);
        for _ in 0..after_change_count {
            input.push(NonNaN::new(after_change_dist.ind_sample(&mut rng)).unwrap());
        }
        let best_candidate = edm_x(&input, delta);
        let abs_loc_diff = abs(best_candidate.location as i64 - before_change_count as i64);
        assert!(abs_loc_diff < tolerance);
    }
}
