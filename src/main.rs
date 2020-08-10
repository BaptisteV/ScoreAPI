#![feature(decl_macro)]
#![feature(proc_macro_hygiene)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate serde;
#[macro_use] extern crate diesel;


extern crate rocket_contrib;
extern crate serde_json;
extern crate dotenv;
use diesel::result::Error;

use rocket::http::{Status};
use rocket_contrib::json::{Json};


pub mod game;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

pub mod player_repo;
pub mod score_repo;
pub mod models;
pub mod schema;

use self::models::*;
/*
#[derive(Serialize, Deserialize)]
struct GamePost {
    name: String,
}
#[post("/game", format="json", data="<game_post>")]
fn post_game(game_post: Json<GamePost>) -> Result<Json<Game>, Status> {
    let new_game = create_new_game_using_repo(game_post.into_inner().name);
    match new_game {
        Ok(g) => Ok(Json(g)),
        Err(_) => Err(Status::NoContent)
    }
}
#[get("/games", format="json")]
fn get_games() -> Json<Vec<Game>> {
    use game_repo::IGameRepository;
    use game_repo::GameRepository;
    let game_repo: GameRepository = GameRepository::new();
    Json(game_repo.get_all_games())
}
*/

fn main() {
    rocket::ignite().mount("/", routes![
        index, 
        post_game, 
        get_games
    ]).launch();
}
/*
fn create_new_game_using_repo(name: String) -> Result<Game, Error> {
    use game_repo::IGameRepository;
    use game_repo::GameRepository;
    let game_repo: GameRepository = GameRepository::new();
    game_repo.insert(NewGame::new(name))
}
*/
fn create_player(name: String) -> Player {
    use player_repo::IPlayerRepository;
    use player_repo::PlayerRepository;
    let player_repo: PlayerRepository = PlayerRepository::new();
    match player_repo.insert(NewPlayer::new(name)){
        Ok(p) => p,
        Err(e) => panic!(e),
    }
}

fn add_score_for_game(value: i64, game: Game, player: Player) {
    use score_repo::IScoreRepository;
    use score_repo::ScoreRepository;
    let score_repo: ScoreRepository = ScoreRepository::new();
    //ScoreRepository::new().insert(NewScore::new(value, game.game_id, score.score_id))
    match score_repo.insert(NewScore::new(value, game.game_id, player.player_id)){
        true => println!("Successfully inserted a score of {} for game {}", value, game.name),
        false => println!("Successfully inserted a score of {} for game {}", value, game.name),
    }
}
