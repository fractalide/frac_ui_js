{ edge, edges }:

edge {
  src = ./.;
  edges =  with edges; [];
  schema = with edges; ''
    struct UiAppCounter {
      value @0 :Int64;
      delta @1 :Int64 = 1;
    }
  '';
}
