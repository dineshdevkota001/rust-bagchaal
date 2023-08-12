use std::io;

pub struct InputManager {
    pub user_input: String,
    stdin: io::Stdin,
}

impl InputManager {
    pub fn new() -> Self {
        InputManager {
            user_input: String::from(""),
            stdin: io::stdin(),
        }
    }
    pub fn ask_input(&mut self) -> &str {
        self.user_input = String::from("");
        self.stdin
            .read_line(&mut self.user_input)
            .expect("Cannot read line");
        return &self.user_input;
    }
    pub fn input_to_indices(&self) -> [i32; 2] {
        let strings = self.user_input.split_whitespace();
        let mut return_arr: [i32; 2] = [-1, -1];

        for (i, str) in strings.enumerate() {
            let num = str.trim().parse::<i32>();
            match num {
                Ok(num) => {
                    if num <= 25 || num > 0 {
                        return_arr[i] = num - 1;
                    }
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }

        return_arr
    }
    pub fn check_reset(&self) -> bool {
        self.user_input.to_lowercase().trim() == "reset"
    }
}
