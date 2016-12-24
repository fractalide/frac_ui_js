{ subgraph, nodes, edges }:

subgraph {
  src = ./.;
  edges = with edges; [ ];
  flowscript = with nodes; with edges; ''
  input => input div(${ui_js_nodes.tag}) output => output
  places => input inserter(${ui_js_nodes.inserter}) output -> input div()
  '';
}
