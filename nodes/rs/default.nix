{ buffet }:

# Please refer to section 2.6 namely Evolution of Public Contracts
# of the Collective Code Construction Contract in CONTRIBUTING.md
let
  callPackage = buffet.pkgs.lib.callPackageWith ( buffet.pkgs // buffet.support.rs // buffet.support // buffet );
in
rec {
  # RAW NODES
  # -   raw nodes are incomplete and immature, they may wink into and out of existance
  # -   use at own risk, anything in this section can change at any time.

  app_counter_add = callPackage ./app/counter/add {};
  app_counter_card = callPackage ./app/counter/card {};
  app_counter_counter = callPackage ./app/counter/counter {};
  app_counter_create = callPackage ./app/counter/create {};
  app_counter_delta = callPackage ./app/counter/delta {};
  app_counter_minus = callPackage ./app/counter/minus {};
  app_counter_view = callPackage ./app/counter/view { };
  app_counter_viewer = callPackage ./app/counter/viewer {};
  app_growtest = callPackage ./app/growtest {};
  test = app_growtest;
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

  # DRAFT NODES
  # -   draft nodes change a lot in tandom with other nodes in their subgraph
  # -   there will be change in these nodes and few people are using these nodes so expect breakage

  # STABLE NODES
  # -   stable nodes do not change names of ports, agents nor subgraphs,
  # -   you may add new port names, but never change, nor remove port names

  # DEPRECATED NODES
  # -   deprecated nodes do not change names of ports, agents nor subgraphs.
  # -   keep the implementation functioning, print a warning message and tell users to use replacement node

  # LEGACY NODES
  # -   legacy nodes do not change names of ports, agents nor subgraphs.
  # -   assert and remove implementation of the node.
}
