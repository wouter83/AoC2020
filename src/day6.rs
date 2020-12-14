use std::collections::HashSet;
use std::iter::FromIterator;

fn parse_questionaire_person(line: &str) -> HashSet<char> {
  let mut map = HashSet::new();
  for c in line.chars() {
    let val = map.insert(c);
  }
  return map;
}

fn parse_questionaire_group(lines: &Vec<String>) -> Vec<HashSet<char>> {
  let mut questionaire_group: Vec<HashSet<char>> = Vec::new();

  for line in lines {
    questionaire_group.push(parse_questionaire_person(&line));
  }
  return questionaire_group;
}

fn parse(lines: &Vec<String>) -> Vec<Vec<HashSet<char>>> {
  let mut lines_vec: Vec<String> = Vec::new();
  let mut questionaire_vec: Vec<Vec<HashSet<char>>> = Vec::new();
  for line in lines {
    if line.len() > 0 {
      lines_vec.push(line.to_string());
    } else {
      questionaire_vec.push(parse_questionaire_group(&lines_vec));
      lines_vec.clear();
    }
  }
  // if we have something left, add it
  questionaire_vec.push(parse_questionaire_group(&lines_vec));
  lines_vec.clear();
  return questionaire_vec;
}

fn count_yes(hmap_vec: &Vec<Vec<HashSet<char>>>) -> usize {
  let mut count = 0;
  for hmap in hmap_vec {
    count += hmap.len();
  }
  return count;
}

fn intersects(sets: &[HashSet<char>]) -> HashSet<char> {
   let ret = sets.iter().fold(sets[0].clone(), |s1: HashSet<char>, s2| s1.intersection(&s2).copied().collect());
  return ret;   
}

fn count_yes_2(hmap_vec: &Vec<Vec<HashSet<char>>>) -> usize {
  hmap_vec.iter().map(|group| intersects(group)).map(|s| s.len()).sum::<usize>()
}

pub fn day6_main() {
  println!("Day 6!");
  let lines = generic::lines_from_file("input/day6.txt");

  println!("Day 6.1 awnser is: {:?}", count_yes(&parse(&lines)));
  println!("Day 6.2 awnser is: {:?}", count_yes_2(&parse(&lines)));
}

#[test]
fn day_6_bla() {
  // let asd = vec!["ABC".to_string()];

  // let mut hmap = HashMap::new();
  // hmap.insert('A', 2);
  // hmap.insert('B', 2);
  // hmap.insert('C', 2);
  // assert_eq!(parse_questionaire(&asd), hmap);

  let asd1 = vec![
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
  assert_eq!(hmap1.len(), 5);

  assert_eq!(count_yes(&hmap1), 11);
  assert_eq!(count_yes_2(&hmap1), 6);
}
