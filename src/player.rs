use std::fmt::{write, Display, Formatter};
use rand::Rng;

pub struct Player {
    name: String,
    health: i32,
    is_bleading: bool,
    player_head: PlayerHead,
    player_right_arm: PlayerArm,
    player_left_arm: PlayerArm,
    player_right_leg: PlayerLeg,
    player_left_leg: PlayerLeg,
    player_torso: PlayerTorso,
}

impl Player {
    pub fn new(playername: String) -> Player {
        Player{
            name: playername,
            health: 100,
            is_bleading: false,
            player_head: PlayerHead{id:"Head".to_string(),brain_health:3 ,skull_health:3 ,vision_health:3 ,tounge:true ,contusion: false,is_helmet: false ,helmet_id:"".to_string() },
            player_right_arm: PlayerArm{id:"RightArm".to_string() ,is_dominant:true, claw_power: 1,fractured: false,holding_item: false, item_id:"".to_string() },
            player_left_arm: PlayerArm{id:"LeftArm".to_string() ,is_dominant:true, claw_power: 1,fractured: false,holding_item: false, item_id:"".to_string() },
            player_right_leg: PlayerLeg{id:"RightLeg".to_string() ,fractured:false, is_shoes:false, shoes_id:"".to_string(), is_leg_armor:false,leg_armor_id:"".to_string()},
            player_left_leg: PlayerLeg{id:"LeftLeg".to_string() ,fractured:false, is_shoes:false, shoes_id:"".to_string(), is_leg_armor:false,leg_armor_id:"".to_string()},
            player_torso: PlayerTorso{id:"Torso".to_string(),player_organs:5, wounds:0 ,ribs_health:3,stomach:3}

        }       

    } 
    pub fn acction_eat(&mut self, amount: i32){
        if(self.player_torso.stomach <= 3){
            self.player_torso.stomach += 1;
        }      
    }
    pub fn acction_make_damage_into_body(&mut self, damage: i32, skill: i32, critical:bool, blunt:bool,piercing:bool ){
        let mut rng = rand::thread_rng();
        let mut extra_damage=0;
        let mut luck=skill + rng.gen_range(1..=10);
        if  (luck >= 10){
            luck=10;
            
        }

        //Esto es terrible, hay que usar herencia en ves de este spagetti
        match luck {
            (10 ) => {
                if(self.player_head.skull_health > 0 && blunt == true){
                    self.player_head.skull_health = self.player_head.skull_health.saturating_sub(rng.gen_range(1..=3));
                }
                if (self.player_head.skull_health == 0){
                    extra_damage = 30;                                       
                }
                 
            }
            (5) => {
                let mut pierceChance= rng.gen_range(1..=5);
                if((blunt == true || piercing == true ) && pierceChance == 5){
                    self.player_right_arm.fractured=true;
                    self.player_right_arm.holding_item=false;
                }
            }
            (6) =>  {
                let mut pierceChance= rng.gen_range(1..=5);
                if((blunt ==true || piercing==true ) && pierceChance == 5){
                    self.player_left_arm.fractured=true;
                    self.player_left_arm.holding_item=false;
                }
            }
            (1 | 3) => {}
            (2 | 4) => {}
            (7|8|9) => {
                let mut pierceChance= rng.gen_range(1..=5);
                if((piercing == true ) && pierceChance == 5){
                    extra_damage = 20;  
                }

            }
            _ => { println!("Warning: Invalid body part '{}'", luck);
        }
        }
        self.health -= damage + extra_damage;


    }
    
}
struct PlayerHead {
    id: String,
    brain_health: u32,
    skull_health: u32,
    vision_health: u32,
    tounge: bool,
    contusion: bool,
    is_helmet: bool,
    helmet_id: String,
}
struct PlayerArm{
    id: String,
    is_dominant: bool,
    claw_power: u32,
    fractured: bool,
    holding_item: bool,
    item_id: String,
}
struct PlayerLeg{   
    id: String, 
    fractured: bool,
    is_shoes: bool,
    shoes_id: String,
    is_leg_armor: bool,
    leg_armor_id: String,
}
struct PlayerTorso{
    id: String,
    player_organs: u32,
    wounds: u32,
    ribs_health: u32,
    stomach:u32,
}



