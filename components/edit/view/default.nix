{ subnet, contracts, components }:

subnet {
  src = ./.;
  flowscript = with contracts; with components; ''
   input => input in_dispatch(${ip_dispatcher}) output -> input out_dispatch(${ip_dispatcher}) output => output

   ph(${ui_js_components.placeholder}) output -> input out_dispatch()

   text(${ui_js_components.tag}) output -> places[1] ph()
   input(${ui_js_components.tag}) output -> places[2] ph()

   text() output[dblclick] -> input disp_input(${ip_action}) output -> input input()
   '${generic_text}:(text="display")' -> option disp_input()

   input() output[keyup] -> input key_filter(${ui_js_components.edit_keyfilter})
   key_filter() validate -> input input()
   key_filter() escape -> input out_dispatch()
   key_filter() display -> input text()

   input() output[focusout] -> input validate(${ui_js_components.edit_validate})
   validate() validate -> input input()
   validate() display -> input text()

   in_dispatch() output[model] -> input viewer(${ui_js_components.edit_viewer})
   in_dispatch() output[escape] -> input viewer()
   viewer() text -> input text()
   viewer() input -> input input()

   in_dispatch() output[create] -> input create(${ui_js_components.edit_create})
   create() ph -> input ph()
   create() text -> input text()
   create() input -> input input()

   in_dispatch() output[delete] -> input clone(${ip_clone})
   clone() clone[1] -> input text()
   clone() clone[2] -> input input()
   clone() clone[3] -> input ph()
   '';
}
