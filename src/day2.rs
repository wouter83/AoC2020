fn decode_password_wrong(password: &std::string::String) -> bool {
 let values: Vec<&str> = password.split(' ').collect();
 // values[0] has the lower and upper counts
 // values[1] has the char + ':'
 // values[2] the password.
 
 let lower_upper: Vec<&str> = values[0].split('-').collect();
 let low = lower_upper[0].parse::<usize>().unwrap();
 let high = lower_upper[1].parse::<usize>().unwrap();
 let what_char: Vec<&str> = values[1].split(':').collect(); //char in [0]
 let password_val = values[2];
 
 let count = password_val.matches(what_char[0]).count();
 return low <= count && count <= high;
}
 
fn decode_password(password: &std::string::String) -> bool {
 let values: Vec<&str> = password.split(' ').collect();
 // values[0] has the lower and upper counts
 // values[1] has the char + ':'
 // values[2] the password.
 
 let lower_upper: Vec<&str> = values[0].split('-').collect();
 let low = lower_upper[0].parse::<usize>().unwrap();
 let high = lower_upper[1].parse::<usize>().unwrap();
 let what_char: Vec<&str> = values[1].split(':').collect(); //char in [0]
 let password_val = values[2];
 
 let mut ret = (false,false);
 for chars in password_val.char_indices()
 {
   if (chars.1.to_string() == what_char[0]) && (chars.0 == low -1) {
     ret.0 = true;
   }
   if (chars.1.to_string() == what_char[0]) && (chars.0 == high -1) {
     ret.1 = true;
   }
 }
 
 return ret.0 ^ ret.1;
}
 
pub fn day2_main() {
 println!("day 2");
 let mut valid_1 = 0;
 let mut valid_2 = 0;
 
 let lines = generic::lines_from_file("input/day2.txt");
 for line in lines {
   if decode_password_wrong(&line){
       valid_1 = valid_1 + 1;
   }
   if decode_password(&line){
       valid_2 = valid_2 + 1;
   }
 }
 println!("Day 2.1 awnser is: {}", valid_1);
 println!("Day 2.1 awnser is: {}", valid_2);
}
 
#[test]
fn test_password_1() {
 let password = String::from("1-3 a: abcde");
 assert_eq!(decode_password(&password), true);
}
#[test]
fn test_password_2() {
 let password = String::from("1-3 b: cdefg");
 assert_eq!(decode_password(&password), false);
}
#[test]
fn test_password_3() {
 let password = String::from("2-9 c: ccccccccc");
 assert_eq!(decode_password(&password), false);
}