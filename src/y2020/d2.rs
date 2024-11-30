struct Entry<'a> {
    min: usize,
    max: usize,
    c: char,
    password: &'a str,
}

impl Entry<'_> {
    fn is_valid1(&self) -> bool {
        let c = self.password.chars().filter(|c| *c == self.c).count();
        self.min <= c && c <= self.max
    }

    fn is_valid2(&self) -> bool {
        let mut password = self.password.chars();
        ({
            let _ = password.advance_by(self.min - 1);
            password.next() == Some(self.c)
        }) ^ ({
            let _ = password.advance_by(self.max - self.min - 1);
            password.next() == Some(self.c)
        })
    }
}

fn parse(input: &str) -> impl Iterator<Item = Entry<'_>> + '_ {
    input.lines().map(|line| {
        let mut inp = line.split_whitespace();

        let (min, max) = inp.next().unwrap().split_once('-').unwrap();
        let (min, max) = (min.parse().unwrap(), max.parse().unwrap());

        let c = inp.next().unwrap().chars().next().unwrap();

        let password = inp.next().unwrap();

        Entry {
            min,
            max,
            c,
            password,
        }
    })
}

pub fn solution_1(input: &str) -> String {
    parse(input).filter(Entry::is_valid1).count().to_string()
}

pub fn solution_2(input: &str) -> String {
    parse(input).filter(Entry::is_valid2).count().to_string()
}
