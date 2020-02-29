//p//ub m//od la//belprop {
extern crate rand;

use rand::thread_rng;
use rand::seq::SliceRandom;

use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

type Node = u64;
type Label = u64;

pub fn get_new_label (adjacents: &HashSet<Node>, 
                  node_labels: &HashMap<Node, Label>) -> Label {
    // TODO: can do lines 16-18 without creating this labels var??
    let labels: Vec<Label> = adjacents.iter()
                .map(|&adj| {node_labels.get(&adj).unwrap().to_owned()}).collect();
    let labels_set: HashSet<Label> = HashSet::from_iter(labels.to_owned());
    let mut labels_counts: HashMap<Label, u64> = HashMap::new();

    for label in labels_set.iter() {
        let num_occurrences = labels.iter().filter(|&lab| lab == label).count() as u64;
        labels_counts.insert(label.to_owned(), num_occurrences);
    }
    
    // TODO: this might be good enough for now, but maybe it should
    // be random in the future
    labels_counts.iter().max_by_key(|(_, &val)| {val}).unwrap().0.to_owned()
}

fn update_nodes(adj_list: &HashMap<Node, HashSet<Node>>,
                 node_labels: &mut HashMap<Node, Label>,
                 nodes: &Vec<Node>) {
    // for each node
    for node in nodes.iter() {
        // get the label with the greatest frequency among neighbors
        let adjs = adj_list.get(&node).unwrap();
        
        let mut new_label: Label = node.to_owned();
        ////////////////
        println!("prior label: {}", node_labels.get(node).unwrap());
        if !adjs.is_empty() {
            new_label = get_new_label(adjs, node_labels);
        }

        if let Some(val) = node_labels.get_mut(node) {
            *val = new_label;
        }
        /////////////
        println!("new label: {}", node_labels.get(node).unwrap());
    }
    
}

pub fn label_prop(adj_list: &HashMap<Node, HashSet<Node>>) -> HashMap<Node, Label>{
    let mut node_labels: HashMap<Node, Label> = HashMap::new();
    let mut rng = thread_rng();
    let mut nodes: Vec<Node> = Vec::new();

    // init - each node gets its own label
    // TODO: dereference node vars here???
    for (node, _adjs) in adj_list.iter() {
        node_labels.insert(node.to_owned(), node.to_owned());
        nodes.push(node.to_owned());
    }
    let mut num_iters = 0;    
    // fit loop - continues until the labels don't change
    loop {
        let prior_labels = node_labels.clone();

        // shuffle nodes vec for random updating
        nodes.shuffle(&mut rng);

        // update nodes
        update_nodes(&adj_list, &mut node_labels, &nodes);
        
        num_iters += 1;
        if (node_labels == prior_labels) || (num_iters == 1000) {
            break;
        }
    }

    node_labels
}
//}

