use std::cmp::Ordering;
use crate::partition_in_place;
use rand::Rng;

// /* QuickSelect finds the k-th smallest element in an unordered list. */
pub fn quick_select<T: PartialOrd + Copy> (slice: &mut [T], k_smallest: usize) -> &T {
        if slice.len() == 1 { return &slice[0] }
        let mut pivot_index = rand::thread_rng().gen_range(0..slice.len());
        // let mut pivot_index = 0;
        pivot_index = partition_in_place(slice, pivot_index); /* after partitioning, [Left | Pivot | Right] */
        return match k_smallest.cmp(&pivot_index) {  /* after partioning, we can now tell if k lives on the left or right side of the slice */
                Ordering::Equal => &slice[k_smallest],                        /* Success! */
                /* When the k-th smallest element is in the left partition,
                         you don't need to change the value of k_smallest because the indices in the left partition are the same as the original array. */
                Ordering::Less => quick_select(&mut slice[..pivot_index], k_smallest),    /* [k_smallest is Left | Pivot ] */ 
                /* However, when the k-th smallest element is in the right partition,
                        you need to adjust the value of k_smallest because the right partition is now a new slice with different indices.
                        Specifically, the new slice starts at the index pivot_index + 1 of the original array,
                        so the relative index of the k-th smallest element in the right partition will be different from its index in the original array. */
                Ordering::Greater => quick_select(&mut slice[pivot_index + 1..], k_smallest - pivot_index - 1), /* [ Pivot | k_smallest is Right ] */
        }
}

#[cfg(test)]
mod tests {
        use super::*;

        #[test]
        fn it_works() {
                let ref mut slice = vec![7,6,5,4,3,2];
                assert_eq!(&2, quick_select(slice, 0));
                assert_eq!(&3, quick_select(slice, 1));
                assert_eq!(&4, quick_select(slice, 2));
                assert_eq!(&5, quick_select(slice, 3));
        }
}


/* 
// High-level Logic:
        1) Choose a pivot element from the list (often chosen randomly or using a specific strategy).
        2) Partition the list around the pivot, such that elements smaller than the pivot come before it, and elements larger than the pivot come after it.
        3) Depending on the position of the pivot and the desired k-th element, recursively perform steps 1 and 2 on the appropriate partition.
*/


// use rand::Rng;

// /* Partition: reordering the list around the pivot element
//         split the array in two parts: => [ less_than | pivot | greater_than ] 
//                 the lower half with objects matching the condition,
//                 the upper half with objects not matching the condition
// */

// fn partition<T: Ord>(slice: &mut [T], low: usize, high: usize, pivot_index: usize) -> usize {
//         // Swap the pivot element with the last element in the list.
//         slice.swap(pivot_index, high); 
//         // Temporary pivot index
//         let mut i = low; 
//         for j in low..high {
//                 // If the current element is less than or equal to the pivot
//                 if slice[j] <= slice[high] { 
//                         // Swap the current element with the element at the temporary pivot index
//                         slice.swap(i, j); 
//                         // Move the temporary pivot index forward
//                         i += 1;  
//                 }
//         }
//         // Move the pivot element to the correct pivot position (between the smaller and larger elements)
//         slice.swap(i, high);
//         // the pivot index
//         i 
// }

// fn quick_select_recurse<T: Ord + Clone>(list: &mut [T], k: usize) -> T {
//         if list.len() == 1 {
//                 return list[0].clone();
//         }
//         let pivot_index = rand::thread_rng().gen_range(0..list.len());
//         let pivot_new_index = partition(list, 0, list.len() - 1, pivot_index);
//         if k == pivot_new_index {
//                 list[pivot_new_index].clone()
//         } else if k < pivot_new_index {
//                 quick_select_recurse(&mut list[0..pivot_new_index], k)
//         } else {
//                 quick_select_recurse(&mut list[pivot_new_index + 1..], k - pivot_new_index - 1)
//         }
// }

// pub fn quick_select<T: Ord + Clone>(list: &mut [T], k: usize) -> T {
//         quick_select_recurse(list, k)
// }