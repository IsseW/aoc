use crate::helpers::extract_numbers;

pub fn solution_1(input: &str) -> String {
    let (row, column) = if let [row, column] = extract_numbers::<i32>(input)[..] {
        (row, column)
    } else {
        return "Input error".into();
    };
    fn transform(i: u128) -> u128 {
        (i * 252533) % 33554393
    }
    let mut d = 20151125;
    let (mut x, mut y) = (1, 1);
    while (y, x) != (row, column) {
        x += 1;
        y -= 1;
        if y < 1 {
            y = x;
            x = 1;
        }
        d = transform(d);
    }

    d.to_string()
}

pub fn solution_2(_: &str) -> String {
    "Lessgo".into()
}
