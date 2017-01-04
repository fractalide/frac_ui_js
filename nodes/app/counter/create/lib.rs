extern crate capnp;

#[macro_use]
extern crate rustfbp;

use std::thread;

agent! {
    input(input: ui_app_counter),
    output(label: ui_js_create, delta: ui_js_create, plus:ui_js_create, minus:ui_js_create, td: ui_js_create, lr: ui_js_create),
    fn run(&mut self) -> Result<Signal> {
        let mut msg_input = try!(self.input.input.recv());

        let (number, delta) = {
            let mut reader: ui_app_counter::Reader = try!(msg_input.read_schema());
            (reader.get_value(), reader.get_delta())
        };

        let mut msg = Msg::new();
        // Plus button
        {
            let mut builder = msg.build_schema::<ui_js_create::Builder>();
            builder.borrow().set_type("button");
            builder.borrow().set_text("+");
        }
        msg.action = "create".into();
        try!(self.output.plus.send(msg));

        // Minus button
        let mut msg = Msg::new();
        {
            let mut builder = msg.build_schema::<ui_js_create::Builder>();
            builder.borrow().set_type("button");
            builder.borrow().set_text("-");
        }
        msg.action = "create".into();
        try!(self.output.minus.send(msg));

        // td
        let mut msg = Msg::new();
        {
            let mut builder = msg.build_schema::<ui_js_create::Builder>();
            builder.borrow().set_type("div");
            {
                let mut css = builder.borrow().get_style()?.init_list(2);
                css.borrow().get(0).get_key()?.set_text("display");
                css.borrow().get(0).get_val()?.set_text("flex");
                css.borrow().get(1).get_key()?.set_text("flex-direction");
                css.borrow().get(1).get_val()?.set_text("column");
            }
        }
        msg.action = "create".into();
        try!(self.output.td.send(msg));

        // lr
        let mut msg = Msg::new();
        {
            let mut builder = msg.build_schema::<ui_js_create::Builder>();
            builder.borrow().set_type("div");
            {
                let mut css = builder.borrow().get_style()?.init_list(1);
                css.borrow().get(0).get_key()?.set_text("display");
                css.borrow().get(0).get_val()?.set_text("flex");
            }
        }
        msg.action = "create".into();
        try!(self.output.lr.send(msg));

        // label
        let mut msg = Msg::new();
        {
            let mut builder = msg.build_schema::<ui_js_create::Builder>();
            builder.borrow().set_type("span");
            builder.borrow().set_text(&format!("{}", number));
            {
                let mut css = builder.borrow().get_style()?.init_list(1);
                css.borrow().get(0).get_key()?.set_text("margin");
                css.borrow().get(0).get_val()?.set_text("0 10px");
            }
        }
        msg.action = "create".into();
        try!(self.output.label.send(msg));

        let mut new_msg = Msg::new();
        {
            let mut builder = new_msg.build_schema::<ui_js_create::Builder>();
            builder.borrow().set_type("input");
            {
                let mut attr = builder.borrow().get_property()?.init_list(1);
                attr.borrow().get(0).get_key()?.set_text("value");
                attr.borrow().get(0).get_val()?.set_text(&format!("{}", delta));
            }
            {
                let mut style = builder.borrow().get_style()?.init_list(1);
                style.borrow().get(0).get_key()?.set_text("width");
                style.borrow().get(0).get_val()?.set_text("90px");
            }
        }
        new_msg.action = "create".into();
        try!(self.output.delta.send(new_msg));


        Ok(End)
    }
}
