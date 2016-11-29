{ subnet, contracts, components }:

subnet {
  src = ./.;
  flowscript = with contracts; with components; ''
   input => input div(${ui_js_components.tag}) output => output
   places => places orderer(${ui_js_components.visible}) output -> input div()
   '';
}
