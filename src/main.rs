use windows::core::Result;

mod fetch;
mod opt;
mod types;
mod winapi;

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
            println!("    {:?}: {}; {}", cs.id, cs.name, cs.help);
            for c in cs.counters {
                println!("        -- {:?}: {}; {}", c.id, c.name, c.help);
            }
            if let Some(instances) = cs.instances {
                for i in instances {
                    println!("        >> {:?}: {}", i.id, i.name);
                }
            }
        }
        println!();
    }

    Ok(())
}
