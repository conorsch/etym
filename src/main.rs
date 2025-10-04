use clap::{arg, command};

// static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

fn main() -> anyhow::Result<()> {
    // TODO 'word' should be a manadatory argument
    let matches = command!()
        .arg(arg!([word] "Word for which etymology is desired"))
        .arg(arg!(
            -d --debug ... "Turn debugging information on"
        ))
        .get_matches();

    if let Some(word) = matches.value_of("word") {
        let e = match etym::Etymology::new(word) {
            Ok(r) => r,
            Err(_) => return Err(anyhow::anyhow!("Failed to find an etymology for '{word}'")),
        };
        // Manually bold the search term
        println!("\x1b[1m{}\x1b[22m", e.label);
        let w = textwrap::termwidth();
        let s = textwrap::fill(&e.etymology, w);
        println!("{s}");
    } else {
        return Err(anyhow::anyhow!("No word to search for was supplied"));
    }
    Ok(())
}
