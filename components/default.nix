{ pkgs, support, allContracts, allComponents, ... }:
let
callPackage = pkgs.lib.callPackageWith (pkgs // support // allContracts // allComponents);
self = rec { # use one line only to insert a component (utils/new_component.py sorts this list)
  block = callPackage ./ui/js/block {};
  edit = callPackage ./ui/js/edit {};
  edit_contentedited = callPackage ./ui/js/edit/contentedited {};
  edit_create = callPackage ./ui/js/edit/create {};
  edit_keyfilter = callPackage ./ui/js/edit/keyfilter {};
  edit_validate = callPackage ./ui/js/edit/validate {};
  edit_view = callPackage ./ui/js/edit/view {};
  edit_viewer = callPackage ./ui/js/edit/viewer {};
  flex = callPackage ./ui/js/flex {};
  growing_flex = callPackage ./ui/js/growing_flex {};
  inserter = callPackage ./ui/js/inserter {};
  orderer = callPackage ./ui/js/orderer {};
  page = callPackage ./ui/js/page {};
  placeholder = callPackage ./ui/js/placeholder {};
  tag = callPackage ./ui/js/tag {};
  visible = callPackage ./ui/js/visible {};
}; # use one line only to insert a component (utils/new_component.py sorts this list)
in
self
