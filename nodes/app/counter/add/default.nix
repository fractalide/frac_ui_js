{ agent, edges, crates, pkgs }:

agent {
  src = ./.;
  edges = with edges; [ UiAppCounter ];
  crates = with crates; [ rustfbp capnp ];
  osdeps = with pkgs; [];
}
