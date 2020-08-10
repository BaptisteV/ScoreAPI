CREATE TABLE player (
  "name" TEXT NOT NULL UNIQUE,
  player_id SERIAL NOT NULL,
  PRIMARY KEY(player_id)
);