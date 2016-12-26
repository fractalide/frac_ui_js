{ agent, edges, crates, pkgs}:

agent {
  src = ./.;
  edges = with edges; [ js_create ntuple_tuple_tt prim_text prim_bool ];
  crates = with crates; [ rustfbp capnp ];
  osdeps = with pkgs; [];
}
