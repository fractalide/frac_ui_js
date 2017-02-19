{ agent, edges, mods, pkgs }:

agent {
  src = ./.;
  edges = with edges; [ UiJsCreate ];
  mods = with mods.rs; [ rustfbp capnp ];
  osdeps = with pkgs; [];
}
