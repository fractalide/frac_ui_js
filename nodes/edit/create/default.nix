{ agent, edges, crates, pkgs }:

agent {
  src = ./.;
  edges = with edges; [ PrimText UiJsCreate ];
  crates = with crates; [ rustfbp capnp ];
  osdeps = with pkgs; [];
}
