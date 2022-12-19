
#[derive(Default, Debug)]
struct PassportPre<'a> {
	byr: Option<&'a str,>,
	iyr: Option<&'a str,>,
	eyr: Option<&'a str,>,
	hgt: Option<&'a str>,
	hcl: Option<&'a str>,
	ecl: Option<&'a str>,
	pid: Option<&'a str>,
	cid: Option<&'a str>,
}

#[derive(Debug)]
struct Passport<'a> {
	byr: &'a str,
	iyr: &'a str,
	eyr: &'a str,
	hgt: &'a str,
	hcl: &'a str,
	ecl: &'a str,
	pid: &'a str,
	cid: Option<&'a str>,
}



fn parse(input: &str) -> impl Iterator<Item = Passport> + '_ {
	input.split("\n\n").filter_map(|input| {
		let mut passport = PassportPre::default();
		
		for item in input.split_whitespace() {
			let (key, value) = item.split_once(':').unwrap();
			match key {
				"byr" => passport.byr = Some(value),
				"iyr" => passport.iyr = Some(value),
				"eyr" => passport.eyr = Some(value),
				"hgt" => passport.hgt = Some(value),
				"hcl" => passport.hcl = Some(value),
				"ecl" => passport.ecl = Some(value),
				"pid" => passport.pid = Some(value),
				"cid" => passport.cid = Some(value),
				_ => panic!("Unexpected key: {key}"),
			}
		}

		Some(Passport {
			byr: passport.byr?,
			iyr: passport.iyr?,
			eyr: passport.eyr?,
			hgt: passport.hgt?,
			hcl: passport.hcl?,
			ecl: passport.ecl?,
			pid: passport.pid?,
			cid: passport.cid,
		})
	})
}

pub fn solution_1(input: &str) -> String {
	parse(input).count().to_string()
}

pub fn solution_2(input: &str) -> String {
	parse(input).enumerate().filter(|(i, passport)| {
		passport.byr.parse::<u32>().map_or(false, |y| (1920..=2002).contains(&y)) &&
		passport.iyr.parse::<u32>().map_or(false, |y| (2010..=2020).contains(&y)) &&
		passport.eyr.parse::<u32>().map_or(false, |y| (2020..=2030).contains(&y)) &&
		(
			passport.hgt.strip_suffix("in").and_then(|s| s.parse::<u32>().ok()).map_or(false, |hgt| (59..=76).contains(&hgt)) ||
			passport.hgt.strip_suffix("cm").and_then(|s| s.parse::<u32>().ok()).map_or(false, |hgt| (150..=193).contains(&hgt))
		)
		&&
		passport.hcl.strip_prefix('#').map_or(false, |s| s.chars().filter(|c| ('0'..='9').contains(c) || ('a'..='f').contains(c)).count() == s.len())
		&&
		matches!(passport.ecl, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth")
		&&
		(passport.pid.len() == 9 && passport.pid.chars().filter(|c| ('0'..='9').contains(c)).count() == 9)
	}).count().to_string()
}
