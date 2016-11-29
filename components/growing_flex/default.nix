{ component, contracts, components }:

component {
  src = ./.;
  contracts = with contracts; [ js_create generic_text fbp_action ];
  depsSha256 = "18z7jbh2hn3pjmgnxw21sa3yj12h23khksw1xilhpaxam3xn779s";
  configurePhase = with components; ''
      substituteInPlace src/lib.rs --replace "ui_js_flex" "${flex}"
  '';
}
