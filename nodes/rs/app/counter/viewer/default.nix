{ agent, edges, mods, pkgs }:

agent {
  src = ./.;
  edges = with edges; [ PrimText UiAppCounter KvKeyTValT ];
  mods = with mods.rs; [ rustfbp capnp ];
  osdeps = with pkgs; [];
}
