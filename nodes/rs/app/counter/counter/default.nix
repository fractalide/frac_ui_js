{ subgraph, imsgs, nodes, edges }:
let
  UiAppCounter = imsg {
    class = edges.UiAppCounter;
    text = ''(value=42)'';
    option = ''create'';
  };
in
subgraph {
  src = ./.;
  flowscript = with nodes.rs; ''
  counter(${app_counter_card}) output -> input page(${page})
  '${UiAppCounter}' -> input counter()
  '';
}
