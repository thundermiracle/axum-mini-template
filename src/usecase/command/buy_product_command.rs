use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BuyProductCommand {
    pub quantity: u32,
}
