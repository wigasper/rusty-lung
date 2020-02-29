extern crate image;

use image::*;
use crate::label_prop::*;

use std::collections::{HashMap, HashSet};

type Node = u64;
type Coord = (u32, u32);

pub fn segment_image(
    file_path: &str,
    radius: u32,
    threshold: u64,) {

    let (adj_list, node_coords, img) = build_adj_list(&file_path, &radius, &threshold);
    ////////////////////////////
    //for (node, adjs) in adj_list.iter() {
    //    println!("node: {} num adjs: {}", node, adjs.len());
    //}
    //////////////////
    let communities = label_prop(&adj_list);
    let mut unique_labels: HashSet<u64> = HashSet::new();
    for (_key, val) in communities.iter() {
        unique_labels.insert(val.to_owned());
    }
    let mut comm_id_map = HashMap::new();
    let mut comm_id: u64 = 0;
    for unique_lab in unique_labels.iter() {
        comm_id_map.insert(unique_lab, comm_id);
        comm_id += 1;
    }
    //////////////////////////////////////////////////////
    println!("Found {} communities", unique_labels.len());
    //////////////////////////////////////////////////////
    let mut output = ImageBuffer::<Luma<u8>, Vec<u8>>::new(img.width(), img.height()); 
    
    for (node, coord) in node_coords.iter() {
        let pixel = output.get_pixel_mut(coord.0, coord.1);
        let pixel_val = 255 - (255 / (comm_id_map.get(communities.get(node).unwrap()).unwrap() + 1));
        *pixel = image::Luma([pixel_val as u8]);
    }

    output.save("output.png").unwrap();
}

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

// FOR HACK
fn euc_dist((x1, y1): &Coord, (x2, y2): &Coord) -> f64 {
    let dx: f64 = x2.to_owned() as f64 - x1.to_owned() as f64;
    let dy: f64 = y2.to_owned() as f64 - y1.to_owned() as f64;
    let radicand: f64 = dx * dx + dy * dy;
    radicand.sqrt()
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
    
    // TODO get neighbors with combinatorics instead of this double for loop maybe?
    for y in y_min..y_max {
        for x in x_min..x_max {
            let neighbor_coords = (x as u32, y as u32);
            // HACK
            //let dist = euc_dist(&neighbor_coords, node_coords);

            if &neighbor_coords != node_coords { //&& dist > 2.0 {
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
    file_path: &str,
    radius: &u32,
    threshold: &u64,
) -> (HashMap<Node, HashSet<Node>>, HashMap<Node, Coord>, GrayImage) {
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
            radius.to_owned(),
            threshold.to_owned(),
        );
    }

    (adj_list, nodes, img)
}

