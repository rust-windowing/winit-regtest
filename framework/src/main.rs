use difference::{assert_diff, Changeset, Difference};
use duct::{cmd, Expression};
use std::{io::Write, path::Path};
use termcolor::{Color, ColorSpec, StandardStream, WriteColor};

fn execute_command(command: Expression) {
    let result = command
        .stderr_capture()
        .stdout_capture()
        .unchecked()
        .run()
        .unwrap();
    if !result.status.success() {
        let stdout = String::from_utf8_lossy(&result.stdout);
        let stderr = String::from_utf8_lossy(&result.stderr);
        panic!(
            "------------------\nFailed to run tests. STATUS: {}\nSTDOUT: {}\nSTDERR: {}\n------------------",
            result.status,
            stdout,
            stderr
        );
    }
}

fn print_change(changeset: &Changeset) {
    let mut stdout = StandardStream::stdout(termcolor::ColorChoice::Always);
    let mut rem_col = ColorSpec::new();
    rem_col.set_fg(Some(Color::Red)).set_intense(true);
    let mut add_col = ColorSpec::new();
    add_col.set_fg(Some(Color::Cyan)).set_intense(true);
    let mut def_col = ColorSpec::new();
    def_col.set_reset(true);

    // The text before the change on the line where the change is.
    let mut is_first = true;
    let diff_count = changeset.diffs.len();

    for (id, diff) in changeset.diffs.iter().enumerate() {
        let is_last = id == diff_count - 1;
        match diff {
            Difference::Same(contents) => {
                let mut printed = false;
                if !is_first {
                    if let Some(pos) = contents.find("\n") {
                        let last_line_contents = &contents.as_bytes()[0..pos + 1];
                        print!("{}", std::str::from_utf8(last_line_contents).unwrap());
                        printed = true;
                    }
                }
                if !is_last {
                    if let Some(pos) = contents.rfind("\n") {
                        let last_line_contents = &contents.as_bytes()[pos..];
                        print!("{}", std::str::from_utf8(last_line_contents).unwrap());
                        printed = true;
                    }
                }
                if (!is_first || !is_last) && !printed {
                    // This can happen if the segment didn't contain any line breaks
                    print!("{}", contents);
                }
            }
            Difference::Add(contents) => {
                stdout.set_color(&add_col).unwrap();
                write!(stdout, "{}", contents).unwrap();
                stdout.set_color(&def_col).unwrap();
                write!(stdout, "{}", changeset.split).unwrap();
            }
            Difference::Rem(contents) => {
                stdout.set_color(&rem_col).unwrap();
                write!(stdout, "{}", contents).unwrap();
                stdout.set_color(&def_col).unwrap();
                write!(stdout, "{}", changeset.split).unwrap();
            }
        }
        is_first = false;
    }
}

fn execute_test(test_name: &str) {
    println!("-- Executing test: '{}'", test_name);

    let output_filename = format!("{}.txt", test_name);

    let prev_folder = Path::new("../tests-previous");
    let command_prev = cmd!("cargo", "run", "--bin", test_name).dir(prev_folder);
    execute_command(command_prev);

    let previous = std::fs::read_to_string(prev_folder.join(&output_filename)).unwrap();

    let curr_folder = Path::new("../tests-current");
    let command_curr = cmd!("cargo", "run", "--bin", test_name).dir(curr_folder);
    execute_command(command_curr);
    let current = std::fs::read_to_string(curr_folder.join(&output_filename)).unwrap();

    let line_changes = Changeset::new(&previous, &current, "");
    if line_changes.distance > 0 {
        print_change(&line_changes);
        println!(
            "ERROR: Previous and current are not identical. Distance is {}",
            line_changes.distance
        );
        return;
    }
    println!("Successfully completed '{}'", test_name);
}

fn main() {
    execute_test("keyboard");
    execute_test("mouse");
}
