extern crate byteorder;
use crate::*;
use std::fs::File;
use std::io::prelude::*;
pub const TILE_SIZE: i32 = 16;
pub const SCALE: f32 = 3.0;

pub struct Map {
    pub grid: Vec<Vec<i32>>, //2d array (2d vec) of i32 (IDs) that correspond to tile types (textures for the tiles, wall, ground, etc.)
    pub width: i32,
    pub height: i32,
    pub x: i32,
    pub y: i32,
    pub tiles: Texture2D,
}
#[allow(dead_code)]
#[allow(clippy::needless_range_loop)]
impl Map {
    pub fn new(
        width: usize,
        height: usize,
        rl: &mut RaylibHandle,
        thread: &mut RaylibThread,
    ) -> Self {
        let mut grid: Vec<Vec<i32>> = vec![];
        grid.resize(width, vec![]);
        for x in 0..width {
            grid[x].resize(height, 78);
        }
        Map {
            grid,
            width: width as i32,
            height: height as i32,
            x: 0,
            y: 0,
            tiles: rl.load_texture(thread, "art/Dungeon_Tileset.png").unwrap(),
        }
    }
    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        for x in 0..self.width {
            for y in 0..self.height {
                //Used ID to determine x and y position of tile on the tileset.
                let id = self.grid[x as usize][y as usize];
                let source = Rectangle::new(
                    ((id % 10) * TILE_SIZE) as f32,
                    ((id / 10) * TILE_SIZE) as f32,
                    TILE_SIZE as f32,
                    TILE_SIZE as f32,
                );
                let dest = Rectangle::new(
                    (x * (TILE_SIZE as f32 * SCALE) as i32) as f32,
                    (y * (TILE_SIZE as f32 * SCALE) as i32) as f32,
                    TILE_SIZE as f32 * SCALE,
                    TILE_SIZE as f32 * SCALE,
                );
                d.draw_texture_pro(
                    &self.tiles,
                    source,
                    dest,
                    Vector2::new(0.0, 0.0),
                    0.0,
                    Color::WHITE,
                );
            }
        }
    }
    pub fn save(&self) -> std::io::Result<()> {
        let mut file = File::create("saved.txt")?;
        for y in 0..self.height {
            for x in 0..self.width {
                file.write_all(&self.grid[x as usize][y as usize].to_le_bytes())
                    .expect("error");
            }
        }
        Ok(())
    }

    pub fn load(&mut self) -> std::io::Result<()> {
        let mut count = 0;
        let mut data = Vec::new();
        let mut f = File::open("saved.txt").expect("unable to open file");
        f.read_to_end(&mut data).expect("Unable to read data");
        println!("{}", data.len());
        println!("{}", data[0]);
        for y in 0..self.height {
            for x in 0..self.width {
                if data[count] == 0 {
                    //println!("{}", data[count]);
                    count += 3;
                }
                self.grid[x as usize][y as usize] = i32::from(data[count]);
                count += 1;
            }
        }
        count = 0;
        let mut filet = File::create("loaded.txt")?;
        for _y in 0..self.height {
            for _x in 0..self.width {
                if data[count] == 0 {
                    //println!("{}", data[count]);
                    count += 3;
                }
                filet.write_all(&data[count].to_le_bytes()).expect("error");
                count += 1;
            }
        }
        Ok(())
    }
}

//Algorithm to get the range of possible movements for a unit
#[derive(Clone, Copy)]
struct FillNode {
    x: i32,
    y: i32,
    depth: i32,
}

impl FillNode {
    fn new(x: i32, y: i32, depth: i32) -> Self {
        FillNode { x, y, depth }
    }
}
#[allow(clippy::too_many_arguments)] //Perhaps fix this later with another func or struct so we don't need an exception.
fn add_fill_node(
    map: &Map,
    dx: i32,
    dy: i32,
    n: &FillNode,
    visited: &mut Vec<bool>,
    q: &mut VecDeque<FillNode>,
    path: &mut Vec<(i32, i32)>,
    range: i32,
    heuristic: fn(i32) -> i32,
) {
    let newx = n.x + dx;
    let newy = n.y + dy;
    if newx < 0 || newx > map.width - 1 {
        return;
    }
    if newy < 0 || newy > map.height - 1 {
        return;
    }
    if n.depth >= range {
        return;
    }
    let h = heuristic(map.grid[newx as usize][newy as usize]);
    if h == -1 {
        return;
    }
    if !visited[(newx + newy * map.width) as usize] {
        visited[(newx + newy * map.width) as usize] = true;
        q.push_back(FillNode::new(newx, newy, n.depth + h));
        path.push((newx, newy));
    }
}

pub fn floodfill(
    map: &Map,
    start: (i32, i32),
    range: i32,
    heuristic: fn(i32) -> i32,
) -> Vec<(i32, i32)> {
    //Set up visited array
    let mut visited: Vec<bool> = vec![false; (map.width * map.height) as usize];
    visited[(start.0 + start.1 * map.width) as usize] = false;

    //Set up queue
    let mut q = VecDeque::new();
    q.push_back(FillNode::new(start.0, start.1, 0));

    //set up result path
    let mut path: Vec<(i32, i32)> = vec![];
    path.push(start);
    while !q.is_empty() {
        let n = *q.front().unwrap(); //.clone(); //Cloneing was uneeded since we could dereference with * instead, this should work.
        q.pop_front();
        add_fill_node(
            &map,
            -1,
            0,
            &n,
            &mut visited,
            &mut q,
            &mut path,
            range,
            heuristic,
        );
        add_fill_node(
            &map,
            1,
            0,
            &n,
            &mut visited,
            &mut q,
            &mut path,
            range,
            heuristic,
        );
        add_fill_node(
            &map,
            0,
            -1,
            &n,
            &mut visited,
            &mut q,
            &mut path,
            range,
            heuristic,
        );
        add_fill_node(
            &map,
            0,
            1,
            &n,
            &mut visited,
            &mut q,
            &mut path,
            range,
            heuristic,
        );
    }

    path
}
