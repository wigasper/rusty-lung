pub mod graphbuilder {
    extern crate image;
    extern crate petgraph;

    use image::*;
    use petgraph::graph::{Graph};

    use std::collections::HashMap;

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
                       g: &mut Graph<u64, ()>, radius: u32, threshold:u64) {
        let node_coords = nodes.get(node).unwrap();
        let (x_min, x_max) = get_bounds(node_coords.0, img.width(), radius);
        let (y_min, y_max) = get_bounds(node_coords.1, img.height(), radius);
        
        let node_pixel = img.get_pixel(node_coords.0, node_coords.1);
        print!("{} {} ", node_coords.0, node_coords.1);
        println!("{}", node_pixel.channels()[0]);
        for col in y_min..y_max {
            for row in x_min..x_max {
                let neighbor_coords = (col as u32, row as u32);
                if &neighbor_coords != node_coords {
                    let neighbor_pixel = img.get_pixel(neighbor_coords.0, neighbor_coords.1);
                    // if node_pixel.channels()[0] > neighbor_pixel.channels()[0]        
                }
            }
        }

    }

    // TODO: get rid of this graph abstraction maybe - if using label prop
    // only need an adjacency list
    pub fn build_graph(file_path: String) {
        let img = image::open(file_path).unwrap().to_luma();
        
        let mut g: Graph<u64, ()> = Graph::new();

        // init nodes
        // TODO: abstract this?
        let mut nodes: HashMap<u64, Coord> = HashMap::new();
        let mut nodes_lookup: HashMap<Coord, u64> = HashMap::new();
        let mut node_id: u64 = 0;

        for pixel in img.enumerate_pixels() {
            nodes.insert(node_id, (pixel.0, pixel.1));
            nodes_lookup.insert((pixel.0, pixel.1), node_id);
            g.add_node(node_id);
            node_id += 1;
        }

        for (node, _coord) in nodes.iter() {
            check_neighbors(&node, &nodes, &nodes_lookup, &img, 
                            &mut g, 3, 10); 
        }
        //for (key, val) in nodes.iter() {
        //    println!("{}: {}, {}", key, val.0, val.1);
        //}
        
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
        build_graph("/media/storage/bio-segmenter/36pixel.png".to_owned())
    }
}
