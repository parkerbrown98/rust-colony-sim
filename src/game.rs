use crossterm::event::{self, Event, KeyCode};
use serde::Deserialize;
use std::{fs, io, thread, time};

use crate::display::GameDisplay;

pub struct GameState {
    entities: Vec<Box<dyn Entity>>,
    actions: Vec<Box<dyn TimedAction>>,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            entities: Vec::new(),
            actions: Vec::new(),
        }
    }

    pub fn add_entity(&mut self, entity: Box<dyn Entity>) {
        self.entities.push(entity);
    }

    pub fn add_action(&mut self, action: Box<dyn TimedAction>) {
        // ...
    }

    pub fn start(&mut self) {
        for entity in &mut self.entities {
            entity.start();
        }
    }

    pub fn update(&mut self, dt: f32) {
        for entity in &mut self.entities {
            entity.update(dt);
        }
    }

    pub fn exit(&self) {
        // ...
    }
}

#[derive(Deserialize, Debug)]
pub struct Config {
    tick_rate: f32,
    max_entities: u32,
    max_actions: u32,
}

impl Config {
    pub fn load(path: &str) -> Result<Config, io::Error> {
        let s = fs::read_to_string(path)?;
        toml::from_str(&s).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }
}

pub struct Game {
    pub state: GameState,
    pub display: GameDisplay,
    pub config: Config,
}

impl Game {
    pub fn new(config_path: &str) -> Result<Game, io::Error> {
        Ok(Game {
            state: GameState::new(),
            display: GameDisplay::new(),
            config: Config::load(config_path)?,
        })
    }

    pub fn load(&mut self, path: &str) {
        // ...
    }

    pub fn save(&self, path: &str) {
        // ...
    }

    pub fn run(&mut self) {
        self.start().unwrap();

        // Use tick rate from config (ticks per second)
        let tick_rate = time::Duration::from_secs_f32(self.config.tick_rate);
        let mut last_tick = time::Instant::now();

        loop {
            let now = time::Instant::now();
            let dt = now.duration_since(last_tick).as_secs_f32();
            last_tick = now;

            self.update(dt).unwrap();

            // Handle input
            let input_event = event::read().expect("Error reading input");
            match input_event {
                Event::Key(event) => {
                    if event.code == KeyCode::Char('q') {
                        break;
                    }
                }
                _ => {}
            }

            // Sleep until next tick
            let elapsed = now.elapsed();
            if elapsed < tick_rate {
                thread::sleep(tick_rate - elapsed);
            }
        }

        self.exit();
    }

    fn start(&mut self) -> io::Result<()> {
        let ship = Ship::new(Coords::new(0.0, 0.0));
        self.state.add_entity(Box::new(ship));
        self.state.start();
        self.display.start()

    }

    fn update(&mut self, dt: f32) -> io::Result<()> {
        self.state.update(dt);
        self.display.update()?;
        Ok(())
    }

    fn exit(&mut self) {
        self.display.exit();
        self.state.exit();
    }
}

#[derive(Clone)]
pub struct Coords {
    x: f32,
    y: f32,
}

impl ToString for Coords {
    fn to_string(&self) -> String {
        format!("({}, {})", self.x, self.y)
    }
}

impl Coords {
    pub fn new(x: f32, y: f32) -> Self {
        Coords { x, y }
    }
}

pub trait Entity {
    fn start(&mut self);
    fn update(&mut self, dt: f32);
    fn location(&self) -> Coords;
}

pub struct Ship {
    location: Coords,
    oxygen: f32,
    fuel: f32,
    food: f32,
}

impl Entity for Ship {
    fn start(&mut self) {}

    fn update(&mut self, dt: f32) {
        self.oxygen -= 0.1 * dt;
        self.fuel -= 0.2 * dt;
        self.food -= 0.3 * dt;
    }

    fn location(&self) -> Coords {
        // We can't give you the location, but we can give you a clone!
        self.location.clone()
    }
}

impl Ship {
    pub fn new(location: Coords) -> Self {
        Ship {
            location,
            oxygen: 100.0,
            fuel: 100.0,
            food: 100.0,
        }
    }
}

pub trait TimedAction {
    // ...
}
