mod algo;
mod utils;

use algo::find_left_max_occurrences_column::find_left_max_occurrences_column;
use utils::vector_from_file::vector_from_file;


fn main(){}

#[test]
fn find_left_max_occurrences_column_test() {
    let lines = vector_from_file("test1.txt");

    let result = find_left_max_occurrences_column(lines);
    assert!(result == 99);
}
