{ subgraph, imsg, nodes, edges }:
let
  PrimI64 = imsg {
    class = edges.PrimI64;
    text = ''(i64=0)'';
  };
in
subgraph {
  src = ./.;
  flowscript = with nodes; ''
   input => input in_dispatch(${msg_dispatcher}) output -> input out_dispatch(${msg_dispatcher}) output => output

   model(${app_model}) output -> input view(${app_counter_view}) output -> input out_dispatch()

   '${PrimI64}' -> acc model()
   out_dispatch() output[add] -> input model()
   out_dispatch() output[minus] -> input model()
   out_dispatch() output[delta] -> input model()

   in_dispatch() output[create] -> input clone_create(${msg_clone})
   clone_create() clone[0] -> input view()
   clone_create() clone[1] -> input model()

   in_dispatch() output[delete] -> input view()

   model() compute[add] -> input add(${app_counter_add}) output -> result model()
   model() compute[minus] -> input minus(${app_counter_minus}) output -> result model()
   model() compute[delta] -> input delta(${app_counter_delta}) output -> result model()
   '';
}
