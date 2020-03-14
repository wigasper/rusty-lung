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

    let img = image::open(file_path).unwrap().to_luma();

    let nodes: HashMap<Label, Vec<Coord>> = init_abstraction(&img);

    let mut adj_list = build_adj_list(&nodes, &img, &radius, &threshold);

    // communities is a hashmap of node: label
    let mut communities = label_prop(&adj_list);

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

    //////////////////////////////////////////////////////////
    // made a wonderful first run, what to do now?
    
    // re-abstract, each community needs to become a node.
    // to do that, all the vecs from all the nodes in each community need to be combined
    let new_nodes: HashMap<Label, Vec<Coord>> = comm_abstraction(&nodes, &community_members);
    let new_radius: u32 = ((radius as f32) * 10.0) as u32;
    adj_list = build_adj_list(&new_nodes, &img, &new_radius, &threshold);
    communities = label_prop(&adj_list);
    community_members = HashMap::new();
    for (_key, val) in communities.iter() {
        community_members.insert(val.to_owned(), Vec::new());
    }
    for (key, val) in communities.iter() {
        if let Some(node_vec) = community_members.get_mut(val) {
            node_vec.push(key.to_owned());
        }
    }
    println!("Found {} communities", community_members.len());
    //////////////////////////////////////////////////////////
    let mut output = ImageBuffer::<Luma<u8>, Vec<u8>>::new(img.width(), img.height());

    for (comm, members) in community_members.iter() {
        // need to get all the coords for a particular community that is comprised of nodes
        let mut member_coords: Vec<Coord> = Vec::new();

        for node in members.iter() {
            for coord in nodes.get(node).unwrap().iter() {
                member_coords.push(coord.to_owned());
            }
        }

        let (border_pixels, internal_pixels) = get_border_coords(&member_coords);

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

pub fn comm_abstraction(prior_nodes: &HashMap<Label, Vec<Coord>>, 
                        communities: &HashMap<Label, Vec<Node>>) -> HashMap<Label, Vec<Coord>> {
    let mut nodes_out: HashMap<Label, Vec<Coord>> = HashMap::new();

    for (community, nodes) in communities.iter() {
        //nodes.insert(community.to_owned(), Vec::new());
        
        let mut coords: Vec<Coord> = Vec::new();
        for node in nodes.iter() {
            for coord in prior_nodes.get(node).unwrap().iter() {
                // TODO: to_owned here too much??
                coords.push(coord.to_owned());
            }
        }
        nodes_out.insert(community.to_owned(), coords);
    }

    nodes_out
}

pub fn init_abstraction(img: &GrayImage) -> HashMap<Label, Vec<Coord>> {
    let mut nodes: HashMap<Label, Vec<Coord>> = HashMap::new();
    let mut current_label: Label = 0;
    
    let num_pixels = img.width() * img.height();
    for pixel in img.enumerate_pixels() {
        nodes.insert(current_label, vec![(pixel.0, pixel.1)]);
        current_label += 1;
    }

    nodes
}

// TODO: refine logic here to deal with one pixel communities?
pub fn get_border_coords(member_coords: &Vec<Coord>) -> (Vec<Coord>, Vec<Coord>) {
    let mut border_coords: Vec<Coord> = Vec::new();
    let mut internal_coords: Vec<Coord> = Vec::new();

    let mut y_vals: Vec<u32> = member_coords.iter().map(|coord| coord.1).collect();
    y_vals.sort();
    y_vals.dedup();

    if y_vals.len() > 2 {
        for coord in member_coords.iter() {
            if coord.1 == y_vals[0] || coord.1 == y_vals[y_vals.len() - 1] {
                border_coords.push(coord.to_owned());
            }
        }

        // TODO: this first one seems bad, moving an entire vec
        y_vals.remove(0);
        y_vals.remove(y_vals.len() - 1);

        for y in y_vals.iter() {
            let mut x_vals: Vec<u32> = member_coords
                .iter()
                .filter(|&coord| &coord.1 == y)
                .map(|&coord| coord.0)
                .collect();
            x_vals.sort();
            border_coords.push((x_vals[0], y.to_owned()));
            border_coords.push((x_vals[x_vals.len() - 1], y.to_owned()));

            if x_vals.len() > 2 {
                x_vals.remove(0);
                x_vals.remove(x_vals.len() - 1);

                for x in x_vals.iter() {
                    let coord_above: Coord = (x.to_owned(), y.to_owned() + 1);
                    let coord_below: Coord = (x.to_owned(), y.to_owned() - 1);
                    if member_coords.contains(&coord_above) && member_coords.contains(&coord_below)
                    {
                        internal_coords.push((x.to_owned(), y.to_owned()));
                    } else {
                        border_coords.push((x.to_owned(), y.to_owned()));
                    }
                }
            }
        }
    } else {
        border_coords = member_coords.to_owned();
    }

    (border_coords, internal_coords)
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

pub fn get_group_means(pixels: &Vec<Coord>) -> (u32, u32) {
    let x_vals: Vec<f32> = pixels.iter().map(|pix| pix.0 as f32).collect();
    let x_sum: f32 = x_vals.iter().sum();
    let x_mean: u32 = (x_sum / x_vals.len() as f32) as u32;

    let y_vals: Vec<f32> = pixels.iter().map(|pix| pix.1 as f32).collect();
    let y_sum: f32 = y_vals.iter().sum();
    let y_mean: u32 = (y_sum / x_vals.len() as f32) as u32;

    (x_mean, y_mean)
}

pub fn check_neighbors(
    node_centers: &HashMap<Node, Coord>,
    node_centers_lookup: &HashMap<Coord, Node>,
    node_values: &HashMap<Node, u8>,
    adj_list: &mut HashMap<Node, Vec<Node>>,
    radius: u32,
    threshold: u8,
    dimensions: (u32, u32),
) {
    // for each node identify other nearby nodes based on their centers
    for (node, center) in node_centers.iter() {
        let (x_min, x_max) = get_bounds(center.0, dimensions.0, radius);
        let (y_min, y_max) = get_bounds(center.1, dimensions.1, radius);

        let node_val: i16 = node_values.get(node).unwrap().to_owned() as i16;

        for x in x_min..x_max {
            for y in y_min..y_max {
                let putative_neighbor: Coord = (x as u32, y as u32);
                if &putative_neighbor != center
                    && node_centers_lookup.contains_key(&putative_neighbor)
                {
                    let neighbor: Node = node_centers_lookup.get(&putative_neighbor).unwrap().to_owned();
                    let neighbor_val: i16 = node_values.get(&neighbor).unwrap().to_owned() as i16;
                    let delta: u8 = (node_val - neighbor_val).abs() as u8;

                    if delta < threshold {
                        adj_list.get_mut(node).unwrap().push(neighbor);
                    }
                }
            }
        }
    }
}

pub fn build_adj_list(
    nodes: &HashMap<Label, Vec<Coord>>,
    img: &GrayImage,
    radius: &u32,
    threshold: &u8,
) -> HashMap<Node, Vec<Node>> {
    // adjacency list, end goal here
    let mut adj_list: HashMap<Node, Vec<Node>> = HashMap::new();

    // center coordinates
    let mut node_centers: HashMap<Node, Coord> = HashMap::new();
    // lookup nodes for any given center coord, will use this later
    let mut node_centers_lookup: HashMap<Coord, Node> = HashMap::new();

    // get the center coord for each node
    for (node, pixels) in nodes.iter() {
        let center_coords = get_group_means(pixels);
        node_centers.insert(node.to_owned(), center_coords);
        node_centers_lookup.insert(center_coords, node.to_owned());
        // init adj_list
        adj_list.insert(node.to_owned(), Vec::new());
    }
    
    // each node needs to get a luma value for comparison to other nodes
    let mut node_values: HashMap<Node, u8> = HashMap::new();

    // get the mean luma value for each node
    // TODO would make sense to have this be its own function
    for (node, pixels) in nodes.iter() {
        let pixel_vals: Vec<f32> = pixels
            .iter()
            .map(|pix| img.get_pixel(pix.0, pix.1).channels()[0] as f32)
            .collect();
        let sum: f32 = pixel_vals.iter().sum();
        let mean: u8 = (sum / pixel_vals.len() as f32) as u8;
        
        node_values.insert(node.to_owned(), mean);
    }

    // now need to check neighbors for the adjacency list
    check_neighbors(
        &node_centers,
        &node_centers_lookup,
        &node_values,
        &mut adj_list,
        radius.to_owned(),
        threshold.to_owned(),
        img.dimensions(),
    );

    adj_list
}
