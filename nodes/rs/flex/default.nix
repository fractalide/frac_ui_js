{ subgraph, nodes, edges }:

subgraph {
  src = ./.;
  flowscript = with nodes.rs; ''
   input => input div(${tag}) output => output
   places => places orderer(${orderer}) output -> input div()
   '';
}
