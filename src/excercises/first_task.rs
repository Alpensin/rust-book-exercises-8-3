// Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) and
// mode (the value that occurs most often; a hash map will be helpful here) of the list.

use std::collections::HashMap;

pub fn find_median(ints: &mut Vec<i32>) -> i32 {
    ints.sort();
    let n = ints.len();
    if n % 2 == 0 {
        let mid = n / 2;
        return (ints[mid - 1] + ints[mid]) / 2;
    } else {
        return ints[n / 2];
    }
}

pub fn find_most_often(ints: &Vec<i32>) -> i32 {
    let mut res = HashMap::new();

    for i in ints {
        let count = res.entry(i).or_insert(0);
        *count += 1;
    }
    let mut max_count = 0;
    let mut key_name = 0;

    for (k, v) in &res {
        if v > &max_count {
            max_count = *v;
            key_name = **k;
        }
    }
    return key_name;
}
