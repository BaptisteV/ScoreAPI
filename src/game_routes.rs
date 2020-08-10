use models::{Game};


#[derive(Serialize, Deserialize)]
struct GamePost {
    name: String,
}
#[post("/game", format="json", data="<game_post>")]
pub fn post_game(game_post: Json<GamePost>) -> Result<Json<Game>, Status> {
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

fn create_new_game_using_repo(name: String) -> Result<Game, Error> {
    use game_repo::IGameRepository;
    use game_repo::GameRepository;
    let game_repo: GameRepository = GameRepository::new();
    game_repo.insert(NewGame::new(name))
}