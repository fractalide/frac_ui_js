extern crate capnp;

#[macro_use]
extern crate rustfbp;

use std::thread;

agent! {
    input(input: prim_text),
    output(ph: js_create, text: js_create, input: js_create),
    fn run(&mut self) -> Result<Signal> {
        let mut msg_input = self.input.input.recv()?;

        let text = {
            let mut reader: prim_text::Reader = msg_input.read_schema()?;
            reader.get_text()?
        };

        // ph
        let mut msg = Msg::new();
        {
            let mut builder = msg.build_schema::<js_create::Builder>();
            builder.get_type()?.set_text("div");
        }
        msg.action = "create".into();
        self.output.ph.send(msg)?;

        // text
        let mut msg = Msg::new();
        {
            let mut builder = msg.build_schema::<js_create::Builder>();
            builder.borrow().get_type()?.set_text("span");
            builder.borrow().get_text()?.set_text(&format!("{}", text));
        }
        msg.action = "create".into();
        self.output.text.send(msg)?;
        let mut msg = Msg::new();
        msg.action = "display".into();
        self.output.text.send(msg)?;

        // input
        let mut new_msg = Msg::new();
        {
            let mut builder = new_msg.build_schema::<js_create::Builder>();
            builder.borrow().get_type()?.set_text("input");
            {
                let mut attr = builder.borrow().get_property()?.init_list(1);
                attr.borrow().get(0).get_key()?.set_text("value");
                attr.borrow().get(0).get_val()?.set_text(text);
            }
        }
        new_msg.action = "create".into();
        self.output.input.send(new_msg)?;


        Ok(End)
    }
}
