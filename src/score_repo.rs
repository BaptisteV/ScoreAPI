use models::{Score, NewScore};
use game_repo::*;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub trait IScoreRepository {
    fn new() -> Self;
    fn insert(&self, new_score: NewScore) -> bool;
    fn get_by_game_name(&self, game_name: String, player_name: String) -> Vec<Score>;
}

pub struct ScoreRepository {}

impl IScoreRepository for ScoreRepository {

    fn new () -> ScoreRepository {
        ScoreRepository{}
    }

    fn insert(&self, new_score: NewScore) -> bool {
        use schema::score;
        let connection = establish_connection();
        let insert_result = diesel::insert_into(score::table).values(new_score).execute(&connection);
        match insert_result {
                Ok(1) => true,
                _ => false,
        }
    }
    
    fn get_by_game_name(&self, game_name: String, player_name: String) -> Vec<Score> {
        use game_repo::IGameRepository;
        use game_repo::GameRepository;
        use schema::score;
        let game_repo: GameRepository = GameRepository::new();
        let connection = establish_connection();
        let target_game = game_repo.get_by_name(game_name).expect("Game not found when trying to get its score");
        score::table.filter(score::game_id.eq(target_game.game_id)).load::<Score>(&connection).expect("Error getting the scores")
        //score.filter(game_id.eq(&target_game.game_id)).load::<Score>(&connection)
    }
}