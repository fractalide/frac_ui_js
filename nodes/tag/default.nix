{ agent, edges, crates, pkgs}:

agent {
  src = ./.;
  edges = with edges; [ ui_js_create ntup_tuple_tt prim_text prim_bool ];
  crates = with crates; [ rustfbp capnp ];
  osdeps = with pkgs; [];
}
