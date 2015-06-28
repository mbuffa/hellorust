mod initializing;
mod playing;

pub enum Event {
  Play,
  Quit
}

pub enum GameState {
  Initializing,
  Playing
}

pub struct GameContext {
  current: GameState,
  update_fn: Option<fn(&mut GameContext) -> ()>,
  on_enter_fn: Option<fn(&mut GameContext) -> ()>,
  on_leave_fn: Option<fn(&mut GameContext) -> ()>
}

impl GameContext {
  pub fn new() -> GameContext {
    let mut ctx = GameContext {
      current: GameState::Initializing,
      update_fn: Some(initializing::update),
      on_enter_fn: Some(initializing::on_enter),
      on_leave_fn: Some(initializing::on_leave)
    };
    (ctx.on_enter_fn.unwrap())(&mut ctx);
    ctx
  }

  pub fn do_stuff(&mut self) {
    println!("Doin' stuff!");
  }
}

pub trait Context {
  fn update(&mut self) -> bool;
  fn trigger(&mut self, event: Event);
}

impl Context for GameContext {
  fn update(&mut self) -> bool {
    match self.update_fn {
      None => { false },
      _ => {
        (self.update_fn.unwrap())(self);
        true
      }
    }
  }

  fn trigger(&mut self, event: Event) {
    match event {
      Event::Play => {
        match self.on_leave_fn {
          None => {},
          _ => {
            (self.on_leave_fn.unwrap())(self);
          }
        }

        self.current = GameState::Playing;
        self.update_fn = Some(playing::update);
        self.on_enter_fn = Some(playing::on_enter);
        self.on_leave_fn = Some(playing::on_leave);

        (self.on_enter_fn.unwrap())(self);
      },
      Event::Quit => {
        self.update_fn = None;
      }
    }
  }
}
