pub mod label_prop;

pub mod utils;

#[cfg(test)]
mod tests {
    type Node = u32;
    type Label = u32;
    type Coord = (u32, u32);

    pub use crate::label_prop::*;
    pub use crate::utils::*;

    use std::collections::HashMap;

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
    fn test_get_border_coords_0() {
        let member_coords = vec![(0, 0), (1, 0), (2, 0)];
        let internal_coords: Vec<Coord> = Vec::new();
        let border_coords = vec![(0, 0), (1, 0), (2, 0)];

        assert_eq!((border_coords, internal_coords), get_border_coords(&member_coords));
    }

    #[test]
    fn test_get_border_coords_1() {
        let member_coords = vec![(0, 0), (0, 1), (0, 2)];
        let internal_coords: Vec<Coord> = Vec::new();
        let border_coords = vec![(0, 0), (0, 2), (0, 1)];

        assert_eq!((border_coords, internal_coords), get_border_coords(&member_coords));
    }

}
