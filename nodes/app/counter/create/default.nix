{ agent, edges, crates, pkgs }:

agent {
  src = ./.;
  edges = with edges; [ UiAppCounter UiJsCreate ];
  crates = with crates; [ rustfbp capnp ];
  osdeps = with pkgs; [];
}
