use specs::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::console;

#[derive(Debug)]
pub struct Position {
  pub x: f32,
  pub y: f32,
}

impl Component for Position {
  type Storage = VecStorage<Self>;
}

pub struct HelloWorld;

impl<'a> System<'a> for HelloWorld {
  type SystemData = ReadStorage<'a, Position>;

  fn run(&mut self, position: Self::SystemData) {
    for position in position.join() {
      // console::log_1(&JsValue::from_str(
      //   format!("Hello, {:?}", &position).as_str(),
      // ));
      // println!("Hello, {:?}", &position);
    }
  }
}
