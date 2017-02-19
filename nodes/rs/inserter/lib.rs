extern crate capnp;

#[macro_use]
extern crate rustfbp;

agent! {
    input(input: any),
    output(output: any),
    fn run(&mut self) -> Result<Signal> {
      let mut msg = try!(self.input.input.recv());
        if msg.action == "create" {
          // msg.action is "insert_content" or "forward"
          msg.action = "insert_text".into();
        }
        try!(self.output.output.send(msg));

        Ok(End)
    }
}
