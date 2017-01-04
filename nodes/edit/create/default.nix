{ agent, edges, crates, pkgs }:

agent {
  src = ./.;
  edges = with edges; [ prim_text ui_js_create ];
  crates = with crates; [ rustfbp capnp ];
  osdeps = with pkgs; [];
}
