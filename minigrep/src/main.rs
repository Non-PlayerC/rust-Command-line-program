use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("解析参数的问题: {}", err);
        process::exit(1);
    });

    println!("查找 {}", config.query);
    println!("文件 {}", config.filename);

    // 处理 main 函数中的错误逻辑
    if let Err(e) = minigrep::run(config) {
        println!("Application error : {}", e);

        process::exit(1)
    }
}
