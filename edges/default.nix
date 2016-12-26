{ buffet }:
let
callPackage = buffet.pkgs.lib.callPackageWith ( buffet // buffet.support);
in
rec {
  js_create = callPackage ./js/create {};
}
