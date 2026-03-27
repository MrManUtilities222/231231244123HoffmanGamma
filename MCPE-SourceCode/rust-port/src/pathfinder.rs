/// AI Pathfinding system ported from world/entity/ai/PathNavigation.h 
/// and world/level/pathfinder/

use std::collections::{BinaryHeap, HashMap, HashSet};
use std::cmp::Ordering;

/// A position in the pathfinding grid
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PathNode {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl PathNode {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    pub fn distance_to(&self, other: &PathNode) -> f32 {
        let dx = (self.x - other.x) as f32;
        let dy = (self.y - other.y) as f32;
        let dz = (self.z - other.z) as f32;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    pub fn manhattan_to(&self, other: &PathNode) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }
}

/// A* search state
#[derive(Clone, Debug)]
struct SearchNode {
    pos: PathNode,
    g_cost: f32,
    f_cost: f32,
}

impl PartialEq for SearchNode {
    fn eq(&self, other: &Self) -> bool { self.pos == other.pos }
}
impl Eq for SearchNode {}

impl Ord for SearchNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f_cost.partial_cmp(&self.f_cost).unwrap_or(Ordering::Equal)
    }
}
impl PartialOrd for SearchNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// A computed path as a sequence of positions
#[derive(Clone, Debug)]
pub struct Path {
    pub nodes: Vec<PathNode>,
    pub current_index: usize,
}

impl Path {
    pub fn new(nodes: Vec<PathNode>) -> Self {
        Self { nodes, current_index: 0 }
    }

    pub fn is_done(&self) -> bool {
        self.current_index >= self.nodes.len()
    }

    pub fn current_node(&self) -> Option<&PathNode> {
        self.nodes.get(self.current_index)
    }

    pub fn advance(&mut self) {
        self.current_index += 1;
    }

    pub fn length(&self) -> usize {
        self.nodes.len()
    }
}

/// Checks if a mob can stand at position (walkable ground, head room)
fn is_walkable(level: &crate::level::Level, x: i32, y: i32, z: i32) -> bool {
    let below = level.get_tile(x, y - 1, z);
    let at = level.get_tile(x, y, z);
    let above = level.get_tile(x, y + 1, z);
    crate::tile::is_solid(below) && !crate::tile::is_solid(at) && !crate::tile::is_solid(above)
}

/// Find a path using A* from start to goal
pub fn find_path(
    level: &crate::level::Level,
    start: PathNode,
    goal: PathNode,
    max_distance: f32,
) -> Option<Path> {
    let mut open = BinaryHeap::new();
    let mut came_from: HashMap<PathNode, PathNode> = HashMap::new();
    let mut g_scores: HashMap<PathNode, f32> = HashMap::new();
    let mut closed: HashSet<PathNode> = HashSet::new();

    g_scores.insert(start, 0.0);
    open.push(SearchNode {
        pos: start,
        g_cost: 0.0,
        f_cost: start.distance_to(&goal),
    });

    let neighbors_offsets = [
        (1, 0, 0), (-1, 0, 0),
        (0, 0, 1), (0, 0, -1),
        (1, 0, 1), (-1, 0, 1), (1, 0, -1), (-1, 0, -1),
    ];

    let mut iterations = 0;
    let max_iterations = 200;

    while let Some(current) = open.pop() {
        iterations += 1;
        if iterations > max_iterations { break; }

        if current.pos == goal {
            // Reconstruct path
            let mut path = vec![goal];
            let mut pos = goal;
            while let Some(&prev) = came_from.get(&pos) {
                path.push(prev);
                pos = prev;
                if pos == start { break; }
            }
            path.reverse();
            return Some(Path::new(path));
        }

        if closed.contains(&current.pos) { continue; }
        closed.insert(current.pos);

        for &(dx, dy, dz) in &neighbors_offsets {
            let nx = current.pos.x + dx;
            let ny = current.pos.y + dy;
            let nz = current.pos.z + dz;
            let neighbor = PathNode::new(nx, ny, nz);

            if closed.contains(&neighbor) { continue; }
            if neighbor.distance_to(&start) > max_distance { continue; }

            // Check walkability - also try stepping up/down by 1
            let walkable_pos = if is_walkable(level, nx, ny, nz) {
                Some(neighbor)
            } else if is_walkable(level, nx, ny + 1, nz) {
                Some(PathNode::new(nx, ny + 1, nz))
            } else if is_walkable(level, nx, ny - 1, nz) {
                Some(PathNode::new(nx, ny - 1, nz))
            } else {
                None
            };

            if let Some(walk_pos) = walkable_pos {
                let step_cost = if dx != 0 && dz != 0 { 1.414 } else { 1.0 };
                let new_g = current.g_cost + step_cost;
                let old_g = g_scores.get(&walk_pos).copied().unwrap_or(f32::MAX);

                if new_g < old_g {
                    g_scores.insert(walk_pos, new_g);
                    came_from.insert(walk_pos, current.pos);
                    open.push(SearchNode {
                        pos: walk_pos,
                        g_cost: new_g,
                        f_cost: new_g + walk_pos.distance_to(&goal),
                    });
                }
            }
        }
    }

    None
}
