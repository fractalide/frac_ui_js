{ agent, edges, crates, pkgs }:

agent {
  src = ./.;
  edges = with edges; [ prim_text ntuple_tuple_tt ];
  crates = with crates; [ rustfbp capnp ];
  osdeps = with pkgs; [];
}
