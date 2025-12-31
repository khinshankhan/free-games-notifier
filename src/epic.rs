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
    pub price: Price,

    #[serde(rename = "productSlug")]
    pub product_slug: Option<String>,

    pub categories: Option<Vec<Category>>,

    #[serde(rename = "offerType")]
    pub offer_type: Option<String>,

    #[serde(default)]
    pub promotions: Option<Promotions>,
}

#[derive(Debug, Deserialize)]
pub struct Category {
    pub path: String,
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

#[derive(Debug, Deserialize)]
pub struct Promotions {
    #[serde(rename = "promotionalOffers", default)]
    pub promotional_offers: Vec<PromotionalOfferBlock>,
}

#[derive(Debug, Deserialize)]
pub struct PromotionalOfferBlock {
    #[serde(rename = "promotionalOffers", default)]
    pub promotional_offers: Vec<Promotion>,
}

#[derive(Debug, Deserialize)]
pub struct Promotion {
    #[serde(rename = "startDate")]
    pub start_date: String,
    #[serde(rename = "endDate")]
    pub end_date: String,

    #[serde(rename = "discountSetting")]
    pub discount_setting: DiscountSetting,
}

#[derive(Debug, Deserialize)]
pub struct DiscountSetting {
    #[serde(rename = "discountPercentage")]
    pub discount_percentage: Option<i64>,
}
