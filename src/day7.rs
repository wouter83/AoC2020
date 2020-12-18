use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, Default, Hash, Eq, PartialEq, Clone)]
struct BagContents {
    number: u32,
    name: String,
}

fn parse_for_bags(bag: &str) -> (u32, String) {
    let henk: Vec<&str> = bag.split(' ').collect();
    let mut rets: String = Default::default();
    let mut retn: u32 = 0;
    for h in henk {
        match &h[..] {
            "bags." => {}
            "bags" => {}
            "bag." => {}
            "bag" => {}
            _ => {
                //println!("Conmtent: {}", h);
                let rslt = h.parse::<u32>();
                match rslt {
                    Ok(x) => retn = x,
                    Err(e) => rets.push_str(h),
                }
            }
        }
    }
    return (retn, rets);
}

fn parse_1(input: &Vec<String>) -> HashMap<String, Vec<BagContents>> {
    let mut mapping: HashMap<String, Vec<BagContents>> = HashMap::new();
    for line in input {
        //println!("=============================== ");
        let sepper1: Vec<&str> = line.split(" bags contain ").collect(); // Bag name | bags inside
                                                                         //println!("sepper1: {:?}", sepper1);
        let bag_name: String = sepper1[0]
            .to_string()
            .replace(&[' '][..], "")
            .to_lowercase();
        //println!("bag_name: {}", bag_name);
        let bag_conttents: Vec<&str> = sepper1[1].split(", ").collect();
        //println!("bag_conttents: {:?}", bag_conttents);
        for bag in bag_conttents {
            //println!("Bag {:?}", bag);
            match &bag[..] {
                "noother" => {}
                _ => {
                    let bag_content = parse_for_bags(&bag);
                    let val = mapping.entry(bag_content.1).or_insert(Default::default());
                    val.push(BagContents {
                        number: bag_content.0,
                        name: bag_name.clone(),
                    });
                }
            }
        }
    }
    return mapping;
}

fn parse_2(input: &Vec<String>) -> HashMap<String, Vec<BagContents>> {
    let mut mapping: HashMap<String, Vec<BagContents>> = HashMap::new();
    for line in input {
        //println!("=============================== ");
        let sepper1: Vec<&str> = line.split(" bags contain ").collect(); // Bag name | bags inside
                                                                         //println!("sepper1: {:?}", sepper1);
        let bag_name: String = sepper1[0]
            .to_string()
            .replace(&[' '][..], "")
            .to_lowercase();
        //println!("bag_name: {}", bag_name);
        let bag_conttents: Vec<&str> = sepper1[1].split(", ").collect();
        //println!("bag_conttents: {:?}", bag_conttents);
        for bag in bag_conttents {
            //println!("Bag {:?}", bag);
            match &bag[..] {
                "noother" => {}
                _ => {
                    let bag_content = parse_for_bags(&bag);
                    let val = mapping
                        .entry(bag_name.clone())
                        .or_insert(Default::default());
                    val.push(BagContents {
                        number: bag_content.0,
                        name: bag_content.1,
                    });
                }
            }
        }
    }
    return mapping;
}

fn calc_1_rec(
    mapping: &HashMap<String, Vec<BagContents>>,
    name: &String,
    foundbags: &mut HashSet<String>,
) -> usize {
    let mut count: usize = 0;
    match mapping.get(name) {
        Some(bags) => {
            count += bags.len();
            for bag in bags {
                if !foundbags.contains(&bag.name) {
                    foundbags.insert(bag.name.clone());
                    count += calc_1_rec(&mapping, &bag.name, foundbags);
                }
            }
        }
        None => {}
    }
    return count;
}

fn calc_2_rec(
    mapping: &HashMap<String, Vec<BagContents>>,
    name: &String,
    foundbags: &mut HashSet<String>,
) -> usize {
    let mut count: usize = 0;
    match mapping.get(name) {
        Some(bags) => {
            count += bags.len();
            for bag in bags {
                if !foundbags.contains(&bag.name) {
                    foundbags.insert(bag.name.clone());
                    count += calc_1_rec(&mapping, &bag.name, foundbags);
                }
            }
        }
        None => count = bag.number,
    }
    return count;
}

fn calc_1(mapping: &HashMap<String, Vec<BagContents>>, name: &String) -> usize {
    let mut foundbags: HashSet<String> = HashSet::new();

    calc_1_rec(&mapping, &name, &mut foundbags);
    return foundbags.len();
}

fn calc_2(mapping: &HashMap<String, Vec<BagContents>>, name: &String) -> usize {
    let mut foundbags: HashSet<String> = HashSet::new();
    println!("calc_2: Mapping: {:?}", mapping);
    let count = calc_2_rec(&mapping, &name, &mut foundbags);
    return count;
}

pub fn day7_main() {
    println!("Day 7!");
    let lines = generic::lines_from_file("input/day7.txt");

    println!(
        "Day 7.1 awnser is: {:?}",
        calc_1(&parse_1(&lines), &"shinygold".to_string())
    );
    //   println!("Day 7.2 awnser is: {:?}", );
}

#[test]
fn day_7_check() {
    let asd1 = vec![
        "light red bags contain 1 bright white bag, 2 muted yellow bags.".to_string(),
        "dark orange bags contain 3 bright white bags, 4 muted yellow bags.".to_string(),
        "bright white bags contain 1 shiny gold bag.".to_string(),
        "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.".to_string(),
        "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.".to_string(),
        "dark olive bags contain 3 faded blue bags, 4 dotted black bags.".to_string(),
        "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.".to_string(),
        "faded blue bags contain no other bags.".to_string(),
        "dotted black bags contain no other bags.".to_string(),
    ];

    let parsed_1 = parse_1(&asd1);
    assert_eq!(calc_1(&parsed_1, &"shinygold".to_string()), 4);

    let parsed_2 = parse_2(&asd1);
    assert_eq!(calc_2(&parsed_2, &"shinygold".to_string()), 4);
}
