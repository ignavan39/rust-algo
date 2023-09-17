use std::collections::BTreeMap;


pub fn find_left_max_occurrences_column(arr: Vec<Vec<i32>>) -> i32 {
    let mut map = BTreeMap::<i32, i32>::new();
    for (_, x) in arr.iter().enumerate() {
        for (ii, _) in x.iter().enumerate() {
            let count = map.get(&x[ii]);

            if count.is_none() {
                map.insert(x[ii], 0);
            } else {
                map.insert(x[ii], *count.unwrap() + 1);
            }
        }
    }

    return *map
        .iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .map(|(k, _v)| k)
        .unwrap();
}
