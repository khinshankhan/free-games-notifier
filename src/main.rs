fn get_epic_data() -> Result<String, reqwest::Error> {
    let epic_url =
        "https://store-site-backend-static.ak.epicgames.com/freeGamesPromotions?locale=en-US";

    match reqwest::blocking::get(epic_url) {
        Ok(response) => {
            match response.text() {
                Ok(text) => Ok(text),
                Err(e) => Err(e),
            }
        }
        Err(e) => Err(e),
    }
}

fn main() {
    match get_epic_data() {
        Ok(data) => println!("Epic Games Data: {}", data),
        Err(e) => eprintln!("Error fetching data: {}", e),
    }
}
