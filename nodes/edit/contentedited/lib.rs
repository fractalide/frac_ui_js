extern crate capnp;

#[macro_use]
extern crate rustfbp;

use std::thread;

agent! {
    input(input: generic_text),
    output(output: generic_text),
    fn run(&mut self) -> Result<Signal> {
        let mut msg_new = try!(self.input.input.recv());
        let _ = try!(self.input.input.recv());

        if &msg_new.action != "content_edited" {
            return Err(result::Error::Misc("Bad action".into()));
        }

        try!(self.output.output.send(msg_new));

        Ok(End)
    }
}
