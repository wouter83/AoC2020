fn has_tree(line: &std::string::String, pos:usize) -> bool
{
  let tree = line.chars().nth(pos).unwrap();
  return tree == '#';
}

fn count_trees(slope: &Vec<String>, right: usize, down: usize) -> usize
{
  let mut start_right = 0 + right;
  let mut start_down = 0 + down;
  let mut trees = 0;

  let boundary_down = slope.len();
  let boundary_right = slope[0].len();
  //println!("boundary_right: {}, boundary_down: {}", boundary_right, boundary_down);

  while start_down < boundary_down
  {
    //println!("start_right: {}, start_down: {}", start_right, start_down);
    start_right = start_right % boundary_right;
    if has_tree(&slope[start_down], start_right){
      trees += 1;
    }
    start_right += right;
    start_down += down;
  }
  return trees;
}

pub fn day3_main() {
  println!("Day 3!");
  let lines = generic::lines_from_file("input/day3.txt");
  let trees_1 = count_trees(&lines, 3, 1);
  println!("Day 3.1 awnser is: {}", trees_1);
  
  let mut trees_2 = 0;
  trees_2 += count_trees(&lines,1,1);
  trees_2 *= count_trees(&lines,3,1);
  trees_2 *= count_trees(&lines,5,1);
  trees_2 *= count_trees(&lines,7,1);
  trees_2 *= count_trees(&lines,1,2);

  println!("Day 3.2 awnser is: {}", trees_2);

}

#[test]
fn day3_test_line_ok()
{
  let line:std::string::String = "..##.......".to_string();
  assert_eq!(has_tree(&line,0), false);
  let line:std::string::String = "..##.......".to_string();
  assert_eq!(has_tree(&line,1), false);
  let line:std::string::String = "..##.......".to_string();
  assert_eq!(has_tree(&line,2), true);
  let line:std::string::String = "..##.......".to_string();
  assert_eq!(has_tree(&line,3), true);

}

#[test]
fn day3_test_slope()
{
  let input = vec![
    "..##.......".to_string(),
    "#...#...#..".to_string(),
    ".#....#..#.".to_string(),
    "..#.#...#.#".to_string(),
    ".#...##..#.".to_string(),
    "..#.##.....".to_string(),
    ".#.#.#....#".to_string(),
    ".#........#".to_string(),
    "#.##...#...".to_string(),
    "#...##....#".to_string(),
    ".#..#...#.#".to_string()];

    assert_eq!(count_trees(&input,1,1), 2);
    assert_eq!(count_trees(&input,3,1), 7);
    assert_eq!(count_trees(&input,5,1), 3);
    assert_eq!(count_trees(&input,7,1), 4);
    assert_eq!(count_trees(&input,1,2), 2);
}