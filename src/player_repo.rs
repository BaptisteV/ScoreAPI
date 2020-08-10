use models::{Player, NewPlayer};

use diesel::prelude::*;
use diesel::result::Error;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

use schema::player::dsl::*;

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub trait IPlayerRepository {
    fn new() -> Self;
    fn insert(&self, new_game: NewPlayer) -> Result<Player, Error>;
    fn get_all_players(&self) -> Vec<Player>;
    fn get_by_id(&self, player_id: i32) -> std::result::Result<Player, Error>;
}

pub struct PlayerRepository {}

impl IPlayerRepository for PlayerRepository {

    fn new () -> PlayerRepository {
        PlayerRepository{}
    }

    fn insert(&self, new_player: NewPlayer) -> Result<Player, Error> {
        use schema::player;
        let connection = establish_connection();
        let insert_result = diesel::insert_into(player::table).values(new_player).get_result(&connection);
        insert_result
    }

    fn get_all_players(&self) -> Vec<Player>{
        use schema::player;
        let connection = establish_connection();
        player::table.load::<Player>(&connection)
            .expect("Error loading players")
    }
    
    fn get_by_id(&self, id: i32) -> std::result::Result<Player, Error> {
        let connection = establish_connection();
        player.filter(player_id.eq(&id)).first::<Player>(&connection)
    }
}