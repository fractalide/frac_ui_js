{ subgraph, nodes, edges }:

subgraph {
  src = ./.;
  flowscript = with nodes; with edges; ''
   input => input in_dispatch(${ip_dispatcher}) output -> input out_dispatch(${ip_dispatcher}) output => output

   model(${app_model}) output -> input view(${ui_js_nodes.edit_view}) output -> input out_dispatch()

   '${generic_text}:(text="")' -> acc model()

   in_dispatch() output[create] -> input create_clone(${ip_clone})
   create_clone() clone[1] -> input view()
   create_clone() clone[2] -> input model()

   in_dispatch() output[delete] -> input view()
   in_dispatch() output[set_property] -> input view()

   view() output[get_model] -> input model()
   view() output[content_edited] -> input model()

   model() compute[content_edited] -> input ce(${ui_js_nodes.edit_contentedited}) output -> result model()
   model() compute[set_property] -> input ce()
   '';
}
