use game::managers::master;
pub mod game;

use app::context::*;
pub mod app;

fn main() {
  let mut ctx = app::context::AppContext::new();

  loop {
    if !ctx.update() {
      break;
    }
  }
}


#[test]
fn it_works() {

}
