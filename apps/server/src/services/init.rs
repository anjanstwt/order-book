use std::path::PathBuf;

use engine::Engine;

pub struct Init {
    pub engine: Engine,
}

impl Init {
    pub fn core() -> Self {
        Self {
            engine: Engine::new(None),
        }
    }

    pub fn env() {
        let root_env = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../.env");
        dotenvy::from_path(root_env).expect(".env not setted up");
    }
}
