{ subnet, components, contracts }:

subnet {
  src = ./.;
  flowscript = with components; with contracts; ''
  input => input div(${ui_js_components.tag}) output => output
  places => input inserter(${ui_js_components.inserter}) output -> input div()
  '';
}
