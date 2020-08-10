use schema::*;

#[derive(Queryable, Debug, Serialize)]
pub struct Game {
    pub name: String,
    pub game_id: i32,
}

#[derive(Insertable)]
#[table_name="game"]
pub struct NewGame {
    pub name: String,
}

impl From<Game> for NewGame {
    fn from(item: Game) -> Self {
        NewGame { name: item.name }
    }
}

impl NewGame {
    pub fn new(name: String) -> Self {
        NewGame {name: name}
    }
}

#[derive(Queryable, Debug)]
pub struct Player {
    pub name: String,
    pub player_id: i32,
}

#[derive(Insertable)]
#[table_name="player"]
pub struct NewPlayer {
    pub name: String,
}

impl From<Player> for NewPlayer {
    fn from(item: Player) -> Self {
        NewPlayer { name: item.name }
    }
}

impl NewPlayer {
    pub fn new(name: String) -> Self {
        NewPlayer {name: name}
    }
}

#[derive(Queryable, Debug)]
pub struct Score {
    pub relative_value: i64,
    pub game_id: i32,
    pub player_id: i32,
    pub score_id: i32,
}

#[derive(Insertable)]
#[table_name="score"]
pub struct NewScore {
    pub relative_value: i64,
    pub game_id: i32,
    pub player_id: i32,
}

impl From<Score> for NewScore {
    fn from(item: Score) -> Self {
        NewScore { relative_value: item.relative_value, game_id: item.game_id, player_id: item.player_id }
    }
}

impl NewScore {
    pub fn new(relative_value: i64, game_id: i32, player_id: i32) -> Self {
        NewScore { relative_value: relative_value, game_id: game_id, player_id: player_id }
    }
}