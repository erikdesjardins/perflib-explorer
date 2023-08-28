use windows::core::Result;

mod opt;
mod winapi;
mod fetch;
mod types;

fn main() -> Result<()> {
    let opt::Options { verbose } = clap::Parser::parse();

    env_logger::Builder::new()
        .filter_level(match verbose {
            0 => log::LevelFilter::Info,
            1 => log::LevelFilter::Debug,
            _ => log::LevelFilter::Trace,
        })
        .init();

    let mut buf = Vec::new();

    let all = fetch::all_providers(&mut buf)?;

    for p in all {
        println!("{:?}: {}", p.id, p.name);
        for cs in p.countersets {
            println!("    {:?}: {}", cs.id, cs.name);
            for c in cs.counters {
                println!("        -- {:?}: {}; {}", c.id, c.name, c.help);
            }
        }
        println!();
    }

    Ok(())
}
