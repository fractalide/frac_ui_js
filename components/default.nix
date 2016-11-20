{ pkgs, support, allContracts, allComponents, ... }:
let
callPackage = pkgs.lib.callPackageWith (pkgs // support // allContracts // allComponents);
self = rec { # use one line only to insert a component (utils/new_component.py sorts this list)
  block = callPackage ./block {};
  edit = callPackage ./edit {};
  edit_contentedited = callPackage ./edit/contentedited {};
  edit_create = callPackage ./edit/create {};
  edit_keyfilter = callPackage ./edit/keyfilter {};
  edit_validate = callPackage ./edit/validate {};
  edit_view = callPackage ./edit/view {};
  edit_viewer = callPackage ./edit/viewer {};
  flex = callPackage ./flex {};
  growing_flex = callPackage ./growing_flex {};
  inserter = callPackage ./inserter {};
  orderer = callPackage ./orderer {};
  page = callPackage ./page {};
  placeholder = callPackage ./placeholder {};
  tag = callPackage ./tag {};
  visible = callPackage ./visible {};
}; # use one line only to insert a component (utils/new_component.py sorts this list)
in
self
