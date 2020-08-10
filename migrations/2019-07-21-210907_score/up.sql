CREATE TABLE score (
    relative_value BIGINT NOT NULL,
    game_id INTEGER NOT NULL REFERENCES game(game_id),
    player_id INTEGER NOT NULL REFERENCES player(player_id),
    score_id SERIAL NOT NULL,
    PRIMARY KEY(score_id)
);