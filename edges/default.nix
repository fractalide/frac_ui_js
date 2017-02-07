{ buffet }:
let
callPackage = buffet.pkgs.lib.callPackageWith ( buffet // buffet.support);
in
rec {
  UiAppCounter = callPackage ./ui/app/counter {};
  UiJsCreate = callPackage ./ui/js/create {};
}
