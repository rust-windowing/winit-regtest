

use std::path::Path;
use duct::cmd;

fn execute_test(test_name: &str) {
    let mut output_filename = String::from(test_name);
    output_filename.push_str(".txt");
    
    let prev_folder = Path::new("../tests-previous");
    let _command = cmd!("cargo", "run", "--bin", test_name).dir(prev_folder).run().unwrap();
    let expected = std::fs::read_to_string(prev_folder.join(&output_filename)).unwrap();

    let curr_folder = Path::new("../tests-current");
    let _command = cmd!("cargo", "run", "--bin", test_name).dir(curr_folder).run().unwrap();
    let current = std::fs::read_to_string(curr_folder.join(&output_filename)).unwrap();

    assert_eq!(expected, current);
}

fn main() {
    execute_test("keyboard");
}
