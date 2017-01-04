{ agent, edges, crates, pkgs }:

agent {
  src = ./.;
  edges = with edges; [ prim_text ui_app_counter kv_key_t_val_t ];
  crates = with crates; [ rustfbp capnp ];
  osdeps = with pkgs; [];
}
