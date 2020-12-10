#[derive(Default)]
struct Passport
{
  byr: String,
  iyr: String,
  eyr: String,
  hgt: String,
  hcl: String,
  ecl: String,
  pid: String,
  cid: String,
}

impl Passport{
  fn new (input: &Vec<String>) -> Self{
    let mut ret:Self = Default::default();
    for line in input{
      let values: Vec<&str> = line.split(' ').collect();
      for val in values{
        let key_val : Vec<&str> = val.split(':').collect();
        match key_val[0]{
          "byr" => ret.byr = key_val[1].to_string(),
          "iyr" => ret.iyr = key_val[1].to_string(),
          "eyr" => ret.eyr = key_val[1].to_string(),
          "hgt" => ret.hgt = key_val[1].to_string(),
          "hcl" => {
            //let temp = key_val[1].find('#');
            //if temp == Some(0) {
              ret.hcl = key_val[1].to_string();
            //}
          },
          "ecl" => ret.ecl = key_val[1].to_string(),
          "pid" => ret.pid = key_val[1].to_string(),
          "cid" => ret.cid = key_val[1].to_string(),
          _ => println!("Ain't special"),
        }
      }
    }
    ret
  }
  fn print(&self){
    println!("====");
    println!("byr: {}",self.byr);
    println!("iyr: {}",self.iyr);
    println!("eyr: {}",self.eyr);
    println!("hgt: {}",self.hgt);
    println!("hcl: {}",self.hcl);
    println!("ecl: {}",self.ecl);
    println!("pid: {}",self.pid);
    println!("cid: {}",self.cid);
    println!("====");
  }

  fn is_valid(&self) -> bool {
    self.byr.len() > 0 &&
    self.iyr.len() > 0 &&
    self.eyr.len() > 0 &&
    self.hgt.len() > 0 &&
    self.hcl.len() > 0 &&
    self.ecl.len() > 0 &&
    self.pid.len() > 0
  } 
}

fn parse_passports(lines: &Vec<String>) -> Vec<Passport> {
  let mut lines_vec: Vec<String> = Vec::new();
  let mut passports: Vec<Passport> = Vec::new();

  for line in lines {
    if line.len() > 0 {
      lines_vec.push(line.to_string());
    }
    else
    {
      passports.push(Passport::new(&lines_vec));
      lines_vec.clear();
    }
  }
  return passports;
}

pub fn day4_main() {
  println!("Day 4!");
  let lines = generic::lines_from_file("input/day4.txt");

  let passports = parse_passports(&lines);
  let mut count_1 = 0;
  for passp in passports{
    if passp.is_valid() {
      count_1 += 1;
      passp.print();
    }
  }
  println!("Day 4.1 awnser is: {}", count_1);

}

#[test]
fn test_day4_passport_1() {
  let input = vec![
    "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd".to_string(),
    "byr:1937 iyr:2017 cid:147 hgt:183cm".to_string()
  ];
  let passp = Passport::new(&input);
  //Passport::print(&passp);
  passp.print();
  assert_eq!(passp.is_valid(),true);
}

#[test]
fn test_day4_passport_2() {
  let input = vec![
    "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884".to_string(),
    "hcl:#cfa07d byr:1929".to_string()
  ];
  let passp = Passport::new(&input);
  //Passport::print(&passp);
  passp.print();
  assert_eq!(passp.is_valid(),false);
}

#[test]
fn test_day4_passport_3() {
  let input = vec![
    "hcl:#ae17e1 iyr:2013".to_string(),
    "eyr:2024".to_string(),
    "ecl:brn pid:760753108 byr:1931".to_string(),
    "hgt:179cm".to_string()
  ];
  let passp = Passport::new(&input);
  //Passport::print(&passp);
  passp.print();
  assert_eq!(passp.is_valid(),true);
}

#[test]
fn test_day4_passport_4() {
  let input = vec![
    "hcl:#cfa07d eyr:2025 pid:166559648".to_string(),
    "iyr:2011 ecl:brn hgt:59in".to_string()
  ];
  let passp = Passport::new(&input);
  //Passport::print(&passp);
  passp.print();
  assert_eq!(passp.is_valid(),false);
}

#[test]
fn test_day4_passport_all() {
  let input = vec![
    "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd".to_string(),
    "byr:1937 iyr:2017 cid:147 hgt:183cm".to_string(),
    "".to_string(),
    "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884".to_string(),
    "hcl:#cfa07d byr:1929".to_string(),
    "".to_string(),
    "hcl:#ae17e1 iyr:2013".to_string(),
    "eyr:2024".to_string(),
    "ecl:brn pid:760753108 byr:1931".to_string(),
    "hgt:179cm".to_string(),
    "".to_string(),
    "hcl:#cfa07d eyr:2025 pid:166559648".to_string(),
    "iyr:2011 ecl:brn hgt:59in".to_string(),
    "".to_string(),
  ];
  let passp = parse_passports(&input);
  assert_eq!(passp.len(),4);

  assert_eq!(passp[0].is_valid(),true);
  assert_eq!(passp[1].is_valid(),false);
  assert_eq!(passp[2].is_valid(),true);
  assert_eq!(passp[3].is_valid(),false);

}