{ subgraph, nodes, edges }:

subgraph {
  src = ./.;
  flowscript = with nodes; with edges; ''
   input => input div(${tag}) output => output
   places => places orderer(${orderer}) output -> input div()
   '';
}
