// Time: O(n^2) | Space: O(1)
pub fn bubble_sort(nums: &mut Vec<i32>) {
    let n = nums.len();
    let mut sorted = true;
    while sorted {
        sorted = false;
        for i in 0..n - 1 {
            if nums[i] > nums[i + 1] {
                // swap if the current number is greater than the next
                nums.swap(i, i + 1);
                sorted = true;
            }
        }
    }
}

// Time: O(n^2) | Space: O(1)
// the outer while loop will run once for each item in the list,
// and the inner for loop will also run once for each item in the list.
pub fn bubble_sort2(nums: &mut Vec<i32>) {
    let n = nums.len();
    for i in 0..n - 1 {
        for j in 0..n - i - 1 {
            if nums[j] > nums[j + 1] {
                // swap if the current number is greater than the next
                nums.swap(i, j + 1);
            }
        }
    }
}

/*
The quick sort algorithm is a divide-and-conquer algorithm that sorts a given array by partitioning it into two sub-arrays,
one with elements smaller than a pivot element and the other with elements greater than the pivot element.
The algorithm then recursively sorts the sub-arrays until the array is fully sorted.
In the given example implementation, the partition() function takes the array, a low index,
and a high index as input and returns the index of the pivot element.
It first selects the last element in the array as the pivot element and initializes a counter i to the low index.
It then iterates over the elements in the array from low to high and swaps any element that is smaller than the pivot element with the element at the i-th index.
Finally, it swaps the pivot element with the element at the i-th index and returns the index of the pivot element.
The quick_sort() function takes the array, a low index, and a high index as input and sorts the array between the low and high indices using the partition() function.
It checks if the low index is less than the high index and recursively sorts the sub-arrays to the left and right of the pivot element.
*/
pub fn quick_sort(arr: &mut Vec<i32>, low: usize, high: usize) {
    if low < high {
        let pivot_index = partition(arr, low, high);
        quick_sort(arr, low, pivot_index - 1);
        quick_sort(arr, pivot_index + 1, high);
    }
}

// Function to partition the array based on the pivot element
fn partition(arr: &mut Vec<i32>, low: usize, high: usize) -> usize {
    let pivot = arr[high];
    let mut i = low;

    for j in low..high {
        if arr[j] < pivot {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, high);
    i
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

    #[test]
    fn test_quick_sort() {}
}
