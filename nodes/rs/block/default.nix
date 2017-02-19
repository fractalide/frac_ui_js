{ subgraph, nodes, edges }:

subgraph {
  src = ./.;
  flowscript = with nodes.rs; ''
  input => input div(${tag}) output => output
  places => input inserter(${inserter}) output -> input div()
  '';
}
