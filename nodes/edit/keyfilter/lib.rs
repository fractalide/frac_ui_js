extern crate capnp;

#[macro_use]
extern crate rustfbp;

use std::thread;

// TODO : add ctrl-maj-meta information
agent! {
    input(input: generic_text),
    output(validate: generic_tuple_text, escape: generic_text, display: any),
    fn run(&mut self) -> Result<Signal> {
        let mut msg_input = try!(self.input.input.recv());

        let mut res = "".to_string();
        {
            let mut reader: generic_text::Reader = try!(msg_input.read_schema());
            res.push_str(try!(reader.get_text()));
        }
        if res == "27" { // Escape
            msg_input.action = "get_model".into();
            {
                let mut builder: generic_text::Builder = msg_input.build_schema();
                builder.set_text("escape");
            }
            try!(self.output.escape.send(msg_input));
            let mut new_msg = Msg::new();
            new_msg.action = "display".into();
            try!(self.output.display.send(new_msg))
        } else if res == "13" { // Enter
            msg_input.action="get_property".into();
            {
                let mut builder: generic_tuple_text::Builder = msg_input.build_schema();
                builder.set_key("content_edited");
                builder.set_value("value");
            }
            try!(self.output.validate.send(msg_input));
            let mut new_msg = Msg::new();
            new_msg.action = "display".into();
            try!(self.output.display.send(new_msg))
        }

        Ok(End)
    }
}
