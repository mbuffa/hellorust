use states::{Context, GameContext};
mod states;

fn main() {
  let mut ctx = states::GameContext::new();

  loop {
    if !ctx.update() {
      break;
    }
  }
}


#[test]
fn it_works() {

}
