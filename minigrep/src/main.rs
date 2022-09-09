use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2]; //- 收集命令行的参数并保存到 vector 中

    //-- 读取文件内容
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something now wrong reading the file"); //-- 读取文件测试

    print!("With text:\n{}", contents)
}
