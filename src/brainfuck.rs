use std::collections::{HashMap, VecDeque};

fn build_brackets_mappings(code_bytes: &[u8]) -> Result<HashMap<usize, usize>, String> {
    let mut stack = VecDeque::with_capacity(code_bytes.len() / 10);
    let mut bracket_map = HashMap::with_capacity(code_bytes.len() / 10);

    for (i, &bytes) in code_bytes.iter().enumerate() {
        match bytes {
            b'[' => stack.push_back(i),
            b']' => {
                match stack.pop_back() {
                    Some(open_index) => {
                        bracket_map.insert(open_index, i);
                        bracket_map.insert(i, open_index);
                    },
                    None => return Err(format!("Error: Unmatched ']' at position {}", i)),
                }
            }
            _ => {}
        }
    }

    if !stack.is_empty() {
        return Err(format!("Error: Unmatched '[' at position {}", stack[0]));
    }

    Ok(bracket_map)
}

pub fn brainfuck_interpreter(code: Vec<u8>, input: Option<Vec<u8>>) {
    const MEMORY_SIZE: usize = 30000;
    let mut memory: [u8; MEMORY_SIZE] = [0; MEMORY_SIZE];
    
    let mut pointer: usize = 0;
    let mut code_pointer: usize = 0;
    let mut input_poiner: usize = 0;

    let bracket_map = match build_brackets_mappings(&code) {
        Ok(map) => map,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    while code_pointer < code.len() {
        let command = code[code_pointer];

        match command {
            b'>' => {
                pointer = (pointer + 1) % MEMORY_SIZE;
            }
            b'<' => {
                pointer = (pointer + MEMORY_SIZE - 1) % MEMORY_SIZE;
            }
            b'+' => {
                memory[pointer] = memory[pointer].wrapping_add(1);
            }
            b'-' => {
                memory[pointer] = memory[pointer].wrapping_sub(1);
            }
            b'.' => {
                print!("{}", memory[pointer] as char);
            }
            b',' => {
                if let Some(ref input_str) = input {
                    if input_poiner < input_str.len() {
                        memory[pointer] = input_str[input_poiner];
                        input_poiner += 1;
                    } else {
                        memory[pointer] = 0;
                    }
                }
            }
            b'[' => {
                if memory[pointer] == 0 {
                    code_pointer = *bracket_map.get(&code_pointer).unwrap();
                }
            }
            b']' => {
                if memory[pointer] != 0 {
                    code_pointer = *bracket_map.get(&code_pointer).unwrap();
                }
            }
            _ => {}
        }

        code_pointer += 1;
    }
}