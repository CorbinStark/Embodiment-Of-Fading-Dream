use crate::*;

#[derive(Clone)]
pub struct Unit {
    id: i32,           //for texture drawing
    currentframe: i32, //to get image, refer to images[(ID * 4) + currentframe]
    name: String,
    pub x: i32,
    pub y: i32,

    alive: bool,
    counter: bool,

    pub health: i32,
    maxhealth: i32,
    pub moverange: i32,
    pub attackrange: i32,
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
    #[allow(dead_code)]
    pub fn new() -> Self {
        Unit {
            id: 0,
            currentframe: 0,
            name: "Default".to_string(),
            x: 0,
            y: 0,

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
    #[allow(dead_code)]
    #[allow(clippy::too_many_arguments)] //Very large function, requires the exception if we aren't reducing it.
    pub fn new_custom(
        id: i32,
        name: &str,
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
        x: i32,
        y: i32,
    ) -> Self {
        Unit {
            id,
            currentframe: 0,
            name: name.to_string(),
            x: x * (TILE_SIZE as f32 * SCALE) as i32,
            y: y * (TILE_SIZE as f32 * SCALE) as i32,

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
            false
        } else {
            true
        }
    }

    //generates hit damage, returns -1 to indicate attack missing
    pub fn get_damage(&self) -> i32 {
        let mut rnjesus = rand::thread_rng();
        if rnjesus.gen_range(1, 100) <= self.basehit {
            rnjesus.gen_range(self.mindamage, self.maxdamage)
        } else {
            -1
        }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle, images: &[Texture2D], timer: i32) {
        //&Vec<Texture2D>)
        let source = Rectangle::new(0.0, 0.0, 16.0, 16.0);
        let dest = Rectangle::new(self.x as f32, self.y as f32, 16.0 * SCALE, 16.0 * SCALE);
        d.draw_texture_pro(
            &images[((self.id * 4) + self.currentframe) as usize],
            source,
            dest,
            Vector2::new(0.0, 0.0),
            0.0,
            Color::WHITE,
        );
        if timer % 15 == 0 {
            self.currentframe += 1;
            if self.currentframe >= 4 {
                self.currentframe = 0;
            }
        }
    }

    pub fn ismoused(&self, mouse: Vector2, tile_size: f32, scale: f32) -> bool {
        mouse.x > self.x as f32
            && mouse.y > self.y as f32
            && mouse.x < self.x as f32 + (tile_size * scale)
            && mouse.y < self.y as f32 + (tile_size * scale)
    }
}
