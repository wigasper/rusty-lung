pub mod label_prop {
    extern crate rand;

    use rand::thread_rng;
    use rand::seq::SliceRandom;

    use std::collections::{HashMap, HashSet};

    type Node = u64;
    type Label = u64;

    fn get_new_label (adjacents: &HashSet<Node>, 
                      node_labels: &HashMap<Node, Label>) -> Label {

    }

    fn update_nodes (adj_list: &HashMap<Node, HashSet<Node>>,
                     node_labels: &mut HashMap<Node, Label>,
                     nodes: &Vec<Node>) {
        
        // for each node
        for node in nodes.iter() {
            // get the label with the greatest frequency among neighbors
            let adjs = adj_list.get(&node).unwrap();
            let new_label = get_new_label(adjs, node_labels);
            
            if let Some(val) = node_labels.get_mut(node) {
                *val = new_label;
            }
        }

    }

    pub fn label_prop(adj_list: &HashMap<Node, HashSet<Node>>) {
        let mut node_labels: HashMap<Node, Label> = HashMap::new();
        let mut rng = thread_rng();
        let mut nodes: Vec<Node> = Vec::new();

        // init - each node gets its own label
        for (node, _adjs) in adj_list.iter() {
            node_labels.insert(node, node);
            nodes.push(node);
        }
        
        // fit loop - continues until the labels don't change
        loop {
            let prior_labels = node_labels.clone();

            // shuffle nodes vec for random updating
            nodes.shuffle(&mut rng);

            // update nodes
            
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_lab_prop_0() {

    }

}
