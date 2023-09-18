use std::collections::BTreeMap;


pub fn find_left_max_occurrences_column(arr: Vec<Vec<i32>>) -> i32 {
    let mut map = BTreeMap::<i32, i32>::new();
    for x in arr.iter() {
        for (ii, _) in x.iter().enumerate() {
            let count = map.get(&x[ii]);

            match count {
                None => {
                    map.insert(x[ii], 0);
                },
                Some(res) => {
                    map.insert(x[ii], *res + 1);
                }
            }
        }
    }

    return  *map
        .iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .map(|(k, _v)| k)
        .unwrap();
}
