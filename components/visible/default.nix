{ component, contracts, crates, pkgs }:

component {
  src = ./.;
  contracts = with contracts; [ js_create ];
  crates = with crates; [];
  osdeps = with pkgs; [];
  depsSha256 = "18z7jbh2hn3pjmgnxw21sa3yj12h23khksw1xilhpaxam3xn779s";
}
