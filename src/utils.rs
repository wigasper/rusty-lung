extern crate image;

use crate::label_prop::*;
use image::*;

use std::collections::{HashMap, HashSet};
// use std::iter::FromIterator;

type Node = u32;
type Label = u32;
// change Label to u8 to test speed
//type Label = u8;
type Coord = (u32, u32);

pub fn segment_image(file_path: &str, out_path: &str, radius: u32, threshold: u8) {
    // using this initially produces a very interesting result:
    //let (adj_list, node_coords, node_labels, img) = build_adj_list(&file_path, &1, &5);

    let (adj_list, node_coords, node_labels, img) = build_adj_list(&file_path, &radius, &threshold);

    // communities is a hashmap of node: label
    let communities = label_prop(&adj_list, node_labels);

    // need to reverse communities here
    let mut community_members: HashMap<Label, Vec<Node>> = HashMap::new();

    for (_key, val) in communities.iter() {
        community_members.insert(val.to_owned(), Vec::new());
    }

    for (key, val) in communities.iter() {
        if let Some(node_vec) = community_members.get_mut(val) {
            node_vec.push(key.to_owned());
        }
    }

    println!("Found {} communities", community_members.len());

    let mut output = ImageBuffer::<Luma<u8>, Vec<u8>>::new(img.width(), img.height());

    for (comm, nodes) in community_members.iter() {
        let (border_pixels, internal_pixels) = get_border_coords(nodes, &node_coords);

        for border_pixel in border_pixels.iter() {
            let pixel = output.get_pixel_mut(border_pixel.0, border_pixel.1);
            let pixel_val: u8 = 0;
            *pixel = image::Luma([pixel_val]);
        }

        for internal_pixel in internal_pixels.iter() {
            let pixel = output.get_pixel_mut(internal_pixel.0, internal_pixel.1);
            let pixel_val: u8 = 240;
            *pixel = image::Luma([pixel_val]);
        }
    }

    output.save(out_path).unwrap();
}

// TODO: need logic here to deal with one pixel communities
pub fn get_border_coords(nodes: &Vec<Node>, node_coords: &HashMap<Node, Coord>) -> (Vec<Coord>, Vec<Coord>) {
    let mut border_coords: Vec<Coord> = Vec::new();
    let mut internal_coords: Vec<Coord> = Vec::new();

    let coord_list: Vec<Coord> = nodes
        .iter()
        .map(|&node| node_coords.get(&node).unwrap().to_owned())
        .collect();

    let mut y_vals: Vec<u32> = coord_list.iter().map(|coord| coord.1).collect();
    y_vals.sort();
    y_vals.dedup();

    if y_vals.len() > 2 {
        for coord in coord_list.iter() {
            if coord.1 == y_vals[0] || coord.1 == y_vals[y_vals.len() - 1] {
                border_coords.push(coord.to_owned());
            }
        }
    

        // TODO: this first one seems bad, moving an entire vec
        y_vals.remove(0);
        y_vals.remove(y_vals.len() - 1);

        for y in y_vals.iter() {
            let mut x_vals: Vec<u32> = coord_list
                .iter()
                .filter(|&coord| &coord.1 == y)
                .map(|&coord| coord.0)
                .collect();
            x_vals.sort();
            border_coords.push((x_vals[0], y.to_owned()));
            border_coords.push((x_vals[x_vals.len() - 1], y.to_owned()));
            
            if x_vals.len() > 2 {
                x_vals.remove(0);
                x_vals.remove(x_vals.len()-1);
                
                for x in x_vals.iter() {
                    let coord_above: Coord = (x.to_owned(), y.to_owned() + 1);
                    let coord_below: Coord = (x.to_owned(), y.to_owned() - 1);
                    if coord_list.contains(&coord_above) || coord_list.contains(&coord_below) {
                        internal_coords.push((x.to_owned(), y.to_owned()));
                    } else {
                        border_coords.push((x.to_owned(), y.to_owned()));
                    }
                }
            }
        }
    } else {
        border_coords = coord_list;
    }
    
    (border_coords, internal_coords)
}

