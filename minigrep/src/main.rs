use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("==> Searching for {}", config.query);
    println!("==> In file {}", config.filename);

    // 处理 main 函数中的错误逻辑
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error : {}", e); //将错误打印到控制台，并将内容输出到指定文件

        process::exit(1)
    }
}
