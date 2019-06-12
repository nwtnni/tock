mod data;

fn main() {
    let sleep = std::time::Duration::from_secs(1);
    loop {
        std::thread::sleep(sleep);
        println!("{:?}", chrono::Local::now());
    }
}
