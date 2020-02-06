pub struct Unit {
    player_owned: bool,
    health: i32,
    maxhealth: i32,
    damage: i32,
    moverange: i32,
    attackrange: i32,
    //whatever else a unit needs
}

pub struct Map {
    grid: Vec<Vec<i32>>, //2d array (2d vec) of i32 (IDs) that correspond to tile types (textures for the tiles, wall, ground, etc.)
    width: u32,
    height: u32,
    units: Vec<Unit>,
}

//Algorithm to get the range of possible movements for a unit
struct FillNode {
    x: i32,
    y: i32,
    depth: i32,
}

fn add_fill_node(map: Map, dx: i32, dy: i32, n: &mut FillNode, visited: bool[], Q: queue<FillNode>, path: Vec<(i32, i32)>, range: i32, heuristic) {

}

fn floodfill(map: Map, start: (i32, i32), range: i32, heuristic) {

}