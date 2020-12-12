fn parse_to_row_column(input: &str) -> (u32,u32) {
  // get 2 strings one 7 char and one 3 char
  let row_string = &input[0..=6];
  let column_string = &input[7..=9];
  let mut index = 0;
  let mut row = 0;
  let mut column = 0;
  for c in row_string.chars().rev().collect::<String>().chars()
  {
    match c {
      'B' => {
        row |= 1 << index;
      }
      _ => {}
    }
    index += 1;
  }
  index = 0;
  for c in column_string.chars().rev().collect::<String>().chars()
  {
    match c {
      'R' => {
        column |= 1 << index;
      }
      _ => {}
    }
    index += 1;
  }

  (row,column)
}

fn seat_id (rowcol: (u32,u32)) -> u32{
  rowcol.0 * 8 + rowcol.1
}

fn get_missing(vect: &Vec<u32>) -> u32{
  for i in 0..(vect.len() -1) {
    if vect[i+1] - vect[i] != 1 {
      return vect[i]  + 1;
    }
  }
  return 0;
}

pub fn day5_main () {
  println!("Day 5!");
  let lines = generic::lines_from_file("input/day5.txt");
  let mut seatids: Vec<u32> = Vec::new();
  for line in lines {
    seatids.push(seat_id(parse_to_row_column(&line)));
  }
  seatids.sort();
  println!("Day 5.1 awnser is: {:?}", seatids.last());
  println!("Day 5.2 awnser is: {:?}", get_missing(&seatids));

}

#[test]
fn day_5_parsing_test() {
  assert_eq!(seat_id(parse_to_row_column("FBFBBFFRLR")),357);
}
#[test]
fn day_5_get_missing() {
  let vect = vec![1,2,3,5];
  assert_eq!(get_missing(&vect), 4);
}