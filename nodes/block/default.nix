{ subgraph, nodes, edges }:

subgraph {
  src = ./.;
  edges = with edges; [ ];
  flowscript = with nodes; with edges; ''
  input => input div(${tag}) output => output
  places => input inserter(${inserter}) output -> input div()
  '';
}
