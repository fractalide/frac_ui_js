extern crate capnp;

#[macro_use]
extern crate rustfbp;

use std::thread;

agent! {
    input(input: prim_text),
    output(text: prim_text, input: ntup_tuple_tt),
    fn run(&mut self) -> Result<Signal> {
        let mut msg_input = try!(self.input.input.recv());

        let mut reader: prim_text::Reader = try!(msg_input.read_schema());
        {
            let mut msg = Msg::new();
            msg.action = "set_text".into();
            {
                let mut build = msg.build_schema::<prim_text::Builder>();
                build.set_text(try!(reader.get_text()));
            }
            try!(self.output.text.send(msg));
        }
        let mut msg = Msg::new();
        {
            let mut builder = msg.build_schema::<ntup_tuple_tt::Builder>();
            builder.borrow().get_first()?.set_text("value");
            builder.borrow().get_second()?.set_text(try!(reader.get_text()));

        }
        msg.action = "set_property".into();
        try!(self.output.input.send(msg));

        Ok(End)
    }
}
