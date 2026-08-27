use std::time::SystemTime;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer};

#[derive(Deserialize, uniffi::Record)]
pub struct Closure {
    pub id: u32,
    #[serde(rename = "rsv")]
    pub can_book: bool,
    #[serde(rename = "paiement")]
    pub can_pay: bool,
    #[serde(rename = "du", deserialize_with = "rfc3339_to_system_time")]
    pub from:SystemTime,
    #[serde(rename = "au", deserialize_with = "rfc3339_to_system_time")]
    pub to: SystemTime
}

fn rfc3339_to_system_time<'de, D: Deserializer<'de>>(d: D) -> Result<SystemTime, D::Error> {
    let dt = DateTime::<Utc>::deserialize(d)?;
    Ok(dt.into())
}

#[derive(Deserialize, uniffi::Record)]
pub struct Configuration {
    #[serde(rename = "url")]
    pub website: String,
    pub email: String,
    #[serde(rename = "nbRepasMini")]
    pub minimum_meal_to_pay: u8,
    #[serde(rename = "msgAccueil")]
    pub motd: String,
    #[serde(rename = "cacherHistorique")]
    pub hide_history: bool,
    #[serde(rename = "fermetures")]
    pub closures: Vec<Closure>
}

#[derive(Deserialize, uniffi::Record)]
pub struct Establishment {
    #[serde(rename = "nom")]
    pub name: String,
    #[serde(rename = "adr1")]
    pub street: String,
    #[serde(rename = "cp")]
    pub postal_code: String,
    #[serde(rename = "ville")]
    pub city: String,
    #[serde(rename = "tel")]
    pub phone_number: String,
    #[serde(rename = "currencySymbol")]
    pub currency_symbol: String,
    pub configuration: Configuration,
}

impl Establishment {
    pub fn full_address(&self) -> String {
        format!("{} {} {}", self.street, self.city, self.postal_code)
    }
}
