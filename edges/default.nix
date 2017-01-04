{ buffet }:
let
callPackage = buffet.pkgs.lib.callPackageWith ( buffet // buffet.support);
in
rec {
  ui_app_counter = callPackage ./ui/app/counter {};
  ui_js_create = callPackage ./ui/js/create {};
}
