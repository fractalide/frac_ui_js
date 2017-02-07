{ subgraph, imsgs, nodes, edges }:

subgraph rec {
  src = ./.;
  imsg = imsgs {
    edges = with edges; [ UiAppCounter UiJsCreate PrimText];
  };
  flowscript = with nodes; ''
   td(${flex}) output -> input page(${page})
   '${imsg}.UiJsCreate:(type="div", style=[(key="display", val="flex"), (key="flex-direction", val="column")])~create' -> input td()

   lr(${flex}) output -> places[1] td()
   '${imsg}.UiJsCreate:(type="div", style=[(key="display", val="flex")])~create' -> input lr()

   button_add(${tag}) output -> places[1] lr()
   button_remove(${tag}) output -> places[2] lr()
   '${imsg}.UiJsCreate:(type="button", text="add")~create' -> input button_add()
   '${imsg}.UiJsCreate:(type="button", text="remove")~create' -> input button_remove()

   dummy()

   gflex(${growing_flex}) output -> places[2] td()
   gflex() scheduler -> action sched(${core_subgraph})
   sched() outputs[flex] -> places[2] td()
   '${imsg}.PrimText:(text="${app_counter_card}")' -> option gflex()
   '${imsg}.UiJsCreate:(type="div", style=[(key="display", val="flex"), (key="flex-direction", val="column")])~create' -> input gflex()

   button_add() output[click] -> input add(${msg_replace}) output -> input gflex()
   button_remove() output[click] -> input minus(${msg_action}) output -> input gflex()
   '${imsg}.PrimText:(text="remove")' -> option minus()
   '${imsg}.UiAppCounter:(value=0)~add' -> option add()

   '';
}
