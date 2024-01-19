extern crate flate2;

use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::env::Args;
use std::fs::File;
use std::io::copy;
use std::io::BufReader;
use std::iter::Skip;
use std::process;
use std::time::Instant;

pub struct Config {
    input_path: String,
    output_path: String,
}

impl Config {
    fn new(mut args: Skip<Args>) -> Result<Config, &'static str> {
        if args.len() != 2 {
            return Err("参数不合法");
        }
        let (input_path, output_path) = (args.next(), args.next());

        Ok(Config {
            input_path: match input_path {
                Some(r) => r,
                None => return Err("无法找到参数input path"),
            },
            output_path: match output_path {
                Some(r) => r,
                None => return Err("无法找到output path"),
            },
        })
    }
}

fn main() {
    // 处理args
    let config = Config::new(args().skip(1)).unwrap_or_else(|err: &str| {
        eprintln!("读取文件失败 err:{}", err);
        process::exit(1)
    });
    // 创建输入流
    let mut input = BufReader::new(File::open(config.input_path).unwrap());
    // 创建文件
    let output = File::create(config.output_path).unwrap();
    let mut encoder = GzEncoder::new(output, Compression::default());
    // 开始时间
    let start_time = Instant::now();
    copy(&mut input, &mut encoder).unwrap();
    let output = encoder.finish().unwrap();
    println!("原始长度:{:?}", input.get_ref().metadata().unwrap().len());
    println!("现在长度:{:?}", output.metadata().unwrap().len());
    println!("操作用时:{:?}", start_time.elapsed());
}
