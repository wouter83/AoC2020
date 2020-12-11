trait Day4Trait {
  fn is_valid(&self) -> bool;
}

#[derive(Default)]
struct Eyecollor {
  color: String,
}
#[derive(Default)]
struct Height {
  number: u16,
  metric: String,
}

#[derive(Default)]
struct Passport {
  byr: u16,
  iyr: u16,
  eyr: u16,
  hgt: Height,
  hcl: String,
  ecl: Eyecollor,
  pid: u32,
  cid: String,
}

impl Day4Trait for Height {
  fn is_valid(&self) -> bool {
    if self.metric == "cm" {
      return self.number >= 150 && self.number <= 193;
    }
    if self.metric == "in" {
      return self.number >= 59 && self.number <= 76;
    }
    false
  }
}

impl Day4Trait for Eyecollor {
  fn is_valid(&self) -> bool {
    match self.color.as_str() {
      "amb" => true,
      "blu" => true,
      "brn" => true,
      "gry" => true,
      "grn" => true,
      "hzl" => true,
      "oth" => true,
      _ => false,
    }
  }
}

impl Passport {
  fn new(input: &Vec<String>) -> Self {
    let mut ret: Self = Default::default();
    for line in input {
      let values: Vec<&str> = line.split(' ').collect();
      for val in values {
        let key_val: Vec<&str> = val.split(':').collect();
        match key_val[0] {
          "byr" => ret.byr = key_val[1].to_string().parse::<u16>().unwrap(),
          "iyr" => ret.iyr = key_val[1].to_string().parse::<u16>().unwrap(),
          "eyr" => ret.eyr = key_val[1].to_string().parse::<u16>().unwrap(),
          "hgt" => {
            let mut number_string: String = String::new();
            let mut metric_string: String = String::new();
            for c in key_val[1].to_string().chars() {
              if c.is_digit(10) {
                number_string.push(c);
              } else {
                metric_string.push(c);
              }
            }
            ret.hgt.metric = metric_string;
            ret.hgt.number = number_string.parse::<u16>().unwrap();
          }
          "hcl" => {
            if key_val[1].len() != 7 {
              break;
            };
            let temp = key_val[1].find('#');
            if temp == Some(0) {
              //              println!("hcl {:?}", key_val[1].as_bytes());
              let byte_array = key_val[1].as_bytes();
              let mut add = true;
              for i in 1..=6 {
                add &= byte_array[i].is_ascii_hexdigit();
              }
              ret.hcl = if add {
                key_val[1].to_string()
              } else {
                "".to_string()
              };
            }
            //            println!("ret.hcl: {}", ret.hcl);
          }
          "ecl" => ret.ecl.color = key_val[1].to_string(),
          "pid" => {
            if key_val[1].to_string().len() == 9 {
              let yess = key_val[1].to_string().parse::<u32>();
              if !yess.is_err() {
                ret.pid = yess.unwrap();
              }
            }
          }
          "cid" => ret.cid = key_val[1].to_string(),
          _ => println!("Ain't special"),
        }
      }
    }
    ret
  }
  fn print(&self) {
    println!("====");
    println!("byr: {}", self.byr);
    println!("iyr: {}", self.iyr);
    println!("eyr: {}", self.eyr);
    println!("hgt: {}{}", self.hgt.number, self.hgt.metric);
    println!("hcl: {}", self.hcl);
    println!("ecl: {}", self.ecl.color);
    println!("pid: {}", self.pid);
    println!("cid: {}", self.cid);
    println!("====");
  }

  fn is_valid(&self) -> bool {
    (self.byr >= 1920 && self.byr <= 2002)
      && (self.iyr >= 2010 && self.iyr <= 2020)
      && (self.eyr >= 2020 && self.eyr <= 2030)
      && (self.hgt.is_valid())
      && (self.hcl.len() == 7)
      && (self.ecl.is_valid())
      && (self.pid > 0 && self.pid < 1000000000)
  }
}

fn parse_passports(lines: &Vec<String>) -> Vec<Passport> {
  let mut lines_vec: Vec<String> = Vec::new();
  let mut passports: Vec<Passport> = Vec::new();

  for line in lines {
    if line.len() > 0 {
      lines_vec.push(line.to_string());
    } else {
      passports.push(Passport::new(&lines_vec));
      lines_vec.clear();
    }
  }
  // if we have something left, add it
  passports.push(Passport::new(&lines_vec));
  lines_vec.clear();
  return passports;
}

pub fn day4_main() {
  println!("Day 4!");
  let lines = generic::lines_from_file("input/day4.txt");

  let passports = parse_passports(&lines);
  let mut count_1 = 0;
  for passp in passports {
    if passp.is_valid() {
      count_1 += 1;
    }
  }
  println!("Day 4.2 awnser is: {}", count_1);
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
    //"".to_string(),  removed the line because it reflects the read_line from the generig lib
  ];
  let passp = parse_passports(&input);
  assert_eq!(passp.len(), 4);

  assert_eq!(passp[0].is_valid(), true);
  assert_eq!(passp[1].is_valid(), false);
  assert_eq!(passp[2].is_valid(), true);
  assert_eq!(passp[3].is_valid(), false);
}

#[test]
fn test_day4_passport_strict_invalid() {
  let input = vec![
    "eyr:1972 cid:100".to_string(),
    "hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926".to_string(),
    "".to_string(),
    "iyr:2019".to_string(),
    "hcl:#602927 eyr:1967 hgt:170cm".to_string(),
    "ecl:grn pid:012533040 byr:1946".to_string(),
    "".to_string(),
    "hcl:dab227 iyr:2012".to_string(),
    "ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277".to_string(),
    "".to_string(),
    "hgt:59cm ecl:zzz".to_string(),
    "eyr:2038 hcl:74454a iyr:2023".to_string(),
    "pid:3556412378 byr:2007".to_string(),
    "".to_string(),
    "iyr:2010 hgt:76in hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719".to_string(),
  ];

  let passp = parse_passports(&input);
  assert_eq!(passp.len(), 5);

  assert_eq!(passp[0].is_valid(), false);
  assert_eq!(passp[1].is_valid(), false);
  assert_eq!(passp[2].is_valid(), false);
  assert_eq!(passp[3].is_valid(), false);
  assert_eq!(passp[4].is_valid(), false);
}

#[test]
fn test_day4_passport_strict_valid() {
  let input = vec![
    "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980".to_string(),
    "hcl:#623a2f".to_string(),
    "".to_string(),
    "eyr:2029 ecl:blu cid:129 byr:1989".to_string(),
    "iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm".to_string(),
    "".to_string(),
    "hcl:#888785".to_string(),
    "hgt:164cm byr:2001 iyr:2015 cid:88".to_string(),
    "pid:545766238 ecl:hzl".to_string(),
    "eyr:2022".to_string(),
    "".to_string(),
    "iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719".to_string(),
  ];

  let passp = parse_passports(&input);
  assert_eq!(passp.len(), 4);

  assert_eq!(passp[0].is_valid(), true);
  assert_eq!(passp[1].is_valid(), true);
  assert_eq!(passp[2].is_valid(), true);
  assert_eq!(passp[3].is_valid(), true);
}
