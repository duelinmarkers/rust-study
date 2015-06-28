pub fn add_3_and_times_4(x : i32) -> i32 {
  times_4(add_3(x))
}

fn add_3(x : i32) -> i32 { x + 3 }

fn times_4(x : i32) -> i32 { x * 4 }

#[cfg(test)]
mod test {
  use super::add_3;
  use super::times_4;

  #[test]
  fn test_add_3() {
    assert_eq!(3, add_3(0));
  }

  #[test]
  fn test_times_4() {
    assert_eq!(8, times_4(2));
  }
}
