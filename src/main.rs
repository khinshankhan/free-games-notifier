mod db;
mod discord;
mod epic;
mod epic_logic;
mod time;

fn main() {
    dotenvy::dotenv().ok();

    let ts = time::SystemTimeSource;

    match epic_logic::handle_epic(&ts) {
        Ok(()) => println!("Successfully fetched and displayed Epic Games offers."),
        Err(e) => eprintln!("HTTP error: {e}"),
    }
}
