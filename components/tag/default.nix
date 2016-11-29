{ component, contracts }:

component {
  src = ./.;
  contracts = with contracts; [ js_create generic_tuple_text generic_text generic_bool ];
  depsSha256 = "1g77ji2kqxacydg8mnzg4a9i5k7x6zgvvfgw6f0m26j0267883lp";
}
