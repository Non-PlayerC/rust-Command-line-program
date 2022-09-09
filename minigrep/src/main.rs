use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2]; //- 收集命令行的参数并保存到 vector 中

    println!("Searching for {}", query);
    println!("In file {}", filename);
}
