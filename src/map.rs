use crate::*;

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct Unit {
    player_owned: bool,
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
    units: Vec<Unit>,
    tiles: Vec<Texture2D>, //the ID in grid[x][y] correlates to an image in this Vec. EG.: map.tiles[ map.grid[x][y] ] would access the tile image for the x y position on the map
}

#[allow(dead_code)]
impl Map {
    fn new(grid: Vec<Vec<i32>>,
    width: i32,
    height: i32,
    units: Vec<Unit>,
    tiles: Vec<Texture2D>) -> Self {
        Map {
            grid: grid,
            width: width,
            height: height,
            units: units,
            tiles: tiles,
        }
    }
    fn copy(other: Map) -> Self {
        Map {
            grid: other.grid,
            width: other.width,
            height: other.height,
            units: other.units,
            tiles: other.tiles,
        }
    }
}

//psuedocode for drawing the map
//for(int x = 0; x < map.width; ++x) {
//  for(int y = 0; y < map.height; ++y) {
//      draw_texture(map.tiles[ map.grid[x][y] ], x * tile_size, y * tile_size);

//Algorithm to get the range of possible movements for a unit
#[derive(Clone, Copy)]
#[allow(dead_code)]
struct FillNode {
    x: i32,
    y: i32,
    depth: i32,
}

#[allow(dead_code)]
impl FillNode {
    fn new(x: i32, y: i32, depth: i32) -> Self {
        FillNode {
            x: x,
            y: y,
            depth: depth,
        }
    }
}

#[allow(dead_code)]
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

    //let mapcopy = Map {
      //  grid: map.grid.clone(),
      //  width: map.width,
      //  height: map.height,
      //  units: map.units.clone(),
      //  tiles: map.tiles.clone(),
    //};
    //let h = heuristic(mapcopy, n.x, n.y);
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
fn floodfill(map: Map, start: (i32, i32), range: i32, heuristic: fn(i32) -> i32) -> Vec<(i32, i32)>{
    let mut visited: Vec<bool> = vec![];
    visited.reserve( (map.width * map.height) as usize);
    visited[ (start.0 + start.1 * map.width) as usize ] = true;

    let mut q: Vec<FillNode> = vec![];
    q.push( FillNode::new(start.0, start.1, 0) );

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