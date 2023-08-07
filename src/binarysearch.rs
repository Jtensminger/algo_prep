/* item not in slice == Err(idx) */
/* item found == Ok(idx) */
fn binary_search<T: Ord + Copy> (slice: &[T], target: &T) -> Result<usize, usize> {
        binary_search_helper(slice, target, 0)
}
fn binary_search_helper<T: Ord + Copy> (slice: &[T], target: &T, offset: usize) -> Result<usize, usize> {
        if slice.is_empty() { /* target not found */
                return Err(offset)
        } else { /* divide and conquer */
                let mid = slice.len() / 2;
                if &slice[mid] < target {
                        binary_search_helper(&slice[mid+1..], &target, offset + mid + 1)
                } else if &slice[mid] > target {
                        binary_search_helper(&slice[..mid], &target, offset)
                } else {
                        Ok(offset + mid)
                }
        }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn it_works_ok() {
        assert_eq!(Ok(1), binary_search(&vec![1,2,3,4,5], &2));
        assert_eq!(Ok(3), binary_search(&vec![1,2,3,4,5], &4));
    }

    #[test]
    fn it_works_err() {
        assert_eq!(Err(0), binary_search(&vec![1,2,3,4,5], &0));
        assert_eq!(Err(5), binary_search(&vec![1,2,3,4,5], &6));
    }
}
