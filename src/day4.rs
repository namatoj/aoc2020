use std::collections::HashSet;

pub fn run(input: Vec<String>) {
    println!("Day 4");
    println!("=====");

    println!("part one: {}", part_one(&input));
    println!("part two: {}", part_two(&input));
}

pub fn part_one(input: &[String]) -> i32 {
    let mut valid_passports = 0;
    let mut keys_in_passport: HashSet<&str> = HashSet::new();

    for row in input {
        if row == "" {
            // check if all mandatory fields are there.

            if check_mandatory_fields(&keys_in_passport) {
                valid_passports += 1;
            }
            keys_in_passport.clear();
        } else {
            let fields = row.split_whitespace();
            for field in fields {
                let key_val: Vec<&str> = field.split(":").collect();
                keys_in_passport.insert(key_val[0]);
            }
        }
    }

    // This isn't nice, but it needs to be checked one last time.
    if check_mandatory_fields(&keys_in_passport) {
        valid_passports += 1;
    }

    valid_passports
}

fn check_mandatory_fields(fields_in_passport: &HashSet<&str>) -> bool {
    let mandatory_fields: HashSet<_> = [
        "eyr", "hgt", "ecl", "hcl", "iyr", "pid", // "cid",
        "byr",
    ]
    .iter()
    .cloned()
    .collect();

    mandatory_fields.difference(&fields_in_passport).count() == 0
}

#[derive(Debug)]
struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl Passport {
    pub fn new(input: &str) -> Self {
        let key_vals: Vec<_> = input.split_whitespace().collect();
        let mut byr = None;
        let mut iyr = None;
        let mut eyr = None;
        let mut hgt = None;
        let mut hcl = None;
        let mut ecl = None;
        let mut pid = None;
        let mut cid = None;

        for key_val in key_vals {
            let key_val: Vec<_> = key_val.split(":").collect();
            let key = key_val[0];
            let val = key_val[1];
            match key {
                "byr" => byr = Some(val.to_string()),
                "iyr" => iyr = Some(val.to_string()),
                "eyr" => eyr = Some(val.to_string()),
                "hgt" => hgt = Some(val.to_string()),
                "hcl" => hcl = Some(val.to_string()),
                "ecl" => ecl = Some(val.to_string()),
                "pid" => pid = Some(val.to_string()),
                "cid" => cid = Some(val.to_string()),
                _ => panic!("unknown key in input"),
            }
        }

        Passport {
            byr,
            iyr,
            eyr,
            hgt,
            hcl,
            ecl,
            pid,
            cid,
        }
    }

    fn is_valid_birth_year(&self) -> bool {
        if let Some(byr) = &self.byr {
            let byr: u16 = byr.parse().unwrap();
            return (byr >= 1920) && (byr <= 2002);
        }
        false
    }

    fn is_valid_issue_year(&self) -> bool {
        if let Some(iyr) = &self.iyr {
            let iyr: u16 = iyr.parse().unwrap();
            return (iyr >= 2010) && (iyr <= 2020);
        }
        false
    }

    fn is_valid_expiration_year(&self) -> bool {
        if let Some(eyr) = &self.eyr {
            let eyr: u16 = eyr.parse().unwrap();
            return (eyr >= 2020) && (eyr <= 2030);
        }
        false
    }

    fn is_valid_height(&self) -> bool {
        if let Some(hgt) = &self.hgt {
            // check if cm or inches.
            if let Some(height_in_inches) = hgt.strip_suffix("in") {
                let height_in_inches: u16 = height_in_inches.parse().unwrap();
                return (height_in_inches >= 59) && (height_in_inches <= 76);
            }

            if let Some(height_in_cm) = hgt.strip_suffix("cm") {
                let height_in_cm: u16 = height_in_cm.parse().unwrap();
                return (height_in_cm >= 150) && (height_in_cm <= 193);
            }
        }
        false
    }

    fn is_valid_hair_color(&self) -> bool {
        if let Some(hcl) = &self.hcl {
            if let Some(hex) = hcl.strip_prefix("#") {
                if hex.len() != 6 {
                    return false;
                }
                let allowed_chars = vec![
                    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
                ];
                return hex.chars().all(|c| allowed_chars.contains(&c));
            }
        }
        false
    }

    fn is_valid_eye_color(&self) -> bool {
        if let Some(ecl) = &self.ecl {
            let allowed_hair_colors = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
            return allowed_hair_colors.contains(&&ecl[..]);
        }
        false
    }

    fn is_valid_passport_id(&self) -> bool {
        if let Some(pid) = &self.pid {
            if pid.len() != 9 {
                return false;
            }

            match pid.parse::<u32>() {
                Ok(_) => return true,
                Err(_) => return false,
            }
        }
        false
    }

    pub fn is_valid(&self) -> bool {
        // Is all mandatory keys not None?

        if !self.is_valid_birth_year() {
            return false;
        }
        if !self.is_valid_issue_year() {
            return false;
        }
        if !self.is_valid_expiration_year() {
            return false;
        }
        if !self.is_valid_height() {
            return false;
        }
        if !self.is_valid_hair_color() {
            return false;
        }
        if !self.is_valid_eye_color() {
            return false;
        }
        if !self.is_valid_passport_id() {
            return false;
        }

        true
    }
}

pub fn part_two(input: &[String]) -> i32 {
    let input = input.join("\n");
    let input = input.split("\n\n");

    let mut passports: Vec<Passport> = Vec::new();

    for passport_string in input {
        let pass = Passport::new(passport_string);
        passports.push(pass);
    }

    let mut nbr_of_valid_passparts = 0;
    for passport in passports {
        if passport.is_valid() {
            nbr_of_valid_passparts += 1;
        }
    }

    nbr_of_valid_passparts
}

#[test]
fn part1_example_1() {
    let v = vec![
        "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd",
        "byr:1937 iyr:2017 cid:147 hgt:183cm",
        "",
        "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884",
        "hcl:#cfa07d byr:1929",
        "",
        "hcl:#ae17e1 iyr:2013",
        "eyr:2024",
        "ecl:brn pid:760753108 byr:1931",
        "hgt:179cm",
        "",
        "hcl:#cfa07d eyr:2025 pid:166559648",
        "iyr:2011 ecl:brn hgt:59in",
    ];

    let v: Vec<_> = v.iter().map(|x| x.to_string()).collect();

    assert_eq!(2, part_one(&v));
}

#[test]
fn part2_example_1() {
    let v = vec![
        "eyr:1972 cid:100",
        "hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926",
        "",
        "iyr:2019",
        "hcl:#602927 eyr:1967 hgt:170cm",
        "ecl:grn pid:012533040 byr:1946",
        "",
        "hcl:dab227 iyr:2012",
        "ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277",
        "",
        "hgt:59cm ecl:zzz",
        "eyr:2038 hcl:74454a iyr:2023",
        "pid:3556412378 byr:2007",
    ];

    let v: Vec<_> = v.iter().map(|x| x.to_string()).collect();

    assert_eq!(0, part_two(&v));
}
#[test]
fn part2_example_2() {
    let v = vec![
        "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980",
        "hcl:#623a2f",
        "",
        "eyr:2029 ecl:blu cid:129 byr:1989",
        "iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm",
        "",
        "hcl:#888785",
        "hgt:164cm byr:2001 iyr:2015 cid:88",
        "pid:545766238 ecl:hzl",
        "eyr:2022",
        "",
        "iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719",
    ];

    let v: Vec<_> = v.iter().map(|x| x.to_string()).collect();

    assert_eq!(4, part_two(&v));
}
