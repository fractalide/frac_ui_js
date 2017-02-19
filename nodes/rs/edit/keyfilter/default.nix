{ agent, edges, mods, pkgs }:

agent {
  src = ./.;
  edges = with edges; [ PrimText NtupTupleTt ];
  mods = with mods.rs; [ rustfbp capnp ];
  osdeps = with pkgs; [];
}
