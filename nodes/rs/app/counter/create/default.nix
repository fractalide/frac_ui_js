{ agent, edges, mods, pkgs }:

agent {
  src = ./.;
  edges = with edges; [ UiAppCounter UiJsCreate ];
  mods = with mods.rs; [ rustfbp capnp ];
  osdeps = with pkgs; [];
}
