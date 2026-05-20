use engine::Engine;

pub struct Init {
    pub engine: Engine,
}

impl Init {
    pub fn new() -> Self {
        Self {
            engine: Engine::new(None),
        }
    }
}
