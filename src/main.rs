type Data = [u8; 30_000];
type Instructions = Vec<char>;
#[derive(Debug)]
enum Mode {
    Num,
    Char,
}

#[derive(Debug)]
struct Program {
    mode: Mode,
    data: Data,
    instructions: Instructions,
    d_pointer: usize,
    i_pointer: usize
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut program = Program {
        data: [0; 30_000],
        mode: Mode::Num,
        instructions: vec![',', '.'],
        d_pointer: 0,
        i_pointer: 0,
    };

    if args.len() == 2 {
        program.mode = Mode::Num;
        program.instructions = args[1].chars().into_iter().collect::<Vec<char>>();
    } else if args.len() == 3 {
        if args[1] == "0" {
            program.mode = Mode::Num;
        } else if args[1] == "1" {
            program.mode = Mode::Char;
        } else {
            error("Unsupported mode!");
        }

        program.instructions = args[2].chars().into_iter().collect::<Vec<char>>();
    } else {
        error("Unsupported args length!");
    }

    println!("\x1b[1;32mBRAIN \x1b[1;31mUHOH \x1b[1;32mINTERPRETER\x1b[0m");

    while program.i_pointer < program.instructions.len() {
        let i = program.instructions.get(program.i_pointer).expect("Failed to parse program instructions");
        
        match i {
            '>' => parse_right(&mut program),
            '<' => parse_left(&mut program),
            '+' => parse_plus(&mut program),
            '-' => parse_minus(&mut program),
            '.' => parse_dot(&program),
            ',' => parse_comma(&mut program),
            '[' => parse_forward(&mut program),
            ']' => parse_backward(&mut program),
            _ => {},
        }
        program.i_pointer += 1;
    }
}

fn parse_right(program: &mut Program) {
    if program.d_pointer == 29_999 {
        error("Cannot move data pointer to the right!");
    } else {
        program.d_pointer += 1;
    }
}

fn parse_left(program: &mut Program) {
    if program.d_pointer == 0 {
        error("Cannot move data pointer to the left!");
    } else {
        program.d_pointer -= 1;
    }
}

fn parse_plus(program: &mut Program) {
    let inc = program.data[program.d_pointer] as i16 + 1;

    program.data[program.d_pointer] = if inc < 0 {
        (inc - 256) as u8
    } else {
        inc as u8
    };
}

fn parse_minus(program: &mut Program) {
    let dec = program.data[program.d_pointer] as i16 - 1;

    program.data[program.d_pointer] = if dec < 0 {
        (dec - 256) as u8
    } else {
        dec as u8
    };
}

fn parse_dot(program: &Program) {
    match program.mode {
        Mode::Num => println!("{}", program.data[program.d_pointer]),
        Mode::Char => println!("{}", program.data[program.d_pointer] as char),
    }
}

fn parse_comma(program: &mut Program) {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input from user");
    
    program.data[program.d_pointer] = match program.mode {
        Mode::Num => {
            match input.chars().next().expect("Failed to read input").to_digit(10) {
                Some(x) => x as u8,
                None => { 
                    error("Didn't enter number in 'Num' mode");
                    unreachable!();
                }
            }
        }
        Mode::Char => input.as_bytes()[0],
    }
}

fn parse_forward(program: &mut Program) {
    if program.data[program.d_pointer] == 0 {
        program.i_pointer = program.i_pointer + find_matching_forward(&(program.instructions)[program.i_pointer..program.instructions.len()]).unwrap();
    }
}

fn parse_backward(program: &mut Program) {
    if program.data[program.d_pointer] != 0 {
        program.i_pointer = find_matching_backward(&(program.instructions)[0..program.i_pointer]).unwrap();
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

fn error(m: &str) {
    panic!("\x1b[1;31mError: \x1b[1;39m{m}\x1b[0m");
}
