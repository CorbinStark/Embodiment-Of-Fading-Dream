use crate::*;

/* enums don't seem to work the way I want them to, so shelving for now
#[derive(Copy, Clone)]
#[allow(dead_code)]

pub enum Class {
    sword {
        maxhealth = 20,
        moverange = 5,
        attackrange: 1,

        armor = 1,   //resists from attack damage
        maxdamage = 8,
        mindamage = 5,
        basehit = 90, //percent hit
        counter = true, //can autoattack after being attacked, assuming in range
    },
    bow {
        maxhealth = 15,
        moverange = 4,
        attackrange = 2,

        armor = 0,
        maxdamage = 10,
        mindamage = 7,
        basehit = 80,
        counter = false,

    },
    knight {
        maxhealth = 15,
        moverange = 3,
        attackrange = 1,

        armor = 5,
        maxdamage = 6,
        mindamage = 5,
        basehit = 90,
        counter = true,
    }
}*/

#[derive(Clone)]
#[allow(dead_code)]
pub struct Unit {
    id: i32, //for texture drawing
    name: String,

    player_owned: bool,
    alive: bool,
    counter: bool,

    health: i32,
    maxhealth: i32,
    moverange: i32,
    attackrange: i32,
    armor: i32,
    maxdamage: i32,
    mindamage: i32,
    basehit: i32,
    //whatever else a unit needs
}

impl Unit {
    //
    //Constructors for various unit types
    //
    pub fn new(player_owned: bool) -> Self {
        Unit {
            id: 0,
            name: "Default".to_string(),

            player_owned: player_owned,
            alive: true,
            counter: true,

            maxhealth: 0,
            health: 0,
            moverange: 0,
            attackrange: 0,
            armor: 0,
            maxdamage: 0,
            mindamage: 0,
            basehit: 0,
        }
    }
    pub fn new_sword(player_owned: bool) -> Self {
        Unit {
            id: 1,
            name: "Swordsman".to_string(),

            player_owned: player_owned,
            alive: true,
            counter: true,

            maxhealth: 20,
            health: 20,
            moverange: 5,
            attackrange: 1,
            armor: 2,
            maxdamage: 8,
            mindamage: 5,
            basehit: 90,
        }
    }
    pub fn new_knight(player_owned: bool) -> Self {
        Unit {
            id: 2,
            name: "Knight".to_string(),

            player_owned: player_owned,
            alive: true,
            counter: true,

            maxhealth: 20,
            health: 20,
            moverange: 3,
            attackrange: 1,
            armor: 5,
            maxdamage: 5,
            mindamage: 1,
            basehit: 90,
        }
    }
    pub fn new_archer(player_owned: bool) -> Self {
        Unit {
            id: 3,
            name: "Archer".to_string(),

            player_owned: player_owned,
            alive: true,
            counter: false,

            maxhealth: 15,
            health: 15,
            moverange: 4,
            attackrange: 2,
            armor: 0,
            maxdamage: 10,
            mindamage: 6,
            basehit: 80,
        }
    }
    pub fn new_custom(
        id: i32,
        name: &str,
        player_owned: bool,
        alive: bool,
        counter: bool,
        maxhealth: i32,
        health: i32,
        moverange: i32,
        attackrange: i32,
        armor: i32,
        maxdamage: i32,
        mindamage: i32,
        basehit: i32,
    ) -> Self {
        Unit {
            id: id,
            name: name.to_string(),

            player_owned: player_owned,
            alive: alive,
            counter: counter,

            maxhealth: maxhealth,
            health: health,
            moverange: moverange,
            attackrange: attackrange,
            armor: armor,
            maxdamage: maxdamage,
            mindamage: mindamage,
            basehit: basehit,
        }
    }

    //
    //Whatever functions units need
    //

    //this function is both a setter and a getter. It checks if a unit is dead,
    //if they aren't it checks if they should be, and sets them as dead accordingly
    pub fn is_alive(&mut self) -> bool {
        if self.alive && self.health > 0 {
            return true;
        }
        if !self.alive {
            return false;
        }
        if self.health <= 0 {
            self.alive = false;
            return false;
        } else {
            return true;
        }
    }

    //generates hit damage, returns -1 to indicate attack missing
    pub fn get_damage(&self) -> i32 {
        let mut rnjesus = rand::thread_rng();
        if rnjesus.gen_range(1, 100) <= self.basehit {
            return rnjesus.gen_range(self.mindamage, self.maxdamage);
        } else {
            return -1;
        }
    }
}

//
//Functions related to units that don't make sense as methods
//

//This function handles combat encounters
//
//Unit is the attacking unit, unit2 is the defending unit
//returns 0 for no units killed, 1 for defending unit killed, and 2 for attacking unit killed
pub fn combat(unit: &mut Unit, unit2: &mut Unit, range: i32) -> i32 {
    assert_eq!(unit.is_alive(), true, "Attacking unit isn't alive");
    assert_eq!(unit2.is_alive(), true, "Defending unit isn't alive");

    let attack = unit.get_damage();
    println!("The {} prepares to attack the {}!", unit.name, unit2.name);
    if attack == -1 {
        println!("The attack misses!");
    } else {
        let damage = attack - unit2.armor;
        if damage > 0 {
            unit2.health -= damage;
            println!(
            "The {} hits the {} for {} damage, leaving them with {} health!",
            unit.name, unit2.name, damage, unit2.health
        );
        } else {
            println!(
                "The {}'s attack can't get through the {}'s armor!",
                unit.name, unit2.name
            );
        }
    }
    if unit2.is_alive() {
        //Checks the defending unit is in range and can counter
        if range <= unit2.attackrange && unit2.counter {
            println!("The {} prepares to counter attack!", unit2.name);
            let counter_attack = unit2.get_damage();
            if counter_attack == -1 {
                println!("The counter attack misses!");
                println!("The combat ends.");
                return 0;
            } else {
                let counter_damage = counter_attack - unit.armor;
                if counter_damage > 0 {
                    unit.health -= counter_damage;
                    println!(
                        "The {} counterattacks the {} for {} damage, leaving them with {} health!",
                        unit2.name, unit.name, counter_damage, unit.health
                    );
                } else {
                    println!(
                        "The {}'s attack can't get through the {}'s armor!",
                        unit2.name, unit.name
                    );
                }
            }
            if unit.is_alive() {
                println!("The combat ends.");
                return 0;
            } else {
                println!("The attacking {} is killed!", unit.name);
                return 2;
            }
        } else {
            println!("The defending {} is unable to strike back.", unit2.name);
            println!("The combat ends.");
            return 0;
        }
    } else {
        println!("The defending {} is killed!", unit2.name);
        println!("The combat ends.");
        return 1;
    }
}
