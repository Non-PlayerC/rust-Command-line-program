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
            return Err("未提供数据");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);

    Ok(())
}
