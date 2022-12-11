// Time: O(n^2) | Space: O(1)
// the outer while loop will run once for each item in the list,
// and the inner for loop will also run once for each item in the list.
pub fn bubble_sort(nums: &mut Vec<i32>) {
    let mut swapped = true;
    // keep looking until no more swaps are needed
    while swapped {
        swapped = false;
        for i in 0..nums.len() - 1 {
            // if the current number is greater than the next,
            // then swap 
            if nums[i] > nums[i + 1] {
                nums.swap(i, i + 1);
                swapped = true;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        let mut nums: Vec<i32> = vec![3, 1, 7, 5];
        let expected: Vec<i32> = vec![1, 3, 5, 7];
        bubble_sort(&mut nums);
        assert_eq!(nums, expected);
    }
}
