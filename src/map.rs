use crate::*;

const TILE_SIZE: i32 = 16;

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct Unit {
    player_owned: bool,
    x: i32,
    y: i32,
    health: i32,
    maxhealth: i32,
    damage: i32,
    moverange: i32,
    attackrange: i32,
    //whatever else a unit needs
}

//#[derive(Clone)]
#[allow(dead_code)]
pub struct Map {
    grid: Vec<Vec<i32>>, //2d array (2d vec) of i32 (IDs) that correspond to tile types (textures for the tiles, wall, ground, etc.)
    width: i32,
    height: i32,
    x: i32,
    y: i32,
    units: Vec<Unit>,
    tiles: Texture2D, //the ID in grid[x][y] correlates to an image in this Vec. EG.: map.tiles[ map.grid[x][y] ] would access the tile image for the x y position on the map
}

#[allow(dead_code)]
impl Map {
    pub fn new(width: usize, height: usize, rl: &mut RaylibHandle, thread: &mut RaylibThread) -> Self{
        let mut grid: Vec<Vec<i32>> = vec![];
        grid.resize(width, vec![]);
        for x in 0..width {
            grid[x].resize(height, 0);
        }
        Map {
            grid: grid,
            width: width as i32,
            height: height as i32,
            x: 0,
            y: 0,
            units: vec![],
            tiles: rl.load_texture(thread, "art/Dungeon_Tileset.png").unwrap(),
        }
    }
    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        for x in 0..self.width {
            for y in 0..self.height {
                //Used ID to determine x and y position of tile on the tileset.
                let id = self.grid[x as usize][y as usize];
                let source = Rectangle::new(((id % 10) * TILE_SIZE) as f32, ((id / 10) * TILE_SIZE) as f32, TILE_SIZE as f32, TILE_SIZE as f32);
                let position = Vector2::new((x * TILE_SIZE) as f32, (y * TILE_SIZE) as f32);
                d.draw_texture_rec(&self.tiles, source, position, Color::WHITE);
            }
        }
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
        FillNode {
            x: x,
            y: y,
            depth: depth,
        }
    }
}

fn add_fill_node(map: &Map, dx: i32, dy: i32, n: &FillNode, visited: &mut Vec<bool>, q: &mut Vec<FillNode>, path: &mut Vec<(i32, i32)>, range: i32, heuristic: fn(i32) -> i32) {
    if n.x + dx < 0 || n.x + dx > map.width - 1 {
        return;
    }
    if n.y + dy < 0 || n.y + dy > map.height - 1 {
        return;
    }
    if n.depth >= range {
        return;
    }
    let h = heuristic(map.grid[n.x as usize][n.y as usize]);
    if h == -1 {
        return;
    }

    if !visited[ ((n.x+dx) + (n.y+dy) * map.width) as usize ] {
        visited[ ((n.x+dx) + (n.y+dy) * map.width) as usize ] = true;
        q.push( FillNode::new(n.x+dx, n.y+dy, n.depth+h) );
        path.push( (n.x+dx, n.y+dy) );
    }
}

#[allow(dead_code)]
fn floodfill(map: Map, start: (i32, i32), range: i32, heuristic: fn(i32) -> i32) -> Vec<(i32, i32)> {
    //Set up visited array
    let mut visited: Vec<bool> = vec![];
    visited.reserve( (map.width * map.height) as usize);
    visited[ (start.0 + start.1 * map.width) as usize ] = true;

    //Set up queue
    let mut q: Vec<FillNode> = vec![];
    q.push( FillNode::new(start.0, start.1, 0) );

    //set up result path
    let mut path: Vec<(i32, i32)> = vec![];
    path.push(start);

    while !q.is_empty() {
        let n = q.first().unwrap().clone();
        q.pop();

        add_fill_node(&map, -1,  0, &n, &mut visited, &mut q, &mut path, range, heuristic);
        add_fill_node(&map,  1,  0, &n, &mut visited, &mut q, &mut path, range, heuristic);
        add_fill_node(&map,  0, -1, &n, &mut visited, &mut q, &mut path, range, heuristic);
        add_fill_node(&map,  0,  1, &n, &mut visited, &mut q, &mut path, range, heuristic);
    }

    path
}
