extern crate capnp;

#[macro_use]
extern crate rustfbp;

use std::thread;

agent! {
    input(input: generic_text),
    output(ph: js_create, text: js_create, input: js_create),
    fn run(&mut self) -> Result<Signal> {
        let mut msg_input = try!(self.input.input.recv());

        let text = {
            let mut reader: generic_text::Reader = try!(msg_input.read_schema());
            try!(reader.get_text())
        };

        // ph
        let mut msg = Msg::new();
        {
            let mut builder = msg.build_schema::<js_create::Builder>();
            builder.set_type("div");
        }
        msg.action = "create".into();
        try!(self.output.ph.send(msg));

        // text
        let mut msg = Msg::new();
        {
            let mut builder = msg.build_schema::<js_create::Builder>();
            builder.set_type("span");
            builder.set_text(&format!("{}", text));
        }
        msg.action = "create".into();
        try!(self.output.text.send(msg));
        let mut msg = Msg::new();
        msg.action = "display".into();
        try!(self.output.text.send(msg));

        // input
        let mut new_msg = Msg::new();
        {
            let mut builder = new_msg.build_schema::<js_create::Builder>();
            builder.set_type("input");
            {
                let mut attr = builder.borrow().init_property(1);
                attr.borrow().get(0).set_key("value");
                attr.borrow().get(0).set_val(text);
            }
        }
        new_msg.action = "create".into();
        try!(self.output.input.send(new_msg));


        Ok(End)
    }
}
