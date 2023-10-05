// python bytecode reverse engineering by @hacbit
use std::{env, process::exit};
mod color;
use color::color::*;
use color::color_utils::*;
use repybytecode::setup;

fn main() {
    // 使用宏快速创建一个ColorMode
    let blue = set_colormode!(
        front_color => FrontColor::Blue,
        mode => DisplayMode::NonBold
    );
    let yellow = set_colormode!(
        front_color => FrontColor::Yellow,
        mode => DisplayMode::Default
    );
    let green = set_colormode!(
        front_color => FrontColor::Green,
        mode => DisplayMode::Default
    );
    let bright_red = ColorMode::from(Color::Red);

    // 当指定 --no-warning 参数时，不显示警告信息
    if env::args().any(|arg| arg == "--no-warning") {
        // todo!();
    }
    else {
        println!(
            "[{}] {}",
            "!".to_color_string(&yellow),
            "this tool is work in progress, please use it carefully".to_color_string(&yellow)
        );
    }

    // 没有参数时提醒用户使用帮助
    if env::args().count() == 1 {
        println!(
            "[{}] {}",
            "-".to_color_string(&yellow),
            "use -h or --help to get help".to_color_string(&green)
        );
        exit(0);
    }
    let help = HELP.replace("{version}", VERSION);
    if env::args().any(|arg| arg == "-h" || arg == "--help") {
        println!("{}", help.to_color_string(&green));
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
                        "x".to_color_string(&bright_red),
                        "please specify a file name".to_color_string(&bright_red)
                    );
                    exit(0);
                })
        })
        .unwrap_or_else(|| {
            eprintln!(
                "[{}] {}",
                "x".to_color_string(&bright_red),
                "please specify a file name".to_color_string(&bright_red)
            );
            exit(0);
        });
    println!(
        "[{}] {}",
        "+".to_color_string(&blue),
        format!("loading file: {}", file_name).to_color_string(&blue)
    );
    println!(
        "[{}] {}",
        "+".to_color_string(&blue),
        "Decompiling...".to_color_string(&green)
    );
    setup(&file_name).unwrap_or_else(|err| {
        eprintln!(
            "[{}] Application error: {err}",
            "x".to_color_string(&bright_red),
            err = err.to_string().to_color_string(&bright_red)
        );
        exit(0);
    });
}

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const HELP: &'static str = r#"
    python bytecode reverse engineering by @hacbit
    usage: repybytecode [options] [file] [args] ...
    version: {version}"#;
const USAGE: &str = r#"
    options:
        -h, --help      show this help message and exit
        -f, --file      specify a file name
        --no-warning    do not show warning message
"#;