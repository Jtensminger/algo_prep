// /* QuickSelect finds the k-th smallest element in an unordered list. */




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



// #[cfg(test)]
// mod tests {
//         use super::*;

//         #[test]
//         fn it_works() {
//                 let mut slice = vec![6,5,3,2,4,0,7];
//                 let (low, high) = (0, slice.len()-1);
//                 partition(&mut slice, low, high, high);
//                 // assert_eq!(vec![0,2,3,7,4,6,5], slice);
//                 dbg!(slice);
//         }
// }


// /* 
// High-level Logic:
//         1) Choose a pivot element from the list (often chosen randomly or using a specific strategy).
//         2) Partition the list around the pivot, such that elements smaller than the pivot come before it, and elements larger than the pivot come after it.
//         3) Depending on the position of the pivot and the desired k-th element, recursively perform steps 1 and 2 on the appropriate partition.

// Notes:
//         It's very useful in finding the k-th smallest element without sorting the entire list.
//         Common Use Cases:
//                 Finding the median of a dataset,
//                 Selecting top-k elements in a ranking system, or
//                 Calculating percentiles in statistical analysis.
// */