fn main() {
    let mut data: [u8; 30_000] = [0; 30_000];
    let mut d_pointer = 0;
    const RAW_INSTRUCTIONS: &'static str = "+++.>++.>+.<-----.";
    let instructions: [char; RAW_INSTRUCTIONS.len()] = RAW_INSTRUCTIONS.chars().into_iter().collect::<Vec<char>>().try_into().unwrap();

    for i_pointer in 0..instructions.len() {
        let i = instructions[i_pointer];
        
        match i {
            '>' => d_pointer += 1,
            '<' => d_pointer -= 1,
            '+' => data[d_pointer] += 1,
            '-' => sub_assign(data[d_pointer], 1),
            '.' => println!("{}", data[d_pointer]),
            ',' => data[d_pointer] = input().unwrap(),
            _ => {},
        }
    }
}

fn sub_assign(mut lhs: u8, rhs: u8) {
    let first = lhs as i16 - rhs as i16;

    lhs = if first < 0 {
        (256 + first) as u8
    } else {
        first as u8
    };
}

fn input() -> Result<u8, std::num::ParseIntError> {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.parse()
}
