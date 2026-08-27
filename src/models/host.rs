use std::fmt;

use serde::Deserialize;

use crate::models::establishment::Establishment;

#[derive(Deserialize, uniffi::Enum)]
pub enum Mode {
    #[serde(rename = "Argent")]
    Cash,
    #[serde(rename = "Forfait")]
    Plan,
}

#[derive(Deserialize, uniffi::Enum)]
#[serde(try_from = "u8")]
pub enum AccountType {
    Student,
    Commensal,
    Teacher,
}

impl TryFrom<u8> for AccountType {
    type Error = UnknownAccountType;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Student),
            1 => Ok(Self::Commensal),
            2 => Ok(Self::Teacher),
            unknown => Err(UnknownAccountType(unknown)),
        }
    }
}

pub struct UnknownAccountType(u8);

impl fmt::Display for UnknownAccountType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unknown account type: {}", self.0)
    }
}

#[derive(Deserialize, uniffi::Record)]
pub struct Host {
    pub id: u32,
    #[serde(rename = "idOrig")]
    pub local_id: u32,
    #[serde(rename = "desactive")]
    pub disabled: bool,
    #[serde(rename = "etab")]
    pub establishment: Establishment,
    #[serde(rename = "prenom")]
    pub first_name: String,
    #[serde(rename = "nom")]
    pub last_name: String,
    pub mode: Mode,
    #[serde(rename = "qualite")]
    pub quality: String,
    pub division: String,
    #[serde(rename = "prixDej")]
    pub meal_price: u16,
    #[serde(rename = "type")]
    pub account_type: AccountType,
    #[serde(rename = "carteCodee")]
    pub card_number: Option<String>,
    #[serde(flatten)]
    pub permissions: Permissions,
}

#[derive(Deserialize, uniffi::Record)]
pub struct Permissions {
    #[serde(rename = "droitPaiement")]
    pub payment: bool,
    #[serde(rename = "droitReservation")]
    pub reservation: bool,
    #[serde(rename = "droitCafeteria")]
    pub cafeteria: bool,
    #[serde(rename = "autoriseReservSoldeIns")]
    pub book_with_negative_balance: bool,
    #[serde(rename = "nbMulti")]
    pub max_passages: u8,
}
