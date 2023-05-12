use std::cmp::PartialOrd;

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
}