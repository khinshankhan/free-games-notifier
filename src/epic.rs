use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct EpicResponse {
    pub data: EpicData,
}

#[derive(Debug, Deserialize)]
pub struct EpicData {
    #[serde(rename = "Catalog")]
    pub catalog: Catalog,
}

#[derive(Debug, Deserialize)]
pub struct Catalog {
    #[serde(rename = "searchStore")]
    pub search_store: SearchStore,
}

#[derive(Debug, Deserialize)]
pub struct SearchStore {
    pub elements: Vec<Offer>,
}

#[derive(Debug, Deserialize)]
pub struct Offer {
    pub title: String,
    pub id: String,
    pub namespace: String,
    pub price: Price,
}

#[derive(Debug, Deserialize)]
pub struct Price {
    #[serde(rename = "totalPrice")]
    pub total_price: TotalPrice,
}

#[derive(Debug, Deserialize)]
pub struct TotalPrice {
    #[serde(rename = "discountPrice")]
    pub discount_price: i64,
}
