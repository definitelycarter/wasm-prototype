use specs::{RunNow, World};

pub struct GameBuilder<'a> {
  systems: Vec<Box<dyn RunNow<'a>>>,
  world: Option<World>,
}

impl<'a> GameBuilder<'a> {
  pub fn system(mut self, system: Box<dyn RunNow<'a>>) -> GameBuilder {
    self.systems.push(system);
    self
  }

  pub fn world(mut self, world: World) -> GameBuilder<'a> {
    self.world = Some(world);
    self
  }

  pub fn build(self) -> Game<'a> {
    return Game {
      systems: self.systems,
      world: self.world.unwrap(),
    };
  }
}

pub struct Game<'a> {
  systems: Vec<Box<dyn RunNow<'a>>>,
  world: World,
}

impl<'a> Game<'a> {
  pub fn builder() -> GameBuilder<'a> {
    return GameBuilder {
      systems: Vec::new(),
      world: None,
    };
  }

  pub fn setup(&mut self) {
    for sys in self.systems.iter_mut() {
      sys.setup(&mut self.world)
    }
  }

  // Try to understand these lifetime issues
  // https://bfnightly.bracketproductions.com/rustbook/chapter_73.html
  pub fn run_now(&'a mut self) {
    for sys in self.systems.iter_mut() {
      sys.run_now(&self.world);
    }
  }
}
