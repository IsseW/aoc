fn is_valid(password: &[u8]) -> bool {
    let mut increasing = false;
    let mut doubles = 0;
    let mut last_counted_double = 0;
    let mut last_last = 1;
    let mut last = 1;
    for (index, &byte) in password.iter().enumerate() {
        if last == byte - 1 && last_last == last - 1 {
            increasing = true;
            if doubles >= 2 {
                return true;
            }
        }
        if byte == last && last_counted_double <= index - 2 {
            last_counted_double = index;
            doubles += 1;
            if increasing && doubles >= 2 {
                return true;
            }
        }

        match byte as char {
            'i' | 'o' | 'l' => return false,
            _ => {}
        }

        last_last = last;
        last = byte;
    }
    increasing && doubles >= 2
}

fn increment(password: &mut [u8], position: usize) {
    password[position] += 1;
    if password[position] > b'z' {
        password[position] = b'a';
        if position > 0 {
            increment(password, position - 1);
        }
    }
}

pub fn solution_1(input: &str) -> String {
    let mut input = input.to_string();
    let password = unsafe { input.as_bytes_mut() };

    while !is_valid(password) {
        increment(password, password.len() - 1);
    }

    input
}

pub fn solution_2(input: &str) -> String {
    let mut input = input.to_string();
    let password = unsafe { input.as_bytes_mut() };

    while !is_valid(password) {
        increment(password, password.len() - 1);
    }
    increment(password, password.len() - 1);
    while !is_valid(password) {
        increment(password, password.len() - 1);
    }

    input
}
