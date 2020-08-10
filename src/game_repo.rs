use models::{Game, NewGame};

use diesel::prelude::*;
use diesel::result::Error;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

use schema::game::dsl::*;




fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub trait IGameRepository {
    fn new() -> Self;
    fn insert(&self, new_game: NewGame) -> Result<Game, Error>;
    fn get_all_games(&self) -> Vec<Game>;
    fn get_by_id(&self, id: i32) -> std::result::Result<Game, Error>;
    fn get_by_name(&self, game_name: String) -> std::result::Result<Game, Error>;
}

pub struct GameRepository {}

impl IGameRepository for GameRepository {
    fn new () -> GameRepository {
        GameRepository{}
    }

    fn insert(&self, new_game: NewGame) -> Result<Game, Error> {
        use schema::game;
        let connection = establish_connection();
        diesel::insert_into(game::table).values(new_game).get_result(&connection)
    }

    fn get_all_games(&self) -> Vec<Game>{
        use schema::game;
        let connection = establish_connection();
        game::table.load::<Game>(&connection)
            .expect("Error loading games")
    }
    
    fn get_by_id(&self, id: i32) -> std::result::Result<Game, Error> {
        let connection = establish_connection();
        game.filter(game_id.eq(&id)).first::<Game>(&connection)
    }

    fn get_by_name(&self, game_name: String) -> std::result::Result<Game, Error> {
        let connection = establish_connection();
        game.filter(name.eq(&game_name)).first::<Game>(&connection)
    }
}