pub fn init_abstraction(file_path: &str) {
    // do nothing
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

// FOR testing
//fn euc_dist((x1, y1): &Coord, (x2, y2): &Coord) -> f64 {
//    let dx: f64 = x2.to_owned() as f64 - x1.to_owned() as f64;
//    let dy: f64 = y2.to_owned() as f64 - y1.to_owned() as f64;
//    let radicand: f64 = dx * dx + dy * dy;
//    radicand.sqrt()
//}

fn check_neighbors(
    node: &Node,
    nodes: &HashMap<Node, Coord>,
    nodes_lookup: &HashMap<Coord, Node>,
    // TODO: delete node_labels if unneeded in the future
    _node_labels: &mut HashMap<Node, Label>,
    img: &GrayImage,
    adj_list: &mut HashMap<Node, Vec<Node>>,
    radius: u32,
    threshold: u8,
) {
    let node_coords = nodes.get(node).unwrap();
    let (x_min, x_max) = get_bounds(node_coords.0, img.width(), radius);
    let (y_min, y_max) = get_bounds(node_coords.1, img.height(), radius);

    let node_pixel_val = img.get_pixel(node_coords.0, node_coords.1).channels()[0] as i32;

    // new test, init pixels with their value
    // moved to build_adj_list
    //if let Some(lab) = node_labels.get_mut(node) {
    //    *lab = img.get_pixel(node_coords.0, node_coords.1).channels()[0] as u32;
    //}
    //println!("label for {}: {}", node, node_labels.get(node).unwrap());
    // TODO get neighbors with combinatorics instead of this double for loop maybe?
    for y in y_min..y_max {
        for x in x_min..x_max {
            let neighbor_coords = (x as u32, y as u32);
            //let dist = euc_dist(&neighbor_coords, node_coords);

            if &neighbor_coords != node_coords {
                //&& dist > 2.0 {
                let neighbor_pixel_val = img
                    .get_pixel(neighbor_coords.0, neighbor_coords.1)
                    .channels()[0] as i32;

                let d_pixel = (node_pixel_val - neighbor_pixel_val).abs() as u8;
                if d_pixel < threshold {
                    let neighbor = nodes_lookup.get(&neighbor_coords).unwrap().to_owned();
                    adj_list.get_mut(&node).unwrap().push(neighbor);
                    adj_list.get_mut(&neighbor).unwrap().push(node.to_owned());

                    // TESTING add multiple edges if neighboring pixels are the same
                    // may speed up convergence
                    if d_pixel == 0 {
                        for _ in 0..2 {
                            adj_list.get_mut(&node).unwrap().push(neighbor);
                            adj_list.get_mut(&neighbor).unwrap().push(node.to_owned());
                        }
                    }
                    // Set neighbor to label if they have the exact same pixel val
                    //if d_pixel == 0 {
                    //    let node_lab = node_labels.get(node).unwrap().to_owned();
                    //    if let Some(lab) = node_labels.get_mut(&neighbor) {
                    //        *lab = node_lab;
                    //    }
                    //}
                }
            }
        }
    }
}

//pub fn build_adj_list(communities: HashMap<Label, Vec<Node>>, node_coords: HashMap<Node, Coord>) {
    
//}
// rebuild this to take nodes
pub fn build_adj_list(
    file_path: &str,
    radius: &u32,
    threshold: &u8,
) -> (
    HashMap<Node, Vec<Node>>,
    HashMap<Node, Coord>,
    HashMap<Node, Label>,
    GrayImage,
) {
    let img = image::open(file_path).unwrap().to_luma();

    // TODO: there is a max possible size here for any given radius, maybe should
    // make this with that size
    let mut adj_list: HashMap<Node, Vec<Node>> = HashMap::new();

    // init nodes
    let mut nodes: HashMap<Node, Coord> = HashMap::new();
    let mut nodes_lookup: HashMap<Coord, Node> = HashMap::new();
    let mut node_id: u32 = 0;
    let mut node_labels: HashMap<Node, Label> = HashMap::new();

    // TODO: test label init where each node gets its pixel value as a label
    for pixel in img.enumerate_pixels() {
        nodes.insert(node_id, (pixel.0, pixel.1));
        nodes_lookup.insert((pixel.0, pixel.1), node_id);
        // new label init, give each node its pixel val
        // TODO: try this as u8 if good
        //let label = img.get_pixel(pixel.0, pixel.1).channels()[0] as u32;
        let label = node_id;
        node_labels.insert(node_id, label);
        adj_list.insert(node_id, Vec::new());
        node_id += 1;
    }

    for (node, _coord) in nodes.iter() {
        check_neighbors(
            &node,
            &nodes,
            &nodes_lookup,
            &mut node_labels,
            &img,
            &mut adj_list,
            radius.to_owned(),
            threshold.to_owned(),
        );
    }

    //let mut adj_list_out: HashMap<Node, Vec<Node>> = HashMap::new();
    //for (key, val) in adj_list.iter() {
    //    let adjs: Vec<Node> = Vec::from_iter(val.to_owned());
    //    adj_list_out.insert(key.to_owned(), adjs);
    //}

    (adj_list, nodes, node_labels, img)
}
