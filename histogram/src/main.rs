use std::collections::HashMap;

// print a histogram with the latency in milliseconds from a given input.
// The input is an array with latency values in milliseconds,
// i.e., [102, 200, 202, 202, 204, 409, 407, 404, 501, 502, 1023] i.e., 102ms, 200ms, 204ms, etc.
fn main() {
    let latency_list = vec![102, 200, 202, 202, 204, 400, 400, 404, 500, 502, 1000];
    let mut buckets_map = HashMap::from([
        (100, String::from("")),
        (200, String::from("")),
        (300, String::from("")),
        (400, String::from("")),
        (500, String::from("")),
        (600, String::from("")),
        (700, String::from("")),
        (800, String::from("")),
        (900, String::from("")),
        (1000, String::from("")),
    ]);

    for price in latency_list {
        for (bucket, hist) in buckets_map.iter_mut() {
            if &price >= bucket && price < bucket + 10 {
                *hist = format!("{}{}", hist, "*");
                break;
            }
        }
    }

    let mut ordered_map: Vec<_> = buckets_map.iter().collect();
    ordered_map.sort_by(|a, b| a.cmp(b));

    for (bucket, hist) in ordered_map {
        println!("{bucket}: {hist}");
    }
}
