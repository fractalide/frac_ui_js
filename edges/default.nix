{ buffet }:
let
callPackage = buffet.pkgs.lib.callPackageWith ( buffet // buffet.support);
in
rec {
}
