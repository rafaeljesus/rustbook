// Time: O(n^2) | Space: O(1)
// the outer while loop will run once for each item in the list,
// and the inner for loop will also run once for each item in the list.
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

// Time: O(n logn) | Space: O(n)
pub fn merge_sort(arr: &mut [i32]) {
    if arr.len() > 1 {
        let mid = arr.len() / 2;
        // sort the left half
        merge_sort(&mut arr[..mid]);
        // sort the right half
        merge_sort(&mut arr[mid..]);
        // merge sorted halfs
        merge(arr, mid);
    }
}

fn merge(arr: &mut [i32], mid: usize) {
    let mut left_index = 0;
    let mut right_index = mid;
    let mut temp = Vec::with_capacity(arr.len());

    while left_index < mid && right_index < arr.len() {
        if arr[left_index] <= arr[right_index] {
            temp.push(arr[left_index]);
            left_index += 1;
        } else {
            temp.push(arr[right_index]);
            right_index += 1;
        }
    }

    if left_index < mid {
        temp.extend_from_slice(&arr[left_index..mid]);
    } else {
        temp.extend_from_slice(&arr[right_index..]);
    }

    arr.copy_from_slice(&temp);
}

// Time: O(n logn) | Space: O(n)
pub fn merge_sort2(arr: &mut [i32], s: usize, e: usize) {
    if e - s + 1 <= 1 {
        return;
    }
    let m = (s + e) / 2;
    merge_sort2(arr, s, m);
    merge_sort2(arr, m + 1, e);
    merge2(arr, s, m, e);
}

fn merge2(arr: &mut [i32], s: usize, m: usize, e: usize) {
    let mut temp = Vec::new();
    let mut i = s;
    let mut j = m + 1;

    while i <= m && j <= e {
        if arr[i] <= arr[j] {
            temp.push(arr[i]);
            i += 1;
        } else {
            temp.push(arr[j]);
            j += 1;
        }
    }

    while i <= m {
        temp.push(arr[i]);
        i += 1;
    }

    while j <= e {
        temp.push(arr[j]);
        j += 1;
    }

    for (k, v) in temp.iter().enumerate() {
        arr[s + k] = *v;
    }
}

// Time: O(n^2) | Space: O(1)
pub fn selection_sort(arr: &mut [i32]) {
    for i in 1..arr.len() {
        let mut j = i - 1;
        while j > 0 && arr[j + 1] < arr[j] {
            arr.swap(j + 1, j);
            j -= 1;
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

    #[test]
    fn test_merge_sort() {
        let mut arr = [5, 2, 4, 6, 1, 3];
        merge_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6]);

        let mut arr = [5, 2, 4, 6, 1, 3];
        let len = arr.len() - 1;
        merge_sort2(&mut arr, 0, len);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_selection_sort() {
        let mut arr = [5, 2, 4, 6, 1, 3];
        selection_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6]);
    }
}
