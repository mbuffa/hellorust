use states::{Context, GameContext};
mod states;

fn main() {
  let mut game_context = states::GameContext::new();
  
  // This is only here to test state switching. It has no other purpose here,
  // and should be replaced by a conventional game loop.
  for i in 1..10 {
    game_context.update();
    if i == 3 {
      game_context.trigger(states::Event::Play);  
    }
  }
}


#[test]
fn it_works() {
  
}
