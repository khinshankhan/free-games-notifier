fn get_epic_data() -> Result<String, reqwest::Error> {
    let epic_url =
        "https://store-site-backend-static.ak.epicgames.com/freeGamesPromotions?locale=en-US";

    Ok(reqwest::blocking::get(epic_url)?.text()?)
}

fn main() {
    match get_epic_data() {
        Ok(data) => println!("Epic Games Data: {}", data),
        Err(e) => eprintln!("Error fetching data: {}", e),
    }
}
