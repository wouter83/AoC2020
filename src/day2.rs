fn decode_password(password: &std::string::String) -> bool {
  println!("{}", password);
  let dash_pos = password.find('-');

  let values: Vec<&str> = password.split(' ').collect();
  // values[0] has the lower and upper counts
  // values[1] has the char + ':'
  // values[2] the password.

  let lower_upper: Vec<&i32> = values[0].split('-').collect();


  return true;
}

pub fn day2_main()
{
  println!("day 2");
  let password = String::from("1-3 a: abcde");
  assert_eq!(decode_password(&password), true);

}

#[test]
fn test_password() {
  let password = String::from("1-3 a: abcde");
  assert_eq!(decode_password(&password), true);
}