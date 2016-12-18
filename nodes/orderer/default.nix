{ agent, edges, crates, pkgs }:

agent {
  src = ./.;
  edges = with edges; [ js_create ];
  crates = with crates; [ rustfbp capnp ];
  osdeps = with pkgs; [];
  depsSha256 = "00j00vqa1nv29dy6vnq03j7xq1801lrbx7kp4faxahbpv7snmgrx";
}
