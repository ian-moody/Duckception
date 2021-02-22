use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

pub enum Role {
  /* Werewolf,
Villager,
Seer,
Witch,
Hunter,
Knight,
Doctor */}

pub struct Player {
  pub id: String,
  pub name: String,
  pub role: Option<Role>,
}

impl Player {
  pub fn new(id: String) -> Self {
    Player {
      id: id,
      name: String::from("Anon"),
      role: None,
    }
  }
}

pub struct GameRoom {
  pub players: Vec<Player>,
}

pub type SharedGames = Arc<Mutex<HashMap<String, GameRoom>>>;

pub fn make_user_id() -> String { Uuid::new_v4().to_string() }

pub fn join_game(games: SharedGames, id: String, room_name: &str) {
  info!("{} is joining Room: {}", id, room_name);
  let mut games = games.lock().unwrap();

  let current_games = games.keys();
  for (i, key) in current_games.enumerate() {
    info!("Current Games {} :: {}", i, key);
  }

  let room_key = room_name.to_string();
  let game = match games.get_mut(&room_key) {
    Some(val) => {
      match val.players.iter().find(|p| p.id == id) {
        Some(you) => {
          info!("Found you! {}: {}", you.id, you.name);
        }
        None => {
          info!("Didn't find you!");
          val.players.push(Player::new(id));
        }
      };
      val
    }
    None => {
      info!("Adding new game {}", room_key);
      let mut vec = Vec::with_capacity(20);
      vec.push(Player::new(id));
      games.insert(room_key.clone(), GameRoom { players: vec });
      games.get_mut(&room_key).unwrap()
    }
  };

  info!("Player count: {}", game.players.len());
  for p in &game.players {
    info!("  {}: {}", p.id, p.name);
  }
}

pub fn new_game() -> SharedGames {
  Arc::new(Mutex::new(HashMap::new()))
}
