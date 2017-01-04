{ edge, edges }:

edge {
  src = ./.;
  edges =  with edges; [ kv_list_key_t_val_t ntup_list_tuple_tb];
  schema = with edges; ''
    @0x932e7ad13a9eb1bb;

    using KvListKeyTValT = import "${kv_list_key_t_val_t}/src/edge.capnp";
    using NtupListTupleTb = import "${ntup_list_tuple_tb}/src/edge.capnp";

    struct UiJsCreate {
      name @0 :Text;
      sender @1 :UInt64;
      append @2 :Text;
      attr @3 :KvListKeyTValT.KvListKeyTValT; #entry
      class @4 :NtupListTupleTb.NtupListTupleTb; #tuple
      style @5 :KvListKeyTValT.KvListKeyTValT; #entry
      property @6 :KvListKeyTValT.KvListKeyTValT; #entry
      text @7 :Text;
      remove @8 :Bool;
      type @9 :Text;
    }
  '';
}
