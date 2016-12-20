extern crate capnp;

#[macro_use]
extern crate rustfbp;

use std::thread;

agent! {
    input(input: any),
    output(output: any, scheduler: fbp_action),
    outarr(output: any),
    option(generic_text),
    portal(usize => 0),
    fn run(&mut self) -> Result<Signal> {

        let mut msg = try!(self.input.input.recv());
        match &msg.action[..] {
            "create" => {
                // Send the create comp Msg
                let mut send_msg = Msg::new();
                {
                    let mut builder: fbp_action::Builder = send_msg.build_schema();
                    let mut add = builder.init_add();
                    add.set_name("flex");
                    add.set_comp("ui_js_flex");
                }
                try!(self.output.scheduler.send(send_msg));
                // Connect to outside
                let mut connect_msg = Msg::new();
                {
                    let mut builder: fbp_action::Builder = connect_msg.build_schema();
                    let mut connect = builder.init_connect_sender();
                    connect.set_name("flex");
                    connect.set_port("output");
                    connect.set_output("flex");
                }
                try!(self.output.scheduler.send(connect_msg));

                // Send the acc Msg
                let mut send_msg = Msg::new();
                {
                    let mut builder: fbp_action::Builder = send_msg.build_schema();
                    let mut connect = builder.init_send();
                    connect.set_comp("flex");
                    connect.set_port("input");
                }
                try!(self.output.scheduler.send(send_msg));
                try!(self.output.scheduler.send(msg));

            }
            "remove" => {
                if self.portal > 0 {
                    let name = format!("i{}", self.portal);

                    // Send the delete Msg
                    let mut send_msg = Msg::new();
                    {
                        let mut builder: fbp_action::Builder = send_msg.build_schema();
                        let mut connect = builder.init_send();
                        connect.set_comp(&name);
                        connect.set_port("input");
                    }
                    try!(self.output.scheduler.send(send_msg));
                    let mut comp_msg = Msg::new();
                    comp_msg.action = "delete".into();
                    try!(self.output.scheduler.send(comp_msg));


                    // TODO : remove the sleep once scheduler.remove_comp is async
                    thread::sleep_ms(50);
                    // Send the remove Msg
                    let mut remove_msg = Msg::new();
                    {
                        let mut builder: fbp_action::Builder = remove_msg.build_schema();
                        let mut rem = builder.set_remove(&name);
                    }
                    try!(self.output.scheduler.send(remove_msg));

                    self.portal -= 1;
                }
            },
            "add" => {
                self.portal += 1;
                // Add link
                let mut msg_opt = self.recv_option();
                let mut reader: generic_text::Reader = try!(msg_opt.read_schema());
                let name = format!("i{}", self.portal);
                // Send the create comp Msg
                let mut send_msg = Msg::new();
                {
                    let mut builder: fbp_action::Builder = send_msg.build_schema();
                    let mut add = builder.init_add();
                    add.set_name(&name);
                    add.set_comp(try!(reader.get_text()));
                }
                try!(self.output.scheduler.send(send_msg));

                // Send the connect Msg
                let mut send_msg = Msg::new();
                {
                    let mut builder: fbp_action::Builder = send_msg.build_schema();
                    let mut connect = builder.init_connect();
                    connect.set_o_name(&name);
                    connect.set_o_port("output");
                    connect.set_i_name("flex");
                    connect.set_i_port("places");
                    connect.set_i_selection(&name)
                }
                try!(self.output.scheduler.send(send_msg));

                // Send the create Msg
                let mut send_msg = Msg::new();
                {
                    let mut builder: fbp_action::Builder = send_msg.build_schema();
                    let mut send = builder.init_send();
                    send.set_comp(&name);
                    send.set_port("input");
                }
                try!(self.output.scheduler.send(send_msg));
                msg.action = "create".into();
                try!(self.output.scheduler.send(msg));
            }
            _ => { try!(self.send_action("output", msg)); }
        };
        Ok(End)
    }
}
