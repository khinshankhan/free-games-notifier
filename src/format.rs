use crate::epic::Offer;

pub fn store_url(offer: &Offer) -> Option<String> {
    let slug = &offer.product_slug;

    match slug {
        Some(s) => Some(format!(
            "https://www.epicgames.com/store/en-US/p/{}",
            s
        )),
        None => return None,
    }
}
