{ subnet, components, contracts }:

subnet {
 src = ./.;
 subnet = with components; with contracts; ''
 input => input div(${ui_js_tag}) output => output
 places => input inserter(${ui_js_inserter}) output -> input div()
 '';
}
