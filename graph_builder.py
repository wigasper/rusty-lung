#!/usr/bin/env python3

from PIL import Image
import numpy as np

def check_neighbors(node, nodes, nodes_lookup, edges, img, radius, threshold):
    edges_out = []
    node_coord = nodes[node]
    row_l_bound = node_coord[0] - radius if node_coord[0] - radius >= 0 else 0
    row_h_bound = node_coord[0] + radius if node_coord[0] + radius < img.shape[0] else img.shape[0]
    
    col_l_bound = node_coord[1] - radius if node_coord[1] - radius >= 0 else 0
    col_h_bound = node_coord[1] + radius if node_coord[1] + radius < img.shape[1] else img.shape[1]

    for row_idx in range(row_l_bound, row_h_bound):
        for col_idx in range(col_l_bound, col_h_bound):
            neighbor_coord = (row_idx, col_idx)
            if neighbor_coord != node_coord:
                if abs(int(img[node_coord[0]][node_coord[1]][0]) - int(img[row_idx][col_idx][0])) < threshold:
                    if (nodes_lookup[neighbor_coord], node) not in edges:
                        edges_out.append((node, nodes_lookup[neighbor_coord]))

    return edges_out 

def build_graph(file_path, radius, threshold):
    temp_img = Image.open(file_path).convert("LA")
    img = np.array(temp_img)

    edges = set()

    # adding this map initially because it may be useful
    # later, if using other techniques
    nodes = {}
    node = 0
    for row_idx in range(img.shape[0]):
        for col_idx in range(img.shape[1]):
            nodes[node] = (row_idx, col_idx)
            node += 1

    nodes_lookup = {val: key for key, val in nodes.items()}
    
    # for every pixel, check for similar neighbors
    for node in nodes:
        result = check_neighbors(node, nodes, nodes_lookup, edges, img, radius, threshold)
        for edge in result:
            edges.add(edge)

    return edges
