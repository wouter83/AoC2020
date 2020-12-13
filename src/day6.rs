
use std::collections::HashMap;

fn parse_questionaire(lines: &Vec<String>) -> HashMap<char, u8> {
  let mut map = HashMap::new();
  let lines_count = lines.len();
  for line in lines{
    for c in line.chars(){
      let val = map.entry(c).or_insert(0);
      *val += 1;
      if lines_count == 1 { // corner case
        *val += 1;
      }
    }
  }
  return map;
}


fn parse(lines: &Vec<String>) -> Vec<HashMap<char,u8>> {
  let mut lines_vec: Vec<String> = Vec::new();
  let mut questionaire_vec: Vec<HashMap<char, u8>> = Vec::new();
  for line in lines {
    if line.len() > 0 {
      lines_vec.push(line.to_string());
    } else {
      questionaire_vec.push(parse_questionaire(&lines_vec));
      lines_vec.clear();
    }
  }
  // if we have something left, add it
  questionaire_vec.push(parse_questionaire(&lines_vec));
  lines_vec.clear();
  return questionaire_vec;
}

fn count_yes(hmap_vec: &Vec<HashMap<char,u8>>) -> usize{
  let mut count = 0;
  for hmap in hmap_vec{
    count += hmap.len();
  }
  return count;
}

fn count_yes_2(hmap_vec: &Vec<HashMap<char,u8>>) -> usize{
  let mut count = 0;
  //merge them
 // let mut combined: HashMap<char,u8> = HashMap::new();
  for hmap in hmap_vec{
  //  combined.exend(hmap);
  
    for m in hmap{
      if *m.1 > 1{
        count += 1;
      }
    }
    println!("hmap: {:?} count: {}", hmap, count);

  }
  return count;
}

pub fn day6_main(){
  println!("Day 6!");
  let lines = generic::lines_from_file("input/day6.txt");
  println!("Day 6.1 awnser is: {:?}", count_yes(&parse(&lines)));
  println!("Day 6.2 awnser is: {:?}", count_yes_2(&parse(&lines)));

}

#[test]
fn day_6_bla() {

  let mut asd = vec!["ABC".to_string()];

  let mut hmap = HashMap::new();
  hmap.insert('A',2);
  hmap.insert('B',2);
  hmap.insert('C',2);
  assert_eq!(parse_questionaire(&asd), hmap);

  let mut asd1 = vec![
    "abc".to_string(),
"".to_string(),
"a".to_string(),
"b".to_string(),
"c".to_string(),
"".to_string(),
"ab".to_string(),
"ac".to_string(),
"".to_string(),
"a".to_string(),
"a".to_string(),
"a".to_string(),
"a".to_string(),
"".to_string(),
"b".to_string(),
  ];

  let hmap1 = parse(&asd1);
  assert_eq!(hmap1.len(),5);

  assert_eq!(count_yes(&hmap1), 11);
  assert_eq!(count_yes_2(&hmap1), 5);
}