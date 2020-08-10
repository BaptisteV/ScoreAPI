table! {
    game (game_id) {
        name -> Text,
        game_id -> Int4,
    }
}

table! {
    player (player_id) {
        name -> Text,
        player_id -> Int4,
    }
}

table! {
    score (score_id) {
        relative_value -> Int8,
        game_id -> Int4,
        player_id -> Int4,
        score_id -> Int4,
    }
}

joinable!(score -> game (game_id));
joinable!(score -> player (player_id));

allow_tables_to_appear_in_same_query!(
    game,
    player,
    score,
);
