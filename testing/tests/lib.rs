extern crate testing;

use testing::add_3_and_times_4;

#[test]
fn math_checks_out() {
  assert_eq!(32i, add_3_and_times_4(5));
}
