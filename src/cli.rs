use crate::config::{Action, Config};

fn seed() {
    println!("seeding...");
}

fn drop() {
    println!("dropping...");
}

pub fn run(config: &Config) {
    if let Some(action) = &config.action {
        match action {
            Action::Seed => seed(),
            Action::Drop => drop(),
            Action::All => {
                seed();
                drop();
            },
        }
    }
}
