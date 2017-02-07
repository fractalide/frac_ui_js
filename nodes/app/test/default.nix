{ subgraph, imsgs, nodes, edges }:

subgraph rec {
  src = ./.;
  imsg = imsgs {
    edges = with edges; [ UiJsCreate PrimText];
  };
  flowscript = with nodes; ''
  '${imsg}.UiJsCreate:(type="div", style=[(key="display", val="flex"), (key="flex-direction", val="column")])~create' -> input td(${flex}) output -> input page(${page})
  '${imsg}.PrimText:(text="initial")~create' -> input edit(${edit}) output -> places[1] td()
  '${imsg}.UiJsCreate:(type="span", text="hello")~create' -> input t(${tag}) output -> places[2] td()
  '';
}
