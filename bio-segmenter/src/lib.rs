pub mod graphbuilder {
    extern crate image;
    extern crate petgraph;

    use image::*;
    use petgraph::graph::{Graph};

    use std::collections::{HashMap, HashSet};

    type Node = u64;
    type Coord = (u32, u32);

    pub fn get_bounds(value: u32, max: u32, radius: u32) -> (u32, u32) {
        let mut min_bound: u32 = 0;
        let mut max_bound: u32 = max;

        if value > radius {
            min_bound = value - radius;
        }
        
        if (value + radius) < max {
            max_bound = value + radius;
        }

        (min_bound, max_bound)
    }

    fn check_neighbors(node: &u64, nodes: &HashMap<u64, Coord>, 
                       nodes_lookup: &HashMap<Coord, u64>, img: &GrayImage,
                       adj_list: &mut HashMap<Node, HashSet<Node>>, radius: u32, threshold: u64) {
        let node_coords = nodes.get(node).unwrap();
        let (x_min, x_max) = get_bounds(node_coords.0, img.width(), radius);
        let (y_min, y_max) = get_bounds(node_coords.1, img.height(), radius);
        
        let node_pixel_val = img.get_pixel(node_coords.0, node_coords.1).channels()[0] as i32;
        //print!("{} {} ", node_coords.0, node_coords.1);
        //println!("{}", node_pixel.channels()[0]);
        for y in y_min..y_max {
            for x in x_min..x_max {
                let neighbor_coords = (x as u32, y as u32);
                if &neighbor_coords != node_coords {
                    let neighbor_pixel_val = img.get_pixel(neighbor_coords.0, neighbor_coords.1).channels()[0] as i32;
                    // if node_pixel.channels()[0] > neighbor_pixel.channels()[0]        
                    if (node_pixel_val - neighbor_pixel_val).abs() as u64 > threshold {
                       // TODO well this is sloppy
                        adj_list.get_mut(&node).unwrap().insert(nodes_lookup.get(&neighbor_coords).unwrap().to_owned()); 
                    }
                }
            }
        }

    }

    // TODO: get rid of this graph abstraction maybe - if using label prop
    // only need an adjacency list
    pub fn build_graph(file_path: String, radius: u32, threshold: u64) -> HashMap<Node, HashSet<Node>> {
        let img = image::open(file_path).unwrap().to_luma();
        
//        let mut g: Graph<u64, ()> = Graph::new();
        let mut adj_list: HashMap<Node, HashSet<Node>> = HashMap::new();
        // init nodes
        // TODO: abstract this?
        let mut nodes: HashMap<Node, Coord> = HashMap::new();
        let mut nodes_lookup: HashMap<Coord, Node> = HashMap::new();
        let mut node_id: u64 = 0;

        for pixel in img.enumerate_pixels() {
            nodes.insert(node_id, (pixel.0, pixel.1));
            nodes_lookup.insert((pixel.0, pixel.1), node_id);
//            g.add_node(node_id);
            adj_list.insert(node_id, HashSet::new());
            node_id += 1;
        }

        for (node, _coord) in nodes.iter() {
            check_neighbors(&node, &nodes, &nodes_lookup, &img, 
                            &mut adj_list, radius, threshold); 
        }
        //for (key, val) in nodes.iter() {
        //    println!("{}: {}, {}", key, val.0, val.1);
        //}
        adj_list 
    }

}

#[cfg(test)]
mod tests {
    use super::graphbuilder::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_get_bounds_0() {
        assert_eq!((0, 7), get_bounds(2, 42, 5))
    }

    #[test]
    fn test_get_bounds_1() {
        assert_eq!((23, 30), get_bounds(28, 30, 5))
    }

    #[test]
    fn test0() {
        //build_graph("/Users/wigasper/repos/bio-segmenter/36pixel.png".to_owned());
        let adj_list = build_graph("/media/storage/bio-segmenter/ct_scan.png".to_owned(), 3, 10);
        for (key, val) in adj_list.iter() {
            print!("{}: ", key);
            for node in val.iter() {
                print!("{} ", node);
            }
            println!();
        }
    }
}
