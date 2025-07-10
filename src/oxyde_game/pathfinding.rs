
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug)]
pub struct Node {
    position: Position,
    f_score: f32,
    g_score: f32,
    parent: Option<Box<Node>>,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
    }
}

impl Eq for Node {}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.f_score.partial_cmp(&self.f_score)
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

pub fn find_path(start: Position, goal: Position, obstacles: &[Position]) -> Option<Vec<Position>> {
    let mut open_set = BinaryHeap::new();
    let mut closed_set = HashMap::new();
    
    let start_node = Node {
        position: start.clone(),
        f_score: heuristic(&start, &goal),
        g_score: 0.0,
        parent: None,
    };
    
    open_set.push(start_node);
    
    while let Some(current) = open_set.pop() {
        if current.position == goal {
            return Some(reconstruct_path(&current));
        }
        
        closed_set.insert((current.position.x, current.position.y), current.g_score);
        
        for neighbor in get_neighbors(&current.position, obstacles) {
            let g_score = current.g_score + distance(&current.position, &neighbor);
            
            if let Some(&best_g) = closed_set.get(&(neighbor.x, neighbor.y)) {
                if g_score >= best_g {
                    continue;
                }
            }
            
            let h_score = heuristic(&neighbor, &goal);
            let f_score = g_score + h_score;
            
            let neighbor_node = Node {
                position: neighbor,
                f_score,
                g_score,
                parent: Some(Box::new(current.clone())),
            };
            
            open_set.push(neighbor_node);
        }
    }
    
    None
}

fn heuristic(a: &Position, b: &Position) -> f32 {
    let dx = b.x - a.x;
    let dy = b.y - a.y;
    (dx * dx + dy * dy).sqrt()
}

fn distance(a: &Position, b: &Position) -> f32 {
    heuristic(a, b)
}

fn get_neighbors(pos: &Position, obstacles: &[Position]) -> Vec<Position> {
    let dirs = [
        (1.0, 0.0), (-1.0, 0.0),
        (0.0, 1.0), (0.0, -1.0),
        (1.0, 1.0), (-1.0, -1.0),
        (1.0, -1.0), (-1.0, 1.0),
    ];
    
    dirs.iter()
        .map(|(dx, dy)| Position {
            x: pos.x + dx,
            y: pos.y + dy,
        })
        .filter(|p| !obstacles.contains(p))
        .collect()
}

fn reconstruct_path(end_node: &Node) -> Vec<Position> {
    let mut path = vec![end_node.position.clone()];
    let mut current = end_node;
    
    while let Some(ref parent) = current.parent {
        path.push(parent.position.clone());
        current = parent;
    }
    
    path.reverse();
    path
}
