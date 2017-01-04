{ agent, edges, crates, pkgs }:

agent {
  src = ./.;
  edges = with edges; [ ui_app_counter ui_js_create];
  crates = with crates; [ rustfbp capnp ];
  osdeps = with pkgs; [];
}
