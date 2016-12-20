extern crate capnp;

#[macro_use]
extern crate rustfbp;

use std::thread;

// TODO : add ctrl-maj-meta information
agent! {
    input(input: generic_text),
    output(validate: generic_text, display: any),
    fn run(&mut self) -> Result<Signal> {
        let mut msg_input = try!(self.input.input.recv());

        {
            let mut builder: generic_text::Builder = msg_input.build_schema();
            builder.set_text("content_edited");
        }
        msg_input.action = "get_val".into();
        try!(self.output.validate.send(msg_input));

        let mut new_msg = Msg::new();
        new_msg.action = "display".into();
        try!(self.output.display.send(new_msg));

        Ok(())
    }
}
