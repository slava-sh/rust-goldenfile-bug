extern crate goldenfile;

use std::fs::File;
use std::io::Write;

use goldenfile::Mint;

fn create_file() {
    let mut file = File::create("test").expect("create filed");
    file.write(b"data").expect("write failed");
}

fn main() {
    create_file();
    let mut mint = Mint::new(".");
    let _file = mint.new_goldenfile("test").expect("new_goldenfile failed");
    assert!(false);
}

#[test]
fn bug() {
    create_file();
    let mut mint = Mint::new(".");
    let _file = mint.new_goldenfile("test").expect("new_goldenfile failed");
    assert!(false);
}
