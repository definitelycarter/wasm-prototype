pub struct WindowBuilder {
  canvas_id: Option<&'static str>,
}

impl WindowBuilder {
  pub fn canvas_id(mut self, canvas_id: &'static str) -> Self {
    self.canvas_id = Some(canvas_id);
    self
  }

  pub fn build(self) -> Window {
    return Window {};
  }
}

pub struct Window {}

impl Window {
  pub fn builder() -> WindowBuilder {
    return WindowBuilder { canvas_id: None };
  }
}
