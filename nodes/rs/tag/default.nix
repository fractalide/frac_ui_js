{ agent, edges, mods, pkgs}:

agent {
  src = ./.;
  edges = with edges; [ UiJsCreate NtupTupleTt PrimText PrimBool ];
  mods = with mods.rs; [ rustfbp capnp ];
  osdeps = with pkgs; [];
}
