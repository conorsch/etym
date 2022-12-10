use clap::{arg, command};

// static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

fn main() {
    let matches = command!()
        .arg(arg!([word] "Word for which etymology is desired"))
        .arg(arg!(
            -d --debug ... "Turn debugging information on"
        ))
        .get_matches();

    if let Some(word) = matches.value_of("word") {
        let e = etym::Etymology::new(word).unwrap();
        // Manually bold the search term
        println!("\x1b[1m{}\x1b[22m", e.label);
        let w = textwrap::termwidth();
        let s = textwrap::fill(&e.etymology, w);
        println!("{}", s);
    }
}
