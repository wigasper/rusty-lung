pub mod label_prop;

pub mod graphbuilder {
    extern crate image;

    use image::*;

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
            max_bound = value + radius + 1;
        }

        (min_bound, max_bound)
    }

    fn check_neighbors(
        node: &Node,
        nodes: &HashMap<Node, Coord>,
        nodes_lookup: &HashMap<Coord, Node>,
        img: &GrayImage,
        adj_list: &mut HashMap<Node, HashSet<Node>>,
        radius: u32,
        threshold: u64,
    ) {
        let node_coords = nodes.get(node).unwrap();
        let (x_min, x_max) = get_bounds(node_coords.0, img.width(), radius);
        let (y_min, y_max) = get_bounds(node_coords.1, img.height(), radius);

        let node_pixel_val = img.get_pixel(node_coords.0, node_coords.1).channels()[0] as i32;

        for y in y_min..y_max {
            for x in x_min..x_max {
                let neighbor_coords = (x as u32, y as u32);
                if &neighbor_coords != node_coords {
                    let neighbor_pixel_val = img
                        .get_pixel(neighbor_coords.0, neighbor_coords.1)
                        .channels()[0] as i32;

                    if ((node_pixel_val - neighbor_pixel_val).abs() as u64) < threshold {
                        let neighbor = nodes_lookup.get(&neighbor_coords).unwrap().to_owned();
                        adj_list.get_mut(&node).unwrap().insert(neighbor);
                        adj_list.get_mut(&neighbor).unwrap().insert(node.to_owned());
                    }
                }
            }
        }
    }

    pub fn build_adj_list(
        file_path: String,
        radius: u32,
        threshold: u64,
    ) -> HashMap<Node, HashSet<Node>> {
        let img = image::open(file_path).unwrap().to_luma();
        
        // TODO: there is a max possible size here for any given radius, maybe should
        // make this with that size
        let mut adj_list: HashMap<Node, HashSet<Node>> = HashMap::new();

        // init nodes
        let mut nodes: HashMap<Node, Coord> = HashMap::new();
        let mut nodes_lookup: HashMap<Coord, Node> = HashMap::new();
        let mut node_id: u64 = 0;

        for pixel in img.enumerate_pixels() {
            nodes.insert(node_id, (pixel.0, pixel.1));
            /////////
            //let pixel_val = img.get_pixel(pixel.0, pixel.1).channels()[0];
            //println!("{}: {}, {} - {}", node_id, pixel.0, pixel.1, pixel_val);
            //////////
            nodes_lookup.insert((pixel.0, pixel.1), node_id);
            adj_list.insert(node_id, HashSet::new());
            node_id += 1;
        }

        for (node, _coord) in nodes.iter() {
            check_neighbors(
                &node,
                &nodes,
                &nodes_lookup,
                &img,
                &mut adj_list,
                radius,
                threshold,
            );
        }

        adj_list
    }
}

#[cfg(test)]
mod tests {
    type Node = u64;
    type Label = u64;
    pub use crate::label_prop::*;
    //use super::label_prop::label_prop::*;
    use super::graphbuilder::*;

    use std::collections::{HashMap, HashSet};

    #[test]
    fn test_get_bounds_0() {
        assert_eq!((0, 8), get_bounds(2, 42, 5));
        assert_eq!((0, 3), get_bounds(0, 3, 2));
    }

    #[test]
    fn test_get_bounds_1() {
        assert_eq!((23, 30), get_bounds(28, 30, 5))
    }

    #[test]
    fn test0() {
        //build_graph("/Users/wigasper/repos/bio-segmenter/36pixel.png".to_owned());
        //let adj_list = build_adj_list("/media/storage/bio-segmenter/36pixel.png".to_owned(), 3, 10);
    }
    
    #[test]
    fn test_get_new_label_0() {
        let mut adjs: HashSet<Node> = HashSet::new();
        adjs.insert(1);
        adjs.insert(2);
        adjs.insert(4);
        adjs.insert(5);

        let mut labs: HashMap<Node, Label> = HashMap::new();
        labs.insert(1, 2);
        labs.insert(2, 2);
        labs.insert(4, 4);
        labs.insert(5, 5);

        let result: Label = 2;
        assert_eq!(result, get_new_label(&adjs, &labs));
    }

    #[test]
    fn test_build_adj_list() {
        let adj_list = build_adj_list("16pixel.png".to_owned(), 2, 30);
        for (key, val) in adj_list.iter() {
            print!("{}: ", key);
            for node in val.iter() {
                print!("{} ", node);
            }
            println!();
        }
    }
}
