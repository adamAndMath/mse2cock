use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(rename = "cockatrice_carddatabase")]
pub struct CustomSet {
    #[serde(rename = "@version")]
    pub version: String,
    pub sets: Sets,
    pub cards: Cards,
}

#[derive(Deserialize)]
pub struct CardNote {
    #[serde(default)]
    pub related: Vec<CardRelation>,
}

#[derive(Deserialize, Serialize)]
pub struct Sets {
    #[serde(default)]
    pub set: Vec<Set>,
}

#[derive(Deserialize, Serialize)]
pub struct Set {
    pub name: String,
    pub longname: String,
    pub settype: String,
    pub releasedate: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct Cards {
    #[serde(default)]
    pub card: Vec<Card>,
}

#[derive(Deserialize, Serialize)]
pub struct Card {
    pub name: String,
    pub text: String,
    pub prop: CardProp,
    pub tablerow: Option<String>,
    pub token: Option<String>,
    #[serde(default)]
    pub set: Vec<CardSet>,
    #[serde(default)]
    pub related: Vec<CardRelation>,
}

#[derive(Deserialize, Serialize)]
pub struct CardProp {
    pub manacost: Option<String>,
    pub side: Option<String>,
    pub cmc: Option<String>,
    pub layout: Option<String>,
    #[serde(rename="type")]
    pub ttype: String,
    #[serde(rename="maintype")]
    pub main_type: String,
    #[serde(rename="coloridentity")]
    pub color_identity: Option<String>,
    pub colors: Option<String>,
    pub pt: Option<String>,
    pub loyalty: Option<String>,
    #[serde(rename="format-commander")]
    pub format_commander: Option<String>,
    #[serde(rename="format-vintage")]
    pub format_vintage: Option<String>,
    #[serde(rename="format-legacy")]
    pub format_legacy: Option<String>,
    #[serde(rename="format-duel")]
    pub format_duel: Option<String>,
    #[serde(rename="format-oathbreaker")]
    pub format_oathbreaker: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct CardSet {
    #[serde(rename="@uuid")]
    pub uuid: Option<String>,
    #[serde(rename="@num")]
    pub num: Option<String>,
    #[serde(rename="@rarity")]
    pub rarity: Option<String>,
    #[serde(rename="@muid")]
    pub muid: Option<String>,
    #[serde(rename="@picurl")]
    pub picurl: Option<String>,
    #[serde(rename="$text")]
    pub name: String,
}

#[derive(Deserialize, Serialize)]
pub struct CardRelation {
    #[serde(rename="@count")]
    pub count: Option<String>,
    #[serde(rename="@attach")]
    pub attach: Option<String>,
    #[serde(rename="@exclude")]
    pub exclude: Option<String>,
    #[serde(rename="@persistent")]
    pub persistent: Option<String>,
    #[serde(rename="$text")]
    pub name: String,
}
