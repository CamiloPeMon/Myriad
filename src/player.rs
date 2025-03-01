use std::fmt::{write, Display, Formatter};
struct Player {
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
    pub fn new(&self,playername: String) -> Player {
        Player{
            name: playername,
            health: 100,
            is_bleading: false,
            player_head: PlayerHead{brain_health:3 ,skull_health:3 ,vision_health:3 ,tounge:true ,contusion: false,is_helmet: false ,helmet_id:"".to_string() },
            player_right_arm: PlayerArm{id:"Right".to_string() ,is_dominant:true, claw_power: 1,fractured: false,holding_item: false, item_id:"".to_string() },
            player_left_arm: PlayerArm{id:"Left".to_string() ,is_dominant:true, claw_power: 1,fractured: false,holding_item: false, item_id:"".to_string() },
            player_right_leg: PlayerLeg{id:"Right".to_string() ,fractured:false, is_shoes:false, shoes_id:"".to_string(), is_leg_armor:false,leg_armor_id:"".to_string()},
            player_left_leg: PlayerLeg{id:"Left".to_string() ,fractured:false, is_shoes:false, shoes_id:"".to_string(), is_leg_armor:false,leg_armor_id:"".to_string()},
            player_torso: PlayerTorso{player_organs:5, wounds:0 ,ribs_health:3,stomach:0}

        }       

    } 
    
}
struct PlayerHead {
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
    player_organs: u32,
    wounds: u32,
    ribs_health: u32,
    stomach:u32,
}



