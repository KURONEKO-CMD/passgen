use rand::Rng;
use std::io::{self, Write};

pub fn generate_password(
    length: usize,
    use_uppercase: bool,
    use_lowercase: bool,
    use_number: bool,
    use_symbols: bool,
) -> String {
    const LOWERCASE_CHARS: &str = "abcdefghijklmnopqrstuvwxyz";
    const UPPERCASE_CHARS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const NUMBER_CHARS: &str = "0123456789";
    const SYMBOL_CHARS: &str = "!@#$%^&*()_+-=[]{}|;:,.<>?";

    let mut charset = String::new();

    if use_lowercase {
        charset.push_str(LOWERCASE_CHARS);
    }

    if use_uppercase {
        charset.push_str(UPPERCASE_CHARS);
    }

    if use_number {
        charset.push_str(NUMBER_CHARS);
    }

    if use_symbols {
        charset.push_str(SYMBOL_CHARS);
    }

    let mut rng = rand::rng();

    let mut password = String::new();

    for _ in 0..length {
        let random_index = rng.random_range(0..charset.len());
        let random_char = charset.chars().nth(random_index).unwrap();
        password.push(random_char);

    }

    password
}

pub fn ask_yes_no_question(message: &str) -> bool {
    loop {
        print!("{}",message);
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        
        io::stdin()
               .read_line(&mut input)
               .expect("无法读取行");
        
        // to_lowercase() 把输入转为小写，比如 "Yes" -> "yes"
        // as_str() 是一个技术步骤，把 String 类型转为 &str 来进行匹配

        match input.trim().to_lowercase().as_str() {
            "y" | "yes" => break true,
            "n" | "no" => break false,

            _ => {  // _ 是一个通配符，代表任何其他情况
                println!("输入无效，请输入 y/n 或者 yes/no。");
                continue;
            }
        }

    }

}