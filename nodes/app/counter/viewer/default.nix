{ agent, edges, crates, pkgs }:

agent {
  src = ./.;
  edges = with edges; [ prim_text app_counter key_t_val_t ];
  crates = with crates; [ rustfbp capnp ];
  osdeps = with pkgs; [];
}
