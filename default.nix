{ fractalide ? import <fractalide> {}
  , pkgs ? fractalide.pkgs
  , support ? fractalide.support
  , contracts ? fractalide.contracts
  , components ? fractalide.components}:
let
  publicComponentOrSubnet = allComponents.ui_js;
  exeSubnet = allComponents.ui_js;
  allContracts = contracts // import ./contracts {inherit pkgs support allContracts;};
  allComponents = components // import ./components {inherit pkgs support allContracts allComponents;};
  result = if fractalide == null
    then publicComponentOrSubnet
    else import (<fractalide> + "/support/vm/") {inherit pkgs support;
      contracts = allContracts;
      components = allComponents;
      exeSubnet = exeSubnet;};
in
  result
