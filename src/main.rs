use std::cmp::max;
use std::cmp::min;

// ==========================================
// 1. DATA STRUCTURES & TYPES
// ==========================================

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum CellType {
    Wall,
    Path,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Coord3D {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

pub struct Maze3D {
    pub width: usize,
    pub height: usize,
    pub depth: usize,
    pub grid: Vec<Vec<Vec<CellType>>>,
}

// ==========================================
// 2. CORE 3D MAZE GENERATION LOGIC
// ==========================================

impl Maze3D {
    pub fn new(width: usize, height: usize, depth: usize) -> Self {
        let grid = vec![vec![vec![CellType::Wall; depth]; height]; width];
        Self { width, height, depth, grid }
    }

    pub fn generate(&mut self) {
        // Linear Congruential Generator (LCG) pseudo-random engine
        // This avoids needing external dependencies so it runs anywhere instantly
        let mut seed: u64 = 123456789;
        let mut next_random = move || {
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            seed
        };

        let start = Coord3D { x: 1, y: 1, z: 1 };
        self.grid[start.x][start.y][start.z] = CellType::Path;

        let mut frontier: Vec<Coord3D> = Vec::new();
        self.add_neighbors_to_frontier(start, &mut frontier);

        while !frontier.is_empty() {
            let index = (next_random() as usize) % frontier.len();
            let current_wall = frontier.remove(index);

            if self.grid[current_wall.x][current_wall.y][current_wall.z] == CellType::Wall {
                let paths = self.get_path_neighbors(current_wall);
                if !paths.is_empty() {
                    let path_index = (next_random() as usize) % paths.len();
                    let target_path = paths[path_index];

                    self.grid[current_wall.x][current_wall.y][current_wall.z] = CellType::Path;
                    
                    let between_x = ((current_wall.x + target_path.x) / 2) as usize;
                    let between_y = ((current_wall.y + target_path.y) / 2) as usize;
                    let between_z = ((current_wall.z + target_path.z) / 2) as usize;
                    self.grid[between_x][between_y][between_z] = CellType::Path;

                    self.add_neighbors_to_frontier(current_wall, &mut frontier);
                }
            }
        }
    }

    fn add_neighbors_to_frontier(&self, pos: Coord3D, frontier: &mut Vec<Coord3D>) {
        let directions = [
            (2, 0, 0), (-2, 0, 0),
            (0, 2, 0), (0, -2, 0),
            (0, 0, 2), (0, 0, -2),
        ];

        for &(dx, dy, dz) in &directions {
            let nx = pos.x as i32 + dx;
            let ny = pos.y as i32 + dy;
            let nz = pos.z as i32 + dz;

            if nx > 0 && nx < (self.width - 1) as i32
                && ny > 0 && ny < (self.height - 1) as i32
                && nz > 0 && nz < (self.depth - 1) as i32 
            {
                let neighbor = Coord3D { x: nx as usize, y: ny as usize, z: nz as usize };
                if self.grid[neighbor.x][neighbor.y][neighbor.z] == CellType::Wall {
                    frontier.push(neighbor);
                }
            }
        }
    }

    fn get_path_neighbors(&self, pos: Coord3D) -> Vec<Coord3D> {
        let mut paths = Vec::new();
        let directions = [
            (2, 0, 0), (-2, 0, 0),
            (0, 2, 0), (0, -2, 0),
            (0, 0, 2), (0, 0, -2),
        ];

        for &(dx, dy, dz) in &directions {
            let nx = pos.x as i32 + dx;
            let ny = pos.y as i32 + dy;
            let nz = pos.z as i32 + dz;

            if nx > 0 && nx < (self.width - 1) as i32
                && ny > 0 && ny < (self.height - 1) as i32
                && nz > 0 && nz < (self.depth - 1) as i32 
            {
                let neighbor = Coord3D { x: nx as usize, y: ny as usize, z: nz as usize };
                if self.grid[neighbor.x][neighbor.y][neighbor.z] == CellType::Path {
                    paths.push(neighbor);
                }
            }
        }
        paths
    }
}

// ==========================================
// 3. MAIN RUNTIME LOOP
// ==========================================

fn main() {
    // Note: Dimensions MUST be odd numbers for the Prim algorithm boundaries!
    let width = 15;
    let height = 15;
    let depth = 5; 

    println!("--- Running 3D Procedural Maze Engine ---");
    let mut maze = Maze3D::new(width, height, depth);
    
    println!("Carving paths out across Z, Y, and X axes...");
    maze.generate();

    // Print the 3D grid slice-by-slice along the Z-axis (height layers)
    for z in 1..(depth - 1) {
        println!("\n================ FLOOR LEVEL LAYER: {} ================", z);
        for y in 0..height {
            for x in 0..width {
                match maze.grid[x][y][z] {
                    CellType::Wall => print!("██"), 
                    CellType::Path => print!("  "), 
                }
            }
            println!(); 
        }
    }
    println!("\n3D Maze Generation Complete!");
}
