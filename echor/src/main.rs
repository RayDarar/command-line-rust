use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Cli {
    /// Input text
    #[arg(required = true)]
    text: Vec<String>,

    /// Do not print newline
    #[arg(short = 'n')]
    no_newline: bool,
}

fn main() {
    let args = Cli::parse();

    let eol = if args.no_newline { "" } else { "\n" };
    print!("{}{}", args.text.join(" "), eol);
}
