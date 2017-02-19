{ subgraph, imsg, nodes, edges }:
let
  UiJsCreate1 = imsg {
    class = edges.UiJsCreate;
    text = ''(type="div", style=(list=[(key=(text="display"), val=(text="flex")), (key=(text="flex-direction"), val=(text="column"))]))'';
    option = "create";
  };
  UiJsCreate2 = imsg {
    class = edges.UiJsCreate;
    text = ''(type="div", style=(list=[(key=(text="display"), val=(text="flex"))]))'';
    option = "create";
  };
  UiJsCreate3 = imsg {
    class = edges.UiJsCreate;
    text = ''(type="button", text="add")'';
    option = "create";
  };
  UiJsCreate4 = imsg {
    class = edges.UiJsCreate;
    text = ''(type="button", text="remove")'';
    option = "create";
  };
  PrimText1 = imsg {
    class = edges.PrimText;
    text = with nodes.rs; ''(text="${app_counter_card}")'';
  };
  UiJsCreate5 = imsg {
    class = edges.UiJsCreate;
    text = ''(type="div", style=(list=[(key=(text="display"), val=(text="flex")), (key=(text="flex-direction"), val=(text="column"))]))'';
    option = "create";
  };
  PrimText2 = imsg {
    class = edges.PrimText;
    text = ''(text="remove")'';
  };
  UiAppCounter = imsg {
    class = edges.UiAppCounter;
    text = ''(value=0)'';
    option = "add";
  };
in
subgraph {
  src = ./.;
  flowscript = with nodes.rs; with nodes; ''
   td(${flex}) output -> input page(${page})
   '${UiJsCreate1}' -> input td()

   lr(${flex}) output -> places[1] td()
   '${UiJsCreate2}' -> input lr()

   button_add(${tag}) output -> places[1] lr()
   button_remove(${tag}) output -> places[2] lr()
   '${UiJsCreate3}' -> input button_add()
   '${UiJsCreate4}' -> input button_remove()

   dummy()

   gflex(${growing_flex}) output -> places[2] td()
   gflex() scheduler -> action sched(${fvm_rs_scheduler})
   sched() outputs[flex] -> places[2] td()
   '${PrimText1}' -> option gflex()
   '${UiJsCreate5}' -> input gflex()

   button_add() output[click] -> input add(${msg_replace}) output -> input gflex()
   button_remove() output[click] -> input minus(${msg_action}) output -> input gflex()
   '${PrimText2}' -> option minus()
   '${UiAppCounter}' -> option add()

   '';
}
