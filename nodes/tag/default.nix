{ agent, edges, crates, pkgs}:

agent {
  src = ./.;
  edges = with edges; [ js_create generic_tuple_text generic_text generic_bool ];
  crates = with crates; [];
  osdeps = with pkgs; [];
  depsSha256 = "1g77ji2kqxacydg8mnzg4a9i5k7x6zgvvfgw6f0m26j0267883lp";
}
