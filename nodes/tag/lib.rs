extern crate capnp;

#[macro_use]
extern crate rustfbp;

use std::thread;

pub struct Portal {
    ty: Option<String>,
    text: Option<String>,
    style: HashMap<String, String>,
    class: HashMap<String, bool>,
    attributes: HashMap<String, String>,
    property: HashMap<String, String>,
    buffer: Vec<Msg>,
}

impl Portal {
    fn new() -> Self {
        Portal {
            ty: None,
            text: None,
            style: HashMap::new(),
            class: HashMap::new(),
            attributes: HashMap::new(),
            property: HashMap::new(),
            buffer: Vec::new(),
        }
    }

    fn clear(&mut self) {
        self.ty = None;
        self.text = None;
        self.style.clear();
        self.class.clear();
        self.attributes.clear();
        self.property.clear();
    }

    fn build(&mut self, msg_input: &mut Msg) -> Result<()> {
        self.clear();
        let reader: js_create::Reader = try!(msg_input.read_schema());
        let ty = try!(reader.get_type());
        self.ty = Some(ty.into());
        let text = try!(reader.get_text());
        if text != "" {
            self.text = Some(text.into());
        }
        for style in try!(reader.get_style()).iter() {
            let key = try!(style.get_key());
            let value = try!(style.get_val());
            self.style.insert(key.into(), value.into());
        }
        for attr in try!(reader.get_attr()).iter() {
            let key = try!(attr.get_key());
            let value = try!(attr.get_val());
            self.attributes.insert(key.into(), value.into());
        }
        for class in try!(reader.get_class()).iter() {
            let name = try!(class.get_name());
            let set = class.get_set();
            self.class.insert(name.into(), set);
        }
        for prop in try!(reader.get_property()).iter() {
            let key = try!(prop.get_key());
            let value = try!(prop.get_val());
            self.property.insert(key.into(), value.into());
        }
        Ok(())
    }
}

agent! {
    input(input: any),
    output(output: any),
    outarr(output: any),
    portal(Portal => Portal::new()),
    fn run(&mut self) -> Result<Signal> {
        let mut msg_input = try!(self.input.input.recv());
        if &msg_input.action != "create" && self.portal.ty.is_none() {
            self.portal.buffer.push(msg_input);
        } else {
            try!(handle_msg(self, msg_input));
        }
        Ok(End)
    }
}

