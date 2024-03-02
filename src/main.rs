fn main() {
    let mut data: [u8; 30_000] = [0; 30_000];
    let mut d_pointer = 15_000;
    const RAW_INSTRUCTIONS: &'static str = ",.";
    let instructions: [char; RAW_INSTRUCTIONS.len()] = RAW_INSTRUCTIONS.chars().into_iter().collect::<Vec<char>>().try_into().unwrap();

    let mut i_pointer = 0;
    while i_pointer < instructions.len() {
        let i = instructions[i_pointer];
        
        match i {
            '>' => d_pointer += 1,
            '<' => d_pointer -= 1,
            '+' => data[d_pointer] = inc(data[d_pointer]),
            '-' => data[d_pointer] = dec(data[d_pointer]),
            '.' => println!("{}", data[d_pointer] as char),
            ',' => data[d_pointer] = input().unwrap(),
            '[' => {
                if data[d_pointer] == 0 {
                    i_pointer = i_pointer + find_matching_forward(&instructions[i_pointer..instructions.len()]).unwrap();
                }
            }
            ']' => {
                if data[d_pointer] != 0 {
                    i_pointer = find_matching_backward(&instructions[0..i_pointer]).unwrap();
                }
            }
            _ => {},
        }
        i_pointer += 1;
    }
}

fn dec(num: u8) -> u8 {
    let dec_num = num as i16 - 1;

    if dec_num < 0 {
        (256 + dec_num) as u8
    } else {
        dec_num as u8
    }
}

fn inc(num: u8) -> u8 {
    let inc_num = num as i16 + 1;

    if inc_num > 255 {
        (inc_num - 256) as u8
    } else {
        inc_num as u8
    }
}

fn find_matching_forward(instructions: &[char]) -> Option<usize> {
    let mut caps_to_skip = 0;

    for i_pointer in 1..instructions.len() {
        let i = instructions[i_pointer];

        if i == '[' {
            caps_to_skip += 1;
        } else if i == ']' {
            if caps_to_skip == 0 {
                return Some(i_pointer);
            } else {
                caps_to_skip -= 1;
            }
        }
    }

    None
}

fn find_matching_backward(instructions: &[char]) -> Option<usize> {
    let mut caps_to_skip = 0;

    for i_pointer in (1..instructions.len()).rev() {
        let i = instructions[i_pointer];

        if i == ']' {
            caps_to_skip += 1;
        } else if i == '[' {
            if caps_to_skip == 0 {
                return Some(i_pointer);
            } else {
                caps_to_skip -= 1;
            }
        }
    }

    None
}

fn input() -> Result<u8, std::num::ParseIntError> {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.parse()
}
