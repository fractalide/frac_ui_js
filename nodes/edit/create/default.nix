{ agent, edges, crates, pkgs }:

agent {
  src = ./.;
  edges = with edges; [ prim_text js_create ];
  crates = with crates; [ rustfbp capnp ];
  osdeps = with pkgs; [];
}
