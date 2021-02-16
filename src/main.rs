use glsp::prelude::*;

fn main() {
    let runtime = Runtime::new();
    runtime.run(|| {
        glsp::load("game.glsp")?;
        Ok(())
    });
}