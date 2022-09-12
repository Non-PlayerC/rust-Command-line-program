use std::error::Error;
use std::fs;
pub struct Config {
    pub query: String,
    pub filename: String,
}
impl Config {
    //! 创造一个 Config 的构造函数
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        //' 检查 slice 的长度
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    for line in search(&config.query, &contents) {
        println!("{}", line) //循环并打印 line 的每一行
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        // 对文本行进行操作
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

// 编写失败测试
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
