extern crate chrono;
extern crate fern;
extern crate log;

use std::io;

pub fn setup_logging(is_verbose: bool) -> Result<(), fern::InitError> {
    let mut base_config = fern::Dispatch::new();

    if is_verbose {
        base_config = base_config.level(log::LevelFilter::Debug);
    } else {
        base_config = base_config.level(log::LevelFilter::Info);
    }

    let stdout_config = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}] [{}:{}] [{}]: {}",
                chrono::Local::now().format("%H:%M:%S"),
                record.file().expect("No filename"),
                record.line().expect("No line"),
                record.level(),
                message
            ))
        })
        .chain(io::stdout());

    base_config.chain(stdout_config).apply()?;

    Ok(())
}
