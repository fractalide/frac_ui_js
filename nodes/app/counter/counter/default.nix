{ subgraph, imsgs, nodes, edges }:

subgraph rec {
  src = ./.;
  imsg = imsgs {
    edges = with edges; [ UiAppCounter ];
  };
  flowscript = with nodes; with edges; ''
  counter(${app_counter_card}) output -> input page(${page})
  '${imsg}.UiAppCounter:(value=42)~create' -> input counter()
  '';
}
