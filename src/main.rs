fn main() {
    let mut orc = Creature::new_orc("Orc Berserker".to_string());
    loop {
        let mut turns_left = 5;
        loop {
            if turns_left == 0 {
                break;
            }
            let selection = Select::new(
                &format!("Where do you want to hit {}", orc.name),
                orc.show_body_parts(),
            )
            .prompt()
            .unwrap();
            let body_part: Vec<&str> = selection.split(":").collect();
            orc.hit(body_part.first().unwrap());

            turns_left -= 1;
        }
        println!("The orc attacked you")
    }
}
