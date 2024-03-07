use clap::{App, Arg, ArgMatches};

fn main() {
    let matches: ArgMatches = App::new("echor")
        .version("0.1.0")
        .author("Shunta Nakamura")
        .about("Rust echo")
        .arg(
            Arg::with_name("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("omit_newline")
                .short("n")
                .help("Do not print newline")
                .takes_value(false)
        )
        .get_matches();
    let text: Vec<String> = matches.values_of_lossy("text").unwrap();
    let omit_newline: bool = matches.is_present("omit_newline");
    print!("{}{}", text.join(" "), if omit_newline { "" } else { "\n" });
}
