#![allow(dead_code)]
#![allow(unused_variables)]

use assert_cmd::Command;
use serial_test::serial;
use std::{
    env::set_current_dir,
    fs::{self, create_dir_all, File},
    path::{Path, PathBuf},
};

struct TestParams<'a> {
    filenames: Vec<&'a str>,
    directories: Vec<&'a str>,
    pattern: &'a str,
    target: &'a str,
    new_names: Vec<&'a str>,
}
impl TestParams<'_> {
    fn new<'a>(
        filenames: Vec<&'a str>,
        directories: Vec<&'a str>,
        pattern: &'a str,
        target: &'a str,
        new_names: Vec<&'a str>,
    ) -> TestParams<'a> {
        TestParams {
            filenames,
            directories,
            pattern,
            target,
            new_names,
        }
    }
}

fn prepare_location(filenames: &Vec<&str>, directories: &Vec<&str>) -> Option<Command> {
    let path = "target/debug";
    set_current_dir(path);
    let cmd = Command::new(PathBuf::from("mmv"));
    fs::remove_dir_all("tmp");
    dbg!(Path::new("./tmp/").exists());
    dbg!(fs::create_dir("./tmp/").is_ok());
    if Path::new("./tmp/").exists() || fs::create_dir("./tmp/").is_ok() {
        for file in filenames {
            let file_full_path = Path::new("./tmp/").join(file);
            let file_dir = file_full_path.parent();
            if file_dir.is_some() {
                create_dir_all(file_dir.unwrap());
            }
            let created_file = File::create(Path::new("./tmp/").join(file));
            if created_file.is_err() {
                dbg!(created_file.err());
                return None;
            }
            created_file.unwrap().sync_all();
        }
        for directory in directories {
            create_dir_all(Path::new("./tmp/").join(directory));
        }
        set_current_dir("tmp/");
        Some(cmd)
    } else {
        dbg!(path);
        None
    }
}

fn check_moves(changed_names: &Vec<&str>) {
    for file in changed_names {
        assert!(Path::new(file).try_exists().unwrap());
    }
}

fn leave_location() {
    set_current_dir("..");
    fs::remove_dir_all("tmp");
}

fn run_with_params(params: TestParams) {
    let TestParams {
        filenames,
        directories,
        pattern,
        target,
        new_names,
    } = params;
    let command = prepare_location(&filenames, &directories);
    if command.is_none() {
        panic!("Initialization failed, please retry testing")
    }
    command.unwrap().arg(pattern).arg(target).unwrap();
    check_moves(&new_names);
    leave_location()
}
#[test]
#[serial]
fn test_single_file() {
    let params = TestParams::new(vec!["abc"], vec![], "a*c", "a#1d", vec!["abd"]);
    run_with_params(params);
}

#[test]
#[serial] // Ah shit, I spend about 4 hours finding out, why running a single test always works, but 2 or more at the same time failed in like half of executions. If only I knew cargo runs tests in parallel, so creating-deleting files operations may go crazy 🫠🫠
fn test_notion_example() {
    let params = TestParams::new(
        vec![
            "some_A_filename.jpg",
            "some_A_filename.bin",
            "some_B_filename.jpg",
            "some_B_filename.bin",
        ],
        vec![],
        "some_*_filename.*",
        "changed_#1_filename.#2",
        vec![
            "changed_A_filename.jpg",
            "changed_B_filename.jpg",
            "changed_A_filename.bin",
            "changed_B_filename.bin",
        ],
    );
    run_with_params(params)
}

#[test]
#[serial]
fn test_utf() {
    let params = TestParams::new(
        vec!["somewhere/not/here/Аленка"],
        vec!["sowhere"],
        "somewhere/not/here/А*ка",
        "sowhere/Е#1а",
        vec!["sowhere/Елена"],
    );
    run_with_params(params)
}
#[test]
#[serial]
fn test_dummy_template() {
    let params = TestParams::new(
        vec!["somewhere/not/a", "somewhere/not/b"],
        vec![],
        "somewhere/not/*",
        "#1",
        vec!["a", "b"],
    );
    run_with_params(params)
}

#[test]
#[serial]
fn test_cut_suffix() {
    let params = TestParams::new(
        vec!["aXXXcYYY", "aXc", "acY"],
        vec![],
        "a*c*",
        "a#1c",
        vec!["aXXXc", "aXc", "ac"],
    );
    run_with_params(params)
}
