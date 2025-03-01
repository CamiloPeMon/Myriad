use std::fmt::{write, Display, Formatter};

struct Creature {
    name: String,
    health: i32,
    attack: i32,   
    speed: i32,
    body_parts: Vec<BodyPart>,
}

impl Creature {
    fn new(name: String, health: i32, attack: i32, body_parts: Vec<BodyPart>, speed: i32) -> Creature {
        Creature {
            name,
            health,
            attack,
            body_parts,
            speed
        }
    }
}

impl Creature {
    fn new_orc(name: String) -> Creature {
        Creature {
            name,
            health: 5,
            attack: 10,
            body_parts: vec![BodyPart::new("Head".to_string(), 6), BodyPart::new("Body".to_string(), 6)],
            speed: 1
        }
    }
}

#[derive(Clone)]
struct BodyPart {
    name: String,
    health: u32,
}

impl Display for BodyPart {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self.health {
            5 => write!(f, "{}: WOUNDED", self.name),
            4 => write!(f, "{}: WOUNDED", self.name),
            3 => write!(f, "{}: WOUNDED", self.name),
            2 => write!(f, "{}: CRITICAL", self.name),
            1 => write!(f, "{}: CRITICAL", self.name),
            0 => write!(f, "{}: DEAD", self.name),
            _ => write!(f, "{}: HEALTHY", self.name),
        }           
    }
}

impl BodyPart {
    fn new(name: String, health: u32) -> BodyPart {
        BodyPart {
            name,
            health
        }
    }
}

impl Creature {
    fn show_body_parts(&self) -> Vec<String>{
        let mut result = vec![];
        for body_part in self.body_parts.clone() {
            result.push(body_part.to_string());
        }
        result
    }

}
impl Creature {
    fn hit(&mut self, body_part: &str) {  
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


fn main() {
    let mut orc = Creature::new_orc("Orc Berserker".to_string());
    loop {
            let mut turns_left = 5;
        loop {
            if turns_left == 0 {
                break;
            }
    let selection = inquire::Select::new(&format!("Where do you want to hit {}", orc.name) , orc.show_body_parts())
        .prompt()
        .unwrap();
    let body_part: Vec<&str> = selection.split(":").collect();
    orc.hit(body_part.first().unwrap());

    turns_left -= 1;
        }
        println!("The orc attacked you")
    }
}
