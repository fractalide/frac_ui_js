extern crate capnp;

#[macro_use]
extern crate rustfbp;

use std::thread;

// TODO : add ctrl-maj-meta information
agent! {
    input(input: prim_text),
    output(validate: ntuple_tuple_tt, escape: prim_text, display: any),
    fn run(&mut self) -> Result<Signal> {
        let mut msg_input = try!(self.input.input.recv());

        let mut res = "".to_string();
        {
            let mut reader: prim_text::Reader = try!(msg_input.read_schema());
            res.push_str(try!(reader.get_text()));
        }
        if res == "27" { // Escape
            msg_input.action = "get_model".into();
            {
                let mut builder: prim_text::Builder = msg_input.build_schema();
                builder.set_text("escape");
            }
            try!(self.output.escape.send(msg_input));
            let mut new_msg = Msg::new();
            new_msg.action = "display".into();
            try!(self.output.display.send(new_msg))
        } else if res == "13" { // Enter
            msg_input.action="get_property".into();
            {
                let mut builder: ntuple_tuple_tt::Builder = msg_input.build_schema();
                builder.borrow().get_first()?.set_text("content_edited");
                builder.borrow().get_second()?.set_text("value");
            }
            try!(self.output.validate.send(msg_input));
            let mut new_msg = Msg::new();
            new_msg.action = "display".into();
            try!(self.output.display.send(new_msg))
        }

        Ok(End)
    }
}
