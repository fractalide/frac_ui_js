{ subnet, contracts, components }:

subnet {
  src = ./.;
  subnet = with contracts; with components; ''
   input => input div(${ui_js_tag}) output => output
   places => places orderer(${ui_js_visible}) output -> input div()
   '';
}
