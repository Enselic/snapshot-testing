use std::env::args;

fn main() {
    let value = args().nth(1).expect("arg must be passed");
    let snapshot_path = args().nth(2).expect("snapshot path must be passed");
    snapshot_testing::assert_eq_or_update(value, snapshot_path);
}
