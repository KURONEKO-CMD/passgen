use passgen::{generate_password,ask_yes_no_question};
use std::io::{self, Write};

fn main() {

    let length:usize = loop {
        print!{"请输入密码长度: "};
        //立即显示上面的提示，而不是等待换行
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
               .read_line(&mut input)
               .expect("无法读取行");

        match input.trim().parse(){
            Ok(num) => break num,
            Err(_) => {
                println!("输入无效，请输入一个数字。");
                continue;
            }
        }
    };

    
    let use_uppercase = ask_yes_no_question("是否有大写字母? 请输入 y/n 或者 yes/no: ");
    let use_lowercase = ask_yes_no_question("是否有小写字母? 请输入 y/n 或者 yes/no: ");
    let use_number = ask_yes_no_question("是否有数字? 请输入 y/n 或者 yes/no: ");
    let use_symbols = ask_yes_no_question("是否有特殊符号? 请输入 y/n 或者 yes/no: ");

    let condition = use_uppercase || use_lowercase || use_number || use_symbols;

    if condition {
            let password = generate_password(length,use_uppercase,use_lowercase,use_number,use_symbols);
            println!("My Password: {}",password);
    } else {
        println!("没有任何可用字符，你无法生成密码！")
    }



}
