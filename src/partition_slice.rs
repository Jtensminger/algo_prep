/* 
What is an Array Partition? 
Reordering a list by splitting it into two parts around a pivot element based on a condition/comparison:
        (1) a lower half of items matching
        (2) an upper half not matching
*/

use std::cmp::PartialOrd; /* items in list must have a Partial Order since this a comparison-based algorithm */
pub fn partition_in_place<T: PartialOrd + Copy> (slice: &mut [T], pivot_index: usize) -> usize {
	let pivot = slice[pivot_index];
	slice.swap(0, pivot_index);
	let mut left_boundary = 1;

	for j in 1..slice.len() {
		if slice[j] < pivot  {
			slice.swap(left_boundary, j);
			left_boundary += 1;
		}
	}
	left_boundary -= 1;
	slice.swap(left_boundary, 0);
	left_boundary
}

#[cfg(test)]
mod tests {
        use super::*;

        #[test]
        fn it_works() {
                let mut slice = vec!["D", "A", "H", "C"];
		let pivot_position = partition_in_place(&mut slice, 3); /* pivot = "C" */ 
                assert_eq!(vec!["A", "C", "H", "D"], slice);
		assert_eq!(pivot_position, 1);
        }

        #[test]
        fn it_works_2() {
                let mut slice = vec!["E", "D", "C", "B", "A"];
		let pivot_position = partition_in_place(&mut slice, 3); /* pivot = "B" */ 
                assert_eq!(vec!["A", "B", "C", "E", "D"], slice); /* all items to the left of the pivot are in partial order, not total order */
		assert_eq!(pivot_position, 1);
        }
}