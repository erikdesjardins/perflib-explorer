use windows::core::Error;

mod opt;

fn main() -> Result<(), Error> {
    let opt::Options { verbose } = clap::Parser::parse();

    env_logger::Builder::new()
        .filter_level(match verbose {
            0 => log::LevelFilter::Info,
            1 => log::LevelFilter::Debug,
            _ => log::LevelFilter::Trace,
        })
        .init();

    Ok(())
}
