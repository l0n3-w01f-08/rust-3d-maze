use std::fs::File;
use std::io::Write;
use rand::seq::SliceRandom;

#[derive(Copy, Clone, Debug, PartialEq, serde::Serialize)]
enum CellType {
    Wall,
    Path,
}

#[derive(serde::Serialize)]
struct MazeData {
    width: usize,
    height: usize,
    depth: usize,
    grid: Vec<Vec<Vec<CellType>>>,
}

fn main() {
    println!("--- Running 3D Procedural Maze Engine with JSON Export ---");
    
    let width = 15;
    let height = 15;
    let depth = 3;
    
    let mut grid = vec![vec![vec![CellType::Wall; width]; height]; depth];
    let mut frontier = Vec::new();
    
    // Start at a random coordinate
    let start_x = 1;
    let start_y = 1;
    let start_z = 0;
    grid[start_z][start_y][start_x] = CellType::Path;
    
    add_neighbors(start_x, start_y, start_z, width, height, depth, &grid, &mut frontier);
    
    let mut rng = rand::thread_rng();
    
    while let Some((fx, fy, fz, px, py, pz)) = frontier.choose(&mut rng).copied() {
        frontier.retain(|&x| x != (fx, fy, fz, px, py, pz));
        
        if grid[fz][fy][fx] == CellType::Wall {
            grid[fz][fy][fx] = CellType::Path;
            grid[pz][py][px] = CellType::Path; // Carve the wall between them
            
            add_neighbors(fx, fy, fz, width, height, depth, &grid, &mut frontier);
        }
    }
    
    // Export to JSON file
    let maze_data = MazeData { width, height, depth, grid };
    let json_string = serde_json::to_string_pretty(&maze_data).unwrap();
    
    let mut file = File::create("maze_data.json").expect("Failed to create file");
    file.write_all(json_string.as_bytes()).expect("Failed to write data");
    
    println!("🚀 Success! 3D layout data exported to 'maze_data.json'");
}

fn add_neighbors(x: usize, y: usize, z: usize, w: usize, h: usize, d: usize, grid: &Vec<Vec<Vec<CellType>>>, frontier: &mut Vec<(usize, usize, usize, usize, usize, usize)>) {
    let dirs = [(0, -2, 0), (0, 2, 0), (-2, 0, 0), (2, 0, 0), (0, 0, -1), (0, 0, 1)];
    
    for (dx, dy, dz) in dirs.iter() {
        let nx = x as idisize + dx;
        let ny = y as idisize + dy;
        let nz = z as idisize + dz;
        
        if nx > 0 && nx < (w - 1) as idisize && ny > 0 && ny < (h - 1) as idisize && nz >= 0 && nz < d as idisize {
            let nx = nx as usize;
            let ny = ny as usize;
            let nz = nz as usize;
            
            if grid[nz][ny][nx] == CellType::Wall {
                // Store frontier cell and the wall cell between it and the current path
                let cx = (x as idisize + dx / 2) as usize;
                let cy = (y as idisize + dy / 2) as usize;
                let cz = (z as idisize + dz / 2) as usize;
                frontier.push((nx, ny, nz, cx, cy, cz));
            }
        }
    }
}