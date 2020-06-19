mod game;
mod systems;
mod window;

use game::Game;
use specs::{prelude::*, World};
use std::cell::RefCell;
use std::rc::Rc;
use systems::{HelloWorld, Position};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
  // This provides better error messages in debug mode.
  // It's disabled in release mode so it doesn't bloat up the file size.
  #[cfg(debug_assertions)]
  console_error_panic_hook::set_once();

  let mut world = World::new();
  world.register::<Position>();

  world
    .create_entity()
    .with(Position { x: 4.0, y: 7.0 })
    .build();

  world
    .create_entity()
    .with(Position { x: 6.0, y: 7.0 })
    .build();

  world
    .create_entity()
    .with(Position { x: 5.0, y: 7.0 })
    .build();

  let f = Rc::new(RefCell::new(None));
  let g = f.clone();

  let mut game = Game::builder()
    .system(Box::new(HelloWorld))
    .build();
    
  game.setup();
  // uncomment this line to see `game` does not live long enough
  // game.run_now();

  *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
    // uncomment this line to see another lifetime error.
    // game.run_now();
    request_animation_frame(f.borrow().as_ref().unwrap());
  }) as Box<dyn FnMut()>));

  request_animation_frame(g.borrow().as_ref().unwrap());
  Ok(())
}

fn window() -> web_sys::Window {
  web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) -> i32 {
  window()
    .request_animation_frame(f.as_ref().unchecked_ref())
    .expect("should register `requestAnimationFrame` OK")
}
