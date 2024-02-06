use slog::{debug, error, info, o, warn, Drain, Logger};
use std::{error::Error, io};

pub fn shave(log: &Logger, yak: usize) -> Result<(), Box<dyn Error + 'static>> {
    debug!(log, "hello! I'm gonna shave a yak."; "excitement" => "yay!");
    if yak == 3 {
        warn!(log, "could not locate yak!");
        return Err(io::Error::new(io::ErrorKind::Other, "shaving yak failed!").into());
    } else {
        debug!(log, "yak shaved successfully");
    }
    Ok(())
}

pub fn shave_all(log: &Logger, yaks: usize) -> usize {
    info!(log, "shaving yaks");

    let mut yaks_shaved = 0;
    for yak in 1..=yaks {
        let res = shave(log, yak);
        debug!(log, "yak shaving result"; "yak" => yak, "shaved" => res.is_ok());

        if let Err(ref error) = res {
            // Like spans, events can also use the field initialization shorthand.
            // In this instance, `yak` is the field being initalized.
            error!(log, "failed to shave yak!"; "yak" => yak, "error" => error.to_string());
        } else {
            yaks_shaved += 1;
        }
        debug!(log, "{yaks_shaved}");
    }

    yaks_shaved
}

fn main() {
    let drain = slog_json::Json::new(std::io::stdout())
        .add_default_keys()
        .build()
        .fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let log = slog::Logger::root(drain, o!("app" => "slog-size"));

    let _ = shave_all(&log, 5);
}
