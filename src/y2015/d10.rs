fn look_and_say(input: &str) -> String {
    let mut result = String::new();
    let mut count = 0;
    let mut last = '_';
    for char in input.chars() {
        if char != last {
            if count > 0 {
                result.push_str(count.to_string().as_str());
                result.push(last);
            }
            count = 1;
            last = char;
        } else {
            count += 1;
        }
    }
    result.push_str(count.to_string().as_str());
    result.push(last);

    result
}

pub fn solution_1(input: &str) -> String {
    let mut input = look_and_say(input);
    for _ in 0..39 {
        input = look_and_say(input.as_str())
    }
    input.len().to_string()
}

pub fn solution_2(input: &str) -> String {
    let mut input = look_and_say(input);
    for _ in 0..49 {
        input = look_and_say(input.as_str())
    }
    input.len().to_string()
}
