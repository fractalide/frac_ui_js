{ component, contracts, crates, pkgs }:

component {
  src = ./.;
  contracts = with contracts; [ js_create ];
  crates = with crates; [];
  osdeps = with pkgs; [];
  depsSha256 = "00j00vqa1nv29dy6vnq03j7xq1801lrbx7kp4faxahbpv7snmgrx";
}
