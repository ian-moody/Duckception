pub enum Role {
  // Werewolf,
// Villager,
// Seer,
// Witch,
// Hunter,
// Knight,
// Doctor,
}

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
