{ subgraph, imsg, nodes, edges }:
let
  UiJsCreate1 = imsg {
    class = edges.UiJsCreate;
    text = ''(type="div", style=(list=[(key=(text="display"), val=(text="flex")), (key=(text="flex-direction"), val=(text="column"))]))'';
    option = "create";
  };
  PrimText = imsg {
    class = edges.PrimText;
    text = ''(text="initial")'';
    option = "create";
  };
  UiJsCreate2 = imsg {
    class = edges.UiJsCreate;
    text = ''(type="span", text="hello")'';
    option = "create";
  };

in
subgraph rec {
  src = ./.;
  flowscript = with nodes; ''
  '${UiJsCreate1}' -> input td(${flex}) output -> input page(${page})
  '${PrimText}' -> input edit(${edit}) output -> places[1] td()
  '${UiJsCreate2}' -> input t(${tag}) output -> places[2] td()
  '';
}
