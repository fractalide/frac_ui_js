{ agent, edges, crates, pkgs}:

agent {
  src = ./.;
  edges = with edges; [ UiJsCreate NtupTupleTt PrimText PrimBool ];
  crates = with crates; [ rustfbp capnp ];
  osdeps = with pkgs; [];
}
