{ agent, edges, crates, pkgs }:

agent {
  src = ./.;
  edges = with edges; [ generic_text js_create ];
  crates = with crates; [ rustfbp capnp ws ];
  osdeps = with pkgs; [];
}
