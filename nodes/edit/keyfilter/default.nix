{ agent, edges, crates, pkgs }:

agent {
  src = ./.;
  edges = with edges; [ PrimText NtupTupleTt ];
  crates = with crates; [ rustfbp capnp ];
  osdeps = with pkgs; [];
}
