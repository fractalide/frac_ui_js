{ edge, edges }:

edge {
  src = ./.;
  edges =  with edges; [ KvListKeyTValT NtupListTupleTb];
  schema = with edges; ''
    struct UiJsCreate {
      name @0 :Text;
      sender @1 :UInt64;
      append @2 :Text;
      attr @3 :KvListKeyTValT;
      class @4 :NtupListTupleTb;
      style @5 :KvListKeyTValT;
      property @6 :KvListKeyTValT;
      text @7 :Text;
      remove @8 :Bool;
      type @9 :Text;
    }
  '';
}
