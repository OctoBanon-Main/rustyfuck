pub fn brainfuck_interpreter(code: String, input: Option<Vec<u8>>) {
    const MEMORY_SIZE: usize = 30000;
    let mut memory: Vec<u8> = vec![0; MEMORY_SIZE];
    
    let mut pointer: usize = 0;
    let mut code_pointer: usize = 0;
    let mut input_poiner: usize = 0;

    let code_bytes = code.as_bytes();
    let code_len = code.len();

    while code_pointer < code_len {
        let command = code_bytes[code_pointer] as char;

        match command {
            '>' => {
                pointer = (pointer + 1) % MEMORY_SIZE;
            }
            '<' => {
                pointer = (pointer + MEMORY_SIZE - 1) % MEMORY_SIZE;
            }
            '+' => {
                memory[pointer] = memory[pointer].wrapping_add(1);
            }
            '-' => {
                memory[pointer] = memory[pointer].wrapping_sub(1);
            }
            '.' => {
                print!("{}", memory[pointer] as char);
            }
            ',' => {
                if let Some(ref input_str) = input {
                    if input_poiner < input_str.len() {
                        memory[pointer] = input_str[input_poiner];
                        input_poiner += 1;
                    } else {
                        memory[pointer] = 0;
                    }
                }
            }
            '[' => {
                if memory[pointer] == 0 {
                    let mut open_loops = 1;
                    while open_loops > 0 {
                        code_pointer += 1;
                        if code_pointer >= code_len {
                            eprintln!("Error: Unmatched '[' at position {}", code_pointer);
                            return;
                        }
                        match code_bytes[code_pointer] as char {
                            '[' => open_loops += 1,
                            ']' => open_loops -= 1,
                            _ => {}
                        }
                    }
                }
            }
            ']' => {
                if memory[pointer] != 0 {
                    let mut close_loops = 1;
                    while close_loops > 0 {
                        if code_pointer == 0 {
                            eprintln!("Error: Unmatched ']' at position {}", code_pointer);
                            return;
                        }
                        code_pointer -= 1;
                        match code_bytes[code_pointer] as char {
                            ']' => close_loops += 1,
                            '[' => close_loops -= 1,
                            _ => {}
                        }
                    }
                }
            }
            _ => {}
        }

        code_pointer += 1;
    }
}