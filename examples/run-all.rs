use colored::Colorize;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

macro_rules! add_file {
    (
        $(
            --$name:ident $value:expr
        ),*$(,)?
    ) => {
        vec![
            $(
                concat!("--", stringify!($name)), $value
            ),+
        ]
    };
}

fn main() -> Result<()> {
    let mut cmd = std::process::Command::new("cargo");

    cmd.arg("run");
    if !cfg!(debug_assertions) && !cfg!(test) {
        cmd.arg("--release");
    }
    
    cmd.arg("--").args(add_file!(
        --file "test/attr.txt",
        --file "test/container.txt",
        --file "test/def.txt",
        --file "test/for.txt",
        --file "test/import.txt",
        --file "test/op.txt",
    ));

    let output = cmd.output()?;

    if output.stdout.is_empty() {
        eprintln!("{}", "Error:".bright_red());
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
    } else {
        println!("{}", "Success:".bright_green());
        println!("{}", String::from_utf8_lossy(&output.stdout))
    }

    Ok(())
}