{ subgraph, nodes, edges }:

subgraph {
  src = ./.;
  edges = with edges; [ app_counter ];
  flowscript = with nodes; with edges; ''
  counter(${app_counter_card}) output -> input page(${page})
  '${app_counter}:(value=42)~create' -> input counter()
  '';
}
