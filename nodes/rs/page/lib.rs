extern crate capnp;
extern crate ws;

#[macro_use]
extern crate rustfbp;

use ws::{listen, Handler, Message, Handshake, CloseCode};
use std::thread;
use std::sync::mpsc::channel;

struct Server {
    out: ws::Sender,
    input: MsgSender,
}

impl Handler for Server {
    fn on_message(&mut self, msg: Message) -> ws::Result<()> {
        let mut new_msg = Msg::new();
        new_msg.action = "intern_msg".into();
        {
            let mut builder = new_msg.build_schema::<prim_text::Builder>();
            let msg = try!(msg.as_text());
            builder.set_text(msg);
        }
        &self.input.send(new_msg).expect("cannot send intern");
        Ok(())
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        // The WebSocket protocol allows for a utf8 reason for the closing state after the
        // close code. WS-RS will attempt to interpret this data as a utf8 descrmsgtion of the
        // reason for closing the connection. I many cases, `reason` will be an empty string.
        // So, you may not normally want to display `reason` to the user,
        // but let's assume that we know that `reason` is human-readable.
        match code {
            CloseCode::Normal => println!("The client is done with the connection."),
            CloseCode::Away   => println!("The client is leaving the site."),
            _ => println!("The client encountered an error: {}", reason),
        }
    }
}

agent! {
    input(input: any),
    output(output: any),
    outarr(output: any),
    fn run(&mut self) -> Result<Signal> {

        let (s, r) = channel();
        let in_sender = self.input.input.get_sender();

        let handle = thread::spawn(move || {
            listen("127.0.0.1:3012", move |out| {
                s.send(out.clone()).expect("cannot send");
                Server {
                    out: out,
                    input: in_sender,
                }
            });
        });

        let mut out = try!(r.recv());
        let mut senders: HashMap<String, Box<MsgSender>> = HashMap::new();

        loop {
            let mut msg = try!(self.input.input.recv());
            let act = msg.action.clone();
            match &act[..] {
                "create" => {
                    // Add append to main
                    {
                        let mut builder = try!(msg.edit_schema::<js_create::Builder, js_create::Reader>());
                        builder.set_append("main");
                    }
                    try!(msg.before_send());
                    // Save the sender
                    {
                        let mut reader: js_create::Reader = try!(msg.read_schema());
                        let name = try!(reader.get_name());
                        let ptr = reader.get_sender();
                        if name.len() > 0 {
                            let sender: Box<MsgSender> = unsafe { Box::from_raw(ptr as *mut MsgSender) };
                            senders.insert(name.into(), sender);
                        }
                    }
                    // Create d3
                    let d3 = try!(create_d3(msg));
                    out.send(d3);
                },
                "forward_create" => {
                    {
                        let mut reader: js_create::Reader = try!(msg.read_schema());
                        let name = try!(reader.get_name());
                        let ptr = reader.get_sender();
                        if name.len() > 0 {
                            let sender: Box<MsgSender> = unsafe { Box::from_raw(ptr as *mut MsgSender) };
                            senders.insert(name.into(), sender);
                        }
                    }
                    let d3 = try!(create_d3(msg));
                    out.send(d3);
                },
                // "delete" => {
                //     out.send("html;main;");
                // }
                "forward" => {
                    let d3 = try!(create_d3(msg));
                    out.send(d3);
                }
                "intern_msg" => {
                    let mut reader: prim_text::Reader = try!(msg.read_schema());
                    let text = try!(reader.get_text());
                    let pos = try!(text.find("#").ok_or(result::Error::Misc("bad response from page".into())));
                    let (action, id) = text.split_at(pos);
                    let (_, id) = id.split_at(1);
                    let mut msg = Msg::new();
                    msg.action = action.into();
                    let id = if action == "input" {
                        let pos = try!(id.find("#").ok_or(result::Error::Misc("bad response from page".into())));
                        let (id, text) = id.split_at(pos);
                        let (_, text) = text.split_at(1);
                        {
                            let mut builder: prim_text::Builder = msg.build_schema();
                            builder.set_text(text);
                        }
                        id
                    } else if action == "keyup" {
                        let pos = try!(id.find("#").ok_or(result::Error::Misc("bad response from page".into())));
                        let (id, text) = id.split_at(pos);
                        let (_, text) = text.split_at(1);
                        {
                            let mut builder: prim_text::Builder = msg.build_schema();
                            builder.set_text(text);
                        }
                        id
                    } else {
                        id
                    };
                    if senders.contains_key(id) {
                        let s = senders.get(id).expect("unreachable");
                        try!(s.send(msg));
                    }
                },
                _ => {
                    println!("Receive a random msg : {}", act);
                    send_action!(self, output, msg);
                }


            }
        }

        handle.join().expect("cannot join");

        Ok(End)
    }
}

fn create_d3(mut msg: Msg) -> Result<String> {
    let mut reader: js_create::Reader = try!(msg.read_schema());
    // Manage name and sender
    let mut d3 = "d3.select(\"#".to_string();
    // Two possibilities : append is set, so add to the parent. append is not send, select the name
    if reader.has_append() {
        d3.push_str(try!(reader.get_append()));
        d3.push_str("\").append(\"");
        d3.push_str(try!(reader.get_type()));
        d3.push_str("\").attr(\"id\", \"");
        d3.push_str(try!(reader.get_name()));
        d3.push_str("\")");
    } else {
        d3.push_str(try!(reader.get_name()));
        d3.push_str("\")");
    }

    if reader.get_remove() {
        d3.push_str(".remove();");
        return Ok(d3);
    }

    let text = try!(reader.get_text());
    if text != "" {
        d3.push_str(".text(\"");
        d3.push_str(text);
        d3.push_str("\")");
    }

    for attr in try!(reader.get_attr()).iter() {
        d3.push_str(".attr(\"");
        d3.push_str(try!(attr.get_key()));
        d3.push_str("\", \"");
        d3.push_str(try!(attr.get_val()));
        d3.push_str("\")");
    }
    for class in try!(reader.get_class()).iter() {
        d3.push_str(".classed(\"");
        d3.push_str(try!(class.get_name()));
        d3.push_str("\",");
        if class.get_set() {
            d3.push_str("true");
        } else {
            d3.push_str("false");
        }
        d3.push_str(")");
    }
    for style in try!(reader.get_style()).iter() {
        d3.push_str(".style(\"");
        d3.push_str(try!(style.get_key()));
        d3.push_str("\", \"");
        d3.push_str(try!(style.get_val()));
        d3.push_str("\")");
    }
    for property in try!(reader.get_property()).iter() {
        d3.push_str(".property(\"");
        d3.push_str(try!(property.get_key()));
        d3.push_str("\", \"");
        d3.push_str(try!(property.get_val()));
        d3.push_str("\")");
    }
    d3.push_str(";");
    Ok(d3)
}
