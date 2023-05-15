use rand::Rng;

use crate::partition_in_place;

// /* Quicksort orders the unordered list in-place */
/*
        1. Choose a pivot element: Select an element from the array as the pivot.
        2. Partition the array
        3. Recursively sort the left and right partitions until the base case is reached, which is when the partition contains only one element or no elements at all
        4. Combine the results (if in_place, then no need to combine results)
 */
pub fn quicksort<T: PartialOrd + Copy> (slice: &mut [T]) {
        let len = slice.len();
        if len <= 1 { return } /* base case */
        let mut pivot_index = rand::thread_rng().gen_range(0..slice.len());
        pivot_index = partition_in_place(slice, pivot_index); /* after partitioning, [Left | Pivot | Right] */
        quicksort(&mut slice[0..pivot_index]); /* recurse left */
        quicksort(&mut slice[pivot_index+1..]); /* recurse right */
}

#[cfg(test)]
mod tests {
        use super::*;

        #[test]
        fn it_works() {
                let ref mut slice = vec![1,0,2,3,4,5,6,7];
                quicksort(slice);
                assert_eq!(vec![0,1,2,3,4,5,6,7], *slice);

                let ref mut slice = vec!['d', 'c', 'b', 'a'];
                quicksort(slice);
                assert_eq!(vec!['a', 'b', 'c', 'd'], *slice);
        }
}