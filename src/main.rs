use std::error;

mod time;
mod view;

fn main() -> Result<(), Box<dyn error::Error>> {
    let sleep = std::time::Duration::from_secs(1);
    loop {
        std::thread::sleep(sleep);
        println!("{:?}", chrono::Local::now());
    }

    Ok(())
}
