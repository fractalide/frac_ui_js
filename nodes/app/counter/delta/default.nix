{ agent, edges, crates, pkgs }:

agent {
  src = ./.;
  edges = with edges; [ UiAppCounter PrimText ];
  crates = with crates; [ rustfbp capnp ];
  osdeps = with pkgs; [];
}
