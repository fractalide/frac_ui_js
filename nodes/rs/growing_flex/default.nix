{ agent, nodes, edges, mods, pkgs }:

agent {
  src = ./.;
  edges = with edges; [ CoreAction ];
  mods = with mods.rs; [ rustfbp capnp ];
  osdeps = with pkgs; [];
  configurePhase = with nodes.rs; ''
      substituteInPlace lib.rs --replace "ui_js_flex" "${flex}"
  '';
}
