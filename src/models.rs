// use diesel::prelude::*;
// use crate::schema::posts;
//
// #[derive(Queryable, Insertable)]
// pub struct Player {
//     pub id: i32,
//     pub player: String,
//     pub dmxp: i32,
// }
//
// #[derive(Queryable, Insertable)]
// pub struct Character {
//     pub id: i32,
//     pub player_id: i32,
//     pub name: String,
//     pub sheet: String,
//     pub downtime: i32,
//     pub house_channel: Option<String>,
//     pub visible: bool,
// }
//
// #[derive(Queryable, Insertable)]
// pub struct CommandMode {
//     pub channel: String,
// }
//
// #[derive(Queryable, Insertable)]
// pub struct CurrentShopItem {
//     pub id: i32,
//     pub item_id: i32,
//     pub market: String,
//     pub store: String,
// }
//
// #[derive(Queryable, Insertable)]
// pub struct Progress {
//     pub id: i32,
//     pub character: i32,
//     pub curr_value: i32,
//     pub target_value: i32,
// }
//
// #[derive(Queryable, Insertable)]
// pub struct ShopItem {
//     pub id: i32,
//     pub rarity: String,
//     pub name: String,
//     pub price: i32,
//     pub in_bazaar: bool,
//     pub in_black_market: bool,
//     pub in_trade_market: bool,
// }
