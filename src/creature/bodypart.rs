use std::fmt::{Display, Formatter};

#[derive(Clone)]
pub struct BodyPart {
    pub name: String,
    pub health: u32,
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
    pub fn new(name: String, health: u32) -> BodyPart {
        BodyPart { name, health }
    }
}