pub fn handle_msg(mut comp: &mut ThisAgent, mut msg_input: Msg) -> Result<()> {
    match &msg_input.action[..] {
        "create" => {
            // Put in the portal
            try!(comp.portal.build(&mut msg_input));
            // create the create Msg
            {
                let mut builder = try!(msg_input.edit_schema::<js_create::Builder, js_create::Reader>());
                // set the name
                builder.set_name(&comp.name);
                // set the sender (raw msg to the input port)
                let sender = Box::new(comp.input.input.get_sender());
                builder.set_sender(Box::into_raw(sender) as u64);
            }
            let _ = send_action!(comp, output, msg_input);
            let buffer = comp.portal.buffer.drain(..).collect::<Vec<_>>();
            for msg in buffer {
                try!(handle_msg(&mut comp, msg));
            }
        }
        // CSS
        "set_css" => {
            // Change in portal
            let reader = try!(msg_input.read_schema::<generic_tuple_text::Reader>());
            let key = try!(reader.get_key());
            let value = try!(reader.get_value());
            comp.portal.style.insert(key.into(), value.into());
            // Send outside
            let mut msg = Msg::new();
            msg.action = "forward".into();
            {
                let mut builder = msg.build_schema::<js_create::Builder>();
                builder.set_name(&comp.name);
                let mut style = builder.init_style(1);
                style.borrow().get(0).set_key(key);
                style.borrow().get(0).set_val(value);
            }
            try!(send_action!(comp, output, msg));
        }
        "get_css" => {
            let reader = try!(msg_input.read_schema::<generic_tuple_text::Reader>());
            let key = try!(reader.get_key());
            let value = try!(reader.get_value());
            let resp = comp.portal.style.get(value).map(|resp| resp.as_str())
                .unwrap_or("");
            let mut msg = Msg::new();
            {
                let mut builder = msg.build_schema::<generic_text::Builder>();
                builder.set_text(resp);
            }
            msg.action = key.to_string();
            let _ = send_action!(comp, output, msg);
        }
        // Attributes
        "set_attr" => {
            // Change in portal
            let reader = try!(msg_input.read_schema::<generic_tuple_text::Reader>());
            let key = try!(reader.get_key());
            let value = try!(reader.get_value());
            comp.portal.attributes.insert(key.into(), value.into());
            // Send outside
            let mut msg = Msg::new();
            msg.action = "forward".into();
            {
                let mut builder = msg.build_schema::<js_create::Builder>();
                builder.set_name(&comp.name);
                let mut attr = builder.init_attr(1);
                attr.borrow().get(0).set_key(key);
                attr.borrow().get(0).set_val(value);
            }
            try!(send_action!(comp, output, msg));
        }
        "get_attr" => {
            let reader = try!(msg_input.read_schema::<generic_tuple_text::Reader>());
            let key = try!(reader.get_key());
            let value = try!(reader.get_value());
            let resp = comp.portal.attributes.get(value).map(|resp| resp.as_str())
                .unwrap_or("");
            let mut msg = Msg::new();
            {
                let mut builder = msg.build_schema::<generic_text::Builder>();
                builder.set_text(resp);
            }
            msg.action = key.to_string();
            let _ = send_action!(comp, output, msg);
        }
        // class
        "set_class" => {
            // Change in portal
            let reader = try!(msg_input.read_schema::<generic_tuple_text::Reader>());
            let key = try!(reader.get_key());
            let value = try!(reader.get_value());
            let value = if value == "true" { true } else { false };
            comp.portal.class.insert(key.into(), value);
            // Send outside
            let mut msg = Msg::new();
            msg.action = "forward".into();
            {
                let mut builder = msg.build_schema::<js_create::Builder>();
                builder.set_name(&comp.name);
                let mut class = builder.init_class(1);
                class.borrow().get(0).set_name(key);
                class.borrow().get(0).set_set(value);
            }
            try!(send_action!(comp, output, msg));
        }
        "get_class" => {
            let reader = try!(msg_input.read_schema::<generic_tuple_text::Reader>());
            let key = try!(reader.get_key());
            let value = try!(reader.get_value());
            let resp = comp.portal.class.get(value).map(|b| b.to_owned()).unwrap_or(false);
            let mut msg = Msg::new();
            {
                let mut builder = msg.build_schema::<generic_bool::Builder>();
                builder.set_bool(resp);
            }
            msg.action = key.to_string();
            let _ = send_action!(comp, output, msg);
        }
        // property
        "set_property" => {
            // Change in portal
            let reader = try!(msg_input.read_schema::<generic_tuple_text::Reader>());
            let key = try!(reader.get_key());
            let value = try!(reader.get_value());
            comp.portal.property.insert(key.into(), value.into());
            // Send outside
            let mut msg = Msg::new();
            msg.action = "forward".into();
            {
                let mut builder = msg.build_schema::<js_create::Builder>();
                builder.set_name(&comp.name);
                let mut prop = builder.init_property(1);
                prop.borrow().get(0).set_key(key);
                prop.borrow().get(0).set_val(value);
            }
            try!(send_action!(comp, output, msg));
        }
        "get_property" => {
            let reader = try!(msg_input.read_schema::<generic_tuple_text::Reader>());
            let key = try!(reader.get_key());
            let value = try!(reader.get_value());
            let resp = comp.portal.property.get(value).map(|resp| resp.as_str())
                .unwrap_or("");
            let mut msg = Msg::new();
            {
                let mut builder = msg.build_schema::<generic_text::Builder>();
                builder.set_text(resp);
            }
            msg.action = key.to_string();
            let _ = send_action!(comp, output, msg);
        }
        // Content
        "set_text" => {
            let reader = try!(msg_input.read_schema::<generic_text::Reader>());
            let new_content = try!(reader.get_text());
            // Change in portal
            comp.portal.text = Some(new_content.to_string());
            // Send new content
            let mut msg = Msg::new();
            msg.action = "forward".to_string();
            {
                let mut builder: js_create::Builder = msg.build_schema();
                builder.set_name(&comp.name);
                builder.set_text(new_content);
            }
            send_action!(comp, output, msg);
        }
        "insert_text" => {
            msg_input.action = "forward_create".into();
            {
                let mut builder = try!(msg_input.edit_schema::<js_create::Builder, js_create::Reader>());
                builder.set_append(&comp.name);
            }
            send_action!(comp, output, msg_input);
        }
        "get_text" => {
            let reader = try!(msg_input.read_schema::<generic_text::Reader>());
            let key = try!(reader.get_text());
            let resp = comp.portal.text.as_ref().map(|resp| resp.as_str()).unwrap_or("");
            let mut msg = Msg::new();
            {
                let mut builder = msg.build_schema::<generic_text::Builder>();
                builder.set_text(resp);
            }
            msg.action = key.to_string();
            let _ = send_action!(comp, output, msg);
        }
        "input" => {
            {
                let mut reader: generic_text::Reader = try!(msg_input.read_schema());
                comp.portal.property.insert("value".into(), try!(reader.get_text()).into());
            }
            let _ = send_action!(comp, output, msg_input);

        }
        "delete" => {
            {
                let mut builder: js_create::Builder = msg_input.build_schema();
                builder.set_name(&comp.name);
                builder.set_remove(true);
            }
            msg_input.action = "forward".into();
            let _ = send_action!(comp, output, msg_input);
        }
        _ => { let _ = send_action!(comp, output, msg_input); }
    }

    Ok(())
}
