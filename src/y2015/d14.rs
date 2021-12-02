const RACE_TIME: u32 = 2503;

pub fn solution_1(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let mut split = line.split_whitespace();
            split.advance_by(3).unwrap();
            let speed = u32::from_str_radix(split.next().unwrap(), 10).unwrap();
            split.advance_by(2).unwrap();
            let time = u32::from_str_radix(split.next().unwrap(), 10).unwrap();
            split.advance_by(6).unwrap();
            let rest = u32::from_str_radix(split.next().unwrap(), 10).unwrap();

            let cycle = time + rest;
            let run_part = time as f64 / cycle as f64;
            let cycles = RACE_TIME as f64 / cycle as f64;
            let run_cycles = cycles.floor() + (cycles.fract() / run_part).min(1.);

            let distance = (speed as f64 * time as f64 * run_cycles).round() as u32;
            distance
        })
        .max()
        .unwrap()
        .to_string()
}
struct Reindeer {
    speed: u32,
    rest_time: u32,
    run_time: u32,
    score: u32,
    position: u32,
}

impl Reindeer {
    fn update(&mut self, time: u32) -> u32 {
        let cycle = self.rest_time + self.run_time;
        let c = time % cycle;
        if c < self.run_time {
            self.position += self.speed;
        }
        self.position
    }
}
pub fn solution_2(input: &str) -> String {
    let mut deers: Vec<_> = input
        .lines()
        .map(|line| {
            let mut split = line.split_whitespace();
            split.advance_by(3).unwrap();
            let speed = u32::from_str_radix(split.next().unwrap(), 10).unwrap();
            split.advance_by(2).unwrap();
            let run_time = u32::from_str_radix(split.next().unwrap(), 10).unwrap();
            split.advance_by(6).unwrap();
            let rest_time = u32::from_str_radix(split.next().unwrap(), 10).unwrap();
            Reindeer {
                speed,
                rest_time,
                run_time,
                score: 0,
                position: 0,
            }
        })
        .collect();

    for i in 0..RACE_TIME {
        let leading = deers.iter_mut().map(|deer| deer.update(i)).max().unwrap();
        deers
            .iter_mut()
            .filter(|deer| deer.position == leading)
            .for_each(|deer| deer.score += 1);
    }
    deers
        .iter()
        .map(|deer| deer.score)
        .max()
        .unwrap()
        .to_string()
}
