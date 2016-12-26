{ buffet }:

let
callPackage = buffet.pkgs.lib.callPackageWith (buffet // buffet.support );
self = rec { # use one line only to insert a component (utils/new_component.py sorts this list)
  app_counter_add = callPackage ./app/counter/add {};
  app_counter_card = callPackage ./app/counter/card {};
  app_counter_counter = callPackage ./app/counter/counter {};
  app_counter_create = callPackage ./app/counter/create {};
  app_counter_delta = callPackage ./app/counter/delta {};
  app_counter_minus = callPackage ./app/counter/minus {};
  app_counter_view = callPackage ./app/counter/view { };
  app_counter_viewer = callPackage ./app/counter/viewer {};
  app_growtest = callPackage ./app/growtest {};
  app_model = callPackage ./app/model {};
  app_test = callPackage ./app/test {};
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
