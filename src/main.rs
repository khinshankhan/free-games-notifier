mod db;
mod discord;
mod epic;
mod epic_logic;

fn main() {
    dotenvy::dotenv().ok();

    match epic_logic::handle_epic() {
        Ok(()) => println!("Successfully fetched and displayed Epic Games offers."),
        Err(e) => eprintln!("HTTP error: {e}"),
    }
}
