// @generated automatically by Diesel CLI.

diesel::table! {
    characters (id) {
        id -> Int4,
        player_id -> Int4,
        name -> Varchar,
        sheet -> Varchar,
        downtime -> Int4,
        house_channel -> Nullable<Varchar>,
        visible -> Bool,
    }
}

diesel::table! {
    command_mode (channel) {
        channel -> Varchar,
    }
}

diesel::table! {
    current_shop_items (id) {
        id -> Int4,
        item_id -> Int4,
        market -> Varchar,
        store -> Varchar,
    }
}

diesel::table! {
    players (id) {
        id -> Int4,
        player -> Varchar,
        dmxp -> Int4,
    }
}

diesel::table! {
    progress (id) {
        id -> Int4,
        character -> Int4,
        curr_value -> Int4,
        target_value -> Int4,
        name -> Varchar,
    }
}

diesel::table! {
    shop_items (id) {
        id -> Int4,
        rarity -> Varchar,
        name -> Varchar,
        price -> Int4,
        in_bazaar -> Bool,
        in_black_market -> Bool,
        in_trade_market -> Bool,
    }
}

diesel::joinable!(characters -> players (player_id));
diesel::joinable!(current_shop_items -> shop_items (item_id));
diesel::joinable!(progress -> characters (character));

diesel::allow_tables_to_appear_in_same_query!(
    characters,
    command_mode,
    current_shop_items,
    players,
    progress,
    shop_items,
);
