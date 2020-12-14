use std::collections::HashMap;
use std::sync::{Arc, Mutex};

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

pub fn join_game(games: SharedGames, id: String, room_name: &str) {
  println!("{} is joining Room: {}", id, room_name);
  let mut games = games.lock().unwrap();

  let current_games = games.keys();
  for (i, key) in current_games.enumerate() {
    println!("Current Games {} :: {}", i, key);
  }

  let room_key = room_name.to_string();
  let game = match games.get_mut(&room_key) {
    Some(val) => {
      match val.players.iter().find(|p| p.id == id) {
        Some(you) => {
          println!("Found you! {}: {}", you.id, you.name);
        }
        None => {
          println!("Didn't find you!");
          val.players.push(Player::new(id));
        }
      };
      val
    }
    None => {
      println!("Adding new game {}", room_key);
      let mut vec = Vec::with_capacity(20);
      vec.push(Player::new(id));
      games.insert(room_key.clone(), GameRoom { players: vec });
      games.get_mut(&room_key).unwrap()
    }
  };

  println!("Player count: {}", game.players.len());
  for p in &game.players {
    println!("  {}: {}", p.id, p.name);
  }
  println!("\n");
}

pub fn new_game() -> SharedGames {
  Arc::new(Mutex::new(HashMap::new()))
}
