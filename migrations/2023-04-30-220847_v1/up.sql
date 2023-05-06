CREATE TABLE players (
  id SERIAL PRIMARY KEY,
  player VARCHAR NOT NULL,
  dmxp INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE characters (
  id SERIAL PRIMARY KEY,
  player_id INT NOT NULL,
  name VARCHAR NOT NULL,
  sheet VARCHAR NOT NULL,
  downtime INTEGER NOT NULL DEFAULT 0,
  house_channel VARCHAR,
  visible BOOLEAN NOT NULL DEFAULT true,
  CONSTRAINT fk_player FOREIGN KEY(player_id) REFERENCES players(id)
);

CREATE TABLE progress (
  id SERIAL PRIMARY KEY,
  character INT NOT NULL,
  curr_value INT NOT NULL,
  target_value INT NOT NULL,
  name VARCHAR NOT NULL,
  CONSTRAINT fk_character FOREIGN KEY(character) REFERENCES characters(id)
);

CREATE TABLE shop_items (
  id SERIAL PRIMARY KEY,
  rarity VARCHAR NOT NULL,
  name VARCHAR NOT NULL,
  price INT NOT NULL,
  in_bazaar BOOLEAN NOT NULL DEFAULT false,
  in_black_market BOOLEAN NOT NULL DEFAULT false,
  in_trade_market BOOLEAN NOT NULL DEFAULT false
);

CREATE TABLE current_shop_items (
  id SERIAL PRIMARY KEY,
  item_id INT NOT NULL,
  market VARCHAR NOT NULL,
  store VARCHAR NOT NULL,
  CONSTRAINT fk_items FOREIGN KEY(item_id) REFERENCES shop_items(id)
);

