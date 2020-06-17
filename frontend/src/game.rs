use specs::{RunNow, World};

pub struct GameBuilder<'a> {
  systems: Vec<Box<dyn RunNow<'a>>>,
}

impl<'a> GameBuilder<'a> {
  pub fn system(mut self, system: Box<dyn RunNow<'a>>) -> GameBuilder {
    self.systems.push(system);
    self
  }

  pub fn build(self) -> Game<'a> {
    return Game {
      systems: self.systems,
    };
  }
}

pub struct Game<'a> {
  systems: Vec<Box<dyn RunNow<'a>>>,
}

impl<'a> Game<'a> {
  pub fn builder() -> GameBuilder<'a> {
    return GameBuilder {
      systems: Vec::new(),
    };
  }

  pub fn setup(&mut self, ecs: &mut World) {
    for sys in self.systems.iter_mut() {
      sys.setup(ecs)
    }
  }

  // Try to understand these lifetime issues
  // https://bfnightly.bracketproductions.com/rustbook/chapter_73.html
  pub fn run_now(&mut self, ecs: *mut World) {
    for sys in self.systems.iter_mut() {
      unsafe {
        let ecs = &*ecs;
        sys.run_now(ecs);
      }
    }
  }
}
