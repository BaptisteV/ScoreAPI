CREATE TABLE game (
  "name" TEXT NOT NULL UNIQUE,
  game_id SERIAL NOT NULL,
  PRIMARY KEY(game_id)
);