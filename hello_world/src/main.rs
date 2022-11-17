use std::collections::HashMap;
use std::{cmp, i32};

fn main() {
    // binary search
    let nums = vec![10, 20, 30, 40, 50, 60, 70];
    let value = binary_search(nums, 50);
    assert_eq!(value, 50);

    // min
    let nums = vec![4, 2, 6, 8, 9, 23, 43, 24, 1, 0];
    let value = min(nums);
    assert_eq!(value, 0);

    // max
    let nums = vec![4, 2, 6, 8, 9, 23, 43, 24, 1, 0];
    let value = max(nums);
    assert_eq!(value, 43);

    // median
    let nums = vec![1, 2, 3, 4, 5, 6, 7];
    let value = median(nums);
    assert_eq!(value, 4);

    // lightstep teams
    let ingest = vec!["foo", "bar", "foobar", "barz", "fooh"];
    let mut teams = HashMap::new();
    teams.insert(String::from("ingest"), ingest);

    match teams.get("ingest") {
        Some(peeps) => assert_eq!(peeps.len(), 5),
        None => panic!(),
    }
}

fn binary_search(nums: Vec<i32>, target: i32) -> i32 {
    let mut left = 0;
    let mut right = nums.len() - 1;

    while left <= right {
        let mid = (left + right) / 2;
        let guess = Some(nums[mid]);

        match guess {
            Some(x) if x == target => return x,
            Some(x) if x < target => left += 1,
            _ => right -= 1,
        }
    }
    0
}

fn min(nums: Vec<i32>) -> i32 {
    let mut min_value = i32::MAX;
    for n in nums {
        min_value = cmp::min(n, min_value);
    }
    min_value
}

fn max(nums: Vec<i32>) -> i32 {
    let mut max_value = i32::MIN;
    for n in nums {
        max_value = cmp::max(n, max_value);
    }
    max_value
}

fn median(nums: Vec<i32>) -> i32 {
    nums[nums.len() / 2]
}
