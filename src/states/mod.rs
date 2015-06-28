mod initializing;
mod playing;

pub enum Event {
  Play
}

pub trait Context {
  fn update(&mut self);
  fn trigger(&mut self, event: Event);
}

pub enum GameState {
  Initializing,
  Playing
}

pub struct GameContext {
  current: GameState
}

impl GameContext {
  pub fn new() -> GameContext {
    GameContext {
      current: GameState::Initializing
    }
  }

  pub fn do_stuff(&mut self) {
    println!("Doin' stuff!");
  }
}

impl Context for GameContext {
  fn update(&mut self) {
    println!("Updating");

    match self.current {
      GameState::Initializing => {
        initializing::update(self);
      },
      GameState::Playing => {
        playing::update(self);
      }
    }
  }

  fn trigger(&mut self, event: Event) {
    println!("Triggered");

    match event {
      Event::Play => {
        println!("Play!");
        self.current = GameState::Playing;
      }
    }
  }
}
