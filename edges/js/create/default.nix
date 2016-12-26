{ edge, edges }:

edge {
  src = ./.;
  edges =  with edges; [ prim_text prim_u64 prim_bool list_key_t_val_t list_ntuple_tuple_tb];
  schema = with edges; ''
    @0x932e7ad13a9eb1bb;

    using PrimText = import "${prim_text}/src/edge.capnp";
    using PrimU64 = import "${prim_u64}/src/edge.capnp";
    using PrimBool = import "${prim_bool}/src/edge.capnp";
    using ListKeyTValT = import "${list_key_t_val_t}/src/edge.capnp";
    using ListNtupleTupleTb = import "${list_ntuple_tuple_tb}/src/edge.capnp";

    struct JsCreate {
      name @0 :PrimText.PrimText;
      sender @1 :PrimU64.PrimU64;
      append @2 :PrimText.PrimText;
      attr @3 :ListKeyTValT.ListKeyTValT; #entry
      class @4 :ListNtupleTupleTb.ListNtupleTupleTb; #tuple
      style @5 :ListKeyTValT.ListKeyTValT; #entry
      property @6 :ListKeyTValT.ListKeyTValT; #entry
      text @7 :PrimText.PrimText;
      remove @8 :PrimBool.PrimBool;
      type @9 :PrimText.PrimText;
    }
  '';
}
