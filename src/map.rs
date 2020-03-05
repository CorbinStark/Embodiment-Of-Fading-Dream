use crate::*;

const TILE_SIZE: i32 = 16;
const SCALE: f32 = 3.0;

//#[derive(Clone)]
#[allow(dead_code)]
pub struct Map {
    pub grid: Vec<Vec<i32>>, //2d array (2d vec) of i32 (IDs) that correspond to tile types (textures for the tiles, wall, ground, etc.)
    pub width: i32,
    pub height: i32,
    pub x: i32,
    pub y: i32,
    pub units: Vec<Unit>,
    pub tiles: Texture2D,
    pub sprites: Vec<Texture2D>,
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
            units: vec![],
            tiles: rl.load_texture(thread, "art/Dungeon_Tileset.png").unwrap(),
            sprites: vec![],
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
        for unit in &self.units {
            unit.draw(d, &self.sprites);
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
    q: &mut Vec<FillNode>,
    path: &mut Vec<(i32, i32)>,
    range: i32,
    heuristic: fn(i32) -> i32,
) {
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

    if !visited[((n.x + dx) + (n.y + dy) * map.width) as usize] {
        visited[((n.x + dx) + (n.y + dy) * map.width) as usize] = true;
        q.push(FillNode::new(n.x + dx, n.y + dy, n.depth + h));
        path.push((n.x + dx, n.y + dy));
    }
}

#[allow(dead_code)]
pub fn floodfill(
    map: &Map,
    start: (i32, i32),
    range: i32,
    heuristic: fn(i32) -> i32,
) -> Vec<(i32, i32)> {
    //Set up visited array
    let mut visited: Vec<bool> = vec![false; (map.width * map.height) as usize];
    visited[(start.0 + start.1 * map.width) as usize] = true;

    //Set up queue
    let mut q: Vec<FillNode> = vec![];
    q.push(FillNode::new(start.0, start.1, 0));

    //set up result path
    let mut path: Vec<(i32, i32)> = vec![];
    path.push(start);

    while !q.is_empty() {
        let n = *q.last().unwrap(); //.clone(); //Cloneing was uneeded since we could dereference with * instead, this should work.
        q.pop();

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
