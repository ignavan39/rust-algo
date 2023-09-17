#[path = "./algo/find_left_max_occurrences_column.rs"]
mod algo;
#[path = "./utils/vector_from_file.rs"]
mod utils;

fn main(){}

#[test]
fn find_left_max_occurrences_column_test() {
    let lines = utils::vector_from_file("test1.txt");

    let result = algo::find_left_max_occurrences_column(lines);
    assert!(result == 99);
}
