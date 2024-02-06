use log::{debug, error, info, warn};
use std::{error::Error, io};

pub fn shave(yak: usize) -> Result<(), Box<dyn Error + 'static>> {
    debug!("hello! I'm gonna shave a yak.");
    if yak == 3 {
        warn!("could not locate yak!");
        return Err(io::Error::new(io::ErrorKind::Other, "shaving yak failed!").into());
    } else {
        debug!("yak shaved successfully");
    }
    Ok(())
}

pub fn shave_all(yaks: usize) -> usize {
    info!("shaving yaks");

    let mut yaks_shaved = 0;
    for yak in 1..=yaks {
        let res = shave(yak);
        debug!(
            "yak {yak} is {}",
            if res.is_ok() { "shaved" } else { "not shaved" }
        );

        if let Err(ref error) = res {
            // Like spans, events can also use the field initialization shorthand.
            // In this instance, `yak` is the field being initalized.
            error!("failed to shave yak: {error}");
        } else {
            yaks_shaved += 1;
        }
        debug!("{yaks_shaved}");
    }

    yaks_shaved
}
