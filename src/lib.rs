pub mod label_prop;

pub mod utils;

#[cfg(test)]
mod tests {
    type Node = u32;
    type Label = u32;
    pub use crate::label_prop::*;
    pub use crate::utils::*;
    //use super::label_prop::label_prop::*;
    //use super::graphbuilder::*;

    use std::collections::{HashMap, HashSet};

    #[test]
    fn label_prop_0() {
        let mut adj_list: HashMap<Node, HashSet<Node>> = HashMap::new();
    }

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
        let mut adjs: Vec<Node> = Vec::new();
        adjs.push(1);
        adjs.push(2);
        adjs.push(4);
        adjs.push(5);

        let mut labs: HashMap<Node, Label> = HashMap::new();
        labs.insert(1, 2);
        labs.insert(2, 2);
        labs.insert(4, 4);
        labs.insert(5, 5);

        let result: Label = 2;
        assert_eq!(result, get_new_label(&adjs, &labs));
    }

    #[test]
    fn test_get_new_label_1() {
        let mut adjs: Vec<Node> = Vec::new();
        adjs.push(1);
        adjs.push(2);
        adjs.push(4);
        adjs.push(5);

        let mut labs: HashMap<Node, Label> = HashMap::new();
        labs.insert(1, 3);
        labs.insert(2, 3);
        labs.insert(4, 3);
        labs.insert(5, 3);

        let result: Label = 3;
        assert_eq!(result, get_new_label(&adjs, &labs));
    }
    #[test]
    fn test_build_adj_list() {
        //let adj_list = build_adj_list("16pixel.png", &2, &30);
        //segment_image("ct_scan.png", 10, 20);
        //for (key, val) in adj_list.iter() {
        //    print!("{}: ", key);
        //   for node in val.iter() {
        //      print!("{} ", node);
        //}
        // println!();
        // }
    }
}
