# rusty-lung
Biomedically motivated community-detection-based image segmentation

This small project utilizes the label propagation community detection algorithm in order 
to segment images. The initial abstraction process involves abstracting pixels to nodes and 
creating edges between nearby nodes that have similar luminosities. 
In order to combat over-segmentation, the resulting communities from the initial round of 
label propagation are then abstracted into the nodes for further rounds. The current results 
are not super great.

Requires [Rust](https://www.rust-lang.org/tools/install).

Installation is simple:

```bash 
git clone https://github.com/wigasper/rusty-lung
cd rusty-lung
cargo build --release
```
