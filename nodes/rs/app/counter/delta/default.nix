{ agent, edges, mods, pkgs }:

agent {
  src = ./.;
  edges = with edges; [ UiAppCounter PrimText ];
  mods = with mods.rs; [ rustfbp capnp ];
  osdeps = with pkgs; [];
}
