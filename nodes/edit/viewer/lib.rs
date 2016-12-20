extern crate capnp;

#[macro_use]
extern crate rustfbp;

use std::thread;

agent! {
    input(input: generic_text),
    output(text: generic_text, input: generic_tuple_text),
    fn run(&mut self) -> Result<Signal> {
        let mut msg_input = try!(self.input.input.recv());

        let mut reader: generic_text::Reader = try!(msg_input.read_schema());
        {
            let mut msg = Msg::new();
            msg.action = "set_text".into();
            {
                let mut build = msg.build_schema::<generic_text::Builder>();
                build.set_text(try!(reader.get_text()));
            }
            try!(self.output.text.send(msg));
        }
        let mut msg = Msg::new();
        {
            let mut builder = msg.build_schema::<generic_tuple_text::Builder>();
            builder.set_key("value");
            builder.set_value(try!(reader.get_text()));

        }
        msg.action = "set_property".into();
        try!(self.output.input.send(msg));

        Ok(End)
    }
}
