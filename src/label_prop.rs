extern crate rand;

use rand::seq::SliceRandom;
use rand::thread_rng;

use std::collections::HashMap;

type Node = u32;
//type Label = u32;
type Label = u32;

// TODO: is there something here where if a nodes adjacents all have one label we can
// remove it from the pool to be checked? speed things up??
//TODO: the change of adjacents from &HashSet to &Vec does not seem to have helped
pub fn get_new_label(adjacents: &Vec<Node>, node_labels: &HashMap<Node, Label>) -> Label {
    let labels: Vec<Label> = adjacents
        .iter()
        .map(|&adj| node_labels.get(&adj).unwrap().to_owned())
        .collect();

    // This seems to be faster
    let mut labels_counts: HashMap<&Label, u16> = HashMap::new();
    for label in labels.iter() {
        if labels_counts.contains_key(&label) {
            if let Some(count) = labels_counts.get_mut(&label) {
                *count += 1;
            }
        } else {
            labels_counts.insert(label, 1);
        }
    }

    // TODO: this might be good enough for now, but maybe it should
    // be random in the future
    labels_counts
        .iter()
        .max_by_key(|(_, &val)| val)
        .unwrap()
        .0
        .to_owned()
        .to_owned()
}

fn update_nodes(
    adj_list: &HashMap<Node, Vec<Node>>,
    node_labels: &mut HashMap<Node, Label>,
    nodes: &Vec<Node>,
) {
    
    for node in nodes.iter() {
        // get the label with the greatest frequency among neighbors
        let adjs = adj_list.get(&node).unwrap();

        //let mut new_label: Label = node.to_owned();
        if !adjs.is_empty() {
            let new_label: Label = get_new_label(adjs, node_labels);
        

            if let Some(val) = node_labels.get_mut(node) {
                *val = new_label;
            }
        }
    }
}

pub fn label_prop(
    adj_list: &HashMap<Node, Vec<Node>>,
    mut node_labels: HashMap<Node, Label>,
) -> HashMap<Node, Label> {
    let mut rng = thread_rng();
    let mut nodes: Vec<Node> = Vec::new();

    let max_iters = 3;

    for (node, _adjs) in adj_list.iter() {
        nodes.push(node.to_owned());
    }

    let mut num_iters = 0;
    // fit loop - continues until the labels don't change
    loop {
        // TODO: track label change better here, maybe in update_nodes
        // this uses up time for the big clone
        // let's be honest the algorithm doesn't converge ever 
        // at this point so just comment this out
        //let prior_labels = node_labels.clone();

        // shuffle nodes vec for random updating
        // TODO: cutting shuffle saves a little time, maybe there
        // is a faster way to do this
        nodes.shuffle(&mut rng);

        // update nodes
        update_nodes(&adj_list, &mut node_labels, &nodes);

        num_iters += 1;
        println!("completed {} iters", num_iters);
        //if (node_labels == prior_labels) || (num_iters == 3) {
        if num_iters == max_iters {
            break;
        }
    }

    node_labels
}
