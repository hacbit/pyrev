// python bytecode reverse engineering by @hacbit
use std::{env, fs::File, process::exit};
mod color;
use color::color::*;
use color::color_utils::*;
use repybytecode::setup;

#[macro_use]
extern crate lazy_static;

fn main() -> std::io::Result<()> {
    let file_name = get_file_name();
    // 获取file的绝对路径
    let file_path = std::fs::canonicalize(&file_name)?; // 返回的是UNC路径, 例如: \\?\C:\Users\hacbit\test
                                                        // 转换为标准路径, 例如: C:\Users\hacbit\test
    let file_path = file_path.to_str().unwrap().split_at(4).1;

    println!(
        "[{}] {} {}",
        "*".to_color_string(&BLUE),
        format!("loading file: {}", file_name).to_color_string(&BLUE),
        format!("({})", file_path)
    );
    // 尝试打开文件
    let mut file = File::open(&file_name).unwrap_or_else(|err| {
        eprintln!(
            "[{}] Application error: {err}",
            "x".to_color_string(&BRIGHT_RED),
            err = err.to_string().to_color_string(&BRIGHT_RED)
        );
        exit(0);
    });
    println!(
        "[{}] {}",
        "*".to_color_string(&BLUE),
        "Decompiling...".to_color_string(&GREEN)
    );
    setup(&mut file).unwrap_or_else(|err| {
        eprintln!(
            "[{}] Application error: {err}",
            "x".to_color_string(&BRIGHT_RED),
            err = err.to_string().to_color_string(&BRIGHT_RED)
        );
        exit(0);
    });
    Ok(())
}

lazy_static! {
    static ref BLUE: ColorMode = set_colormode!(
        front_color => FrontColor::Blue,
        mode => DisplayMode::Highlight
    );
    static ref YELLOW: ColorMode = set_colormode!(
        front_color => FrontColor::Yellow,
        mode => DisplayMode::Default
    );
    static ref GREEN: ColorMode = set_colormode!(
        front_color => FrontColor::Green
    );
    static ref BRIGHT_RED: ColorMode = set_colormode!(
        front_color => FrontColor::Red,
        mode => DisplayMode::Highlight
    );
}

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const HELP: &'static str = r#"
    python bytecode reverse engineering by @hacbit
    usage: repybytecode [options] [file] [args] ...
    version: {version}"#;
const USAGE: &'static str = r#"
    options:
        -h, --help      show this help message and exit
        -f, --file      specify a file name
        -o, --output    specify a output file name(还没实装)
"#;

fn get_file_name() -> String {
    println!(
        "[{}] {}",
        "!".to_color_string(&YELLOW),
        "this tool is work in progress, please use it carefully".to_color_string(&YELLOW)
    );

    // 没有参数时提醒用户使用帮助
    if env::args().count() == 1 {
        println!(
            "[{}] {}",
            "-".to_color_string(&YELLOW),
            "use -h or --help to get help".to_color_string(&GREEN)
        );
        exit(0);
    }
    let help = HELP.replace("{version}", VERSION);
    if env::args().any(|arg| arg == "-h" || arg == "--help") {
        println!("{}", help.to_color_string(&GREEN));
        println!("{}", USAGE);
        exit(0);
    }
    // 使用 -f 或 --file 参数来指定文件名
    let file_name = env::args()
        .find(|arg| arg == "-f" || arg == "--file")
        .map(|_| {
            env::args()
                .skip_while(|arg| arg != "-f" && arg != "--file")
                .skip(1)
                .next()
                .unwrap_or_else(|| {
                    eprintln!(
                        "[{}] {}",
                        "x".to_color_string(&BRIGHT_RED),
                        "please specify a file name".to_color_string(&BRIGHT_RED)
                    );
                    exit(0);
                })
        })
        .unwrap_or_else(|| {
            eprintln!(
                "[{}] {}",
                "x".to_color_string(&BRIGHT_RED),
                "please specify a file name".to_color_string(&BRIGHT_RED)
            );
            exit(0);
        });
    file_name
}
