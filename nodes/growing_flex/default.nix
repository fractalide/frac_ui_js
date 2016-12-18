{ agent, nodes, edges, crates, pkgs }:

agent {
  src = ./.;
  edges = with edges; [ js_create generic_text fbp_action ];
  crates = with crates; [ rustfbp capnp ];
  osdeps = with pkgs; [];
  configurePhase = with nodes; ''
      substituteInPlace src/lib.rs --replace "ui_js_flex" "${flex}"
  '';
}
