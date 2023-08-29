use std::time::Instant;

use windows::core::Result;

mod fetch;
mod opt;
mod print;
mod types;
mod winapi;

fn main() -> Result<()> {
    let opt::Options { verbose, command } = clap::Parser::parse();

    env_logger::Builder::new()
        .filter_level(match verbose {
            0 => log::LevelFilter::Info,
            1 => log::LevelFilter::Debug,
            _ => log::LevelFilter::Trace,
        })
        .init();

    let start = Instant::now();

    let mut buf = Vec::new();

    let all = fetch::all_providers(&mut buf)?;

    log::info!("Load completed at T + {}ms", start.elapsed().as_millis());

    match command {
        opt::Command::Summary => print::summary(&all),
        opt::Command::Counterset(opt::Counterset { guid }) => print::counterset(&all, &guid),
    }

    log::info!("Print completed at T + {}ms", start.elapsed().as_millis());

    Ok(())
}
