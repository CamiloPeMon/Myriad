use std::fmt::{write, Display, Formatter};

use super::bodypart::BodyPart;

pub struct Creature {
    pub name: String,
    pub health: i32,
    pub attack: i32,
    pub speed: i32,
    pub body_parts: Vec<BodyPart>,
}

impl Creature {
    pub fn new(
        name: String,
        health: i32,
        attack: i32,
        body_parts: Vec<BodyPart>,
        speed: i32,
    ) -> Creature {
        Creature {
            name,
            health,
            attack,
            body_parts,
            speed,
        }
    }
}

impl Creature {
    pub fn new_orc(name: String) -> Creature {
        Creature {
            name,
            health: 5,
            attack: 10,
            body_parts: vec![
                BodyPart::new("Head".to_string(), 6),
                BodyPart::new("Body".to_string(), 6),
            ],
            speed: 1,
        }
    }
}

impl Creature {
    pub fn show_body_parts(&self) -> Vec<String> {
        let mut result = vec![];
        for body_part in self.body_parts.clone() {
            result.push(body_part.to_string());
        }
        result
    }
}
impl Creature {
    pub fn hit(&mut self, body_part: &str) {
        for part in &mut self.body_parts {
            if part.name == body_part {
                let old_health = part.health;
                part.health = old_health.saturating_sub(1);
                println!("You hit the {}, life left: {}", part.name, part.health);
                break;
            }
        }
    }
}
