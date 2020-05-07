extern crate rand;

use rand::seq::SliceRandom;
use rand::thread_rng;

use std::collections::HashMap;

type Node = u32;
type Label = u32;

pub fn get_new_label(adjacents: &[Node], node_labels: &HashMap<Node, Label>) -> Label {
    let labels: Vec<Label> = adjacents
        .iter()
        .map(|&adj| node_labels.get(&adj).unwrap().to_owned())
        .collect();

    let mut labels_counts: HashMap<&Label, u16> = HashMap::new();
    for label in labels.iter() {
        let count = labels_counts.entry(label).or_insert(0);
        *count += 1;
    }

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
    nodes: &[Node],
) {
    for node in nodes.iter() {
        // get the label with the greatest frequency among neighbors
        let adjs = adj_list.get(&node).unwrap();

        if !adjs.is_empty() {
            let new_label: Label = get_new_label(adjs, node_labels);

            if let Some(val) = node_labels.get_mut(node) {
                *val = new_label;
            }
        }
    }
}

pub fn label_prop(adj_list: &HashMap<Node, Vec<Node>>) -> HashMap<Node, Label> {
    let mut node_labels: HashMap<Node, Label> = HashMap::new();

    // init, each node is labeled with its name
    for (node, _) in adj_list.iter() {
        node_labels.insert(node.to_owned(), node.to_owned());
    }

    let mut rng = thread_rng();
    let mut nodes: Vec<Node> = Vec::new();

    let max_iters = 30;

    for (node, _adjs) in adj_list.iter() {
        nodes.push(node.to_owned());
    }

    let mut num_iters = 0;

    // fit loop
    loop {
        // change tracking could occur here. original label prop paper has
        // the algorithm terminate when the labels don't change. that doesn't
        // seem to be possible with the characteristics of this graph though

        // shuffle nodes vec for random updating
        nodes.shuffle(&mut rng);

        // update nodes
        update_nodes(&adj_list, &mut node_labels, &nodes);
        
        num_iters += 1;
        println!("Completed {} iters", num_iters);

        if num_iters == max_iters {
            break;
        }
    }

    node_labels
}
