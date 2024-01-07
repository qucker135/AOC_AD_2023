#!/bin/bash

dot -Tsvg src/graph.dot -Kneato > imgs/graph.svg
dot -Tsvg src/graph_deleted.dot -Kneato > imgs/graph_deleted.svg

