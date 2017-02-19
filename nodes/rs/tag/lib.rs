extern crate capnp;

#[macro_use]
extern crate rustfbp;

use std::thread;

pub struct State {
    ty: Option<String>,
    text: Option<String>,
    style: HashMap<String, String>,
    class: HashMap<String, bool>,
    attributes: HashMap<String, String>,
    property: HashMap<String, String>,
    buffer: Vec<Msg>,
}

impl State {
    fn new() -> Self {
        State {
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
        let reader: ui_js_create::Reader = try!(msg_input.read_schema());
        let ty = reader.get_type()?;
        self.ty = Some(ty.into());
        let text = reader.get_text()?;
        if text != "" {
            self.text = Some(text.into());
        }
        for style in reader.get_style()?.get_list()?.iter() {
            let key = style.get_key()?.get_text()?;
            let value = style.get_val()?.get_text()?;
            self.style.insert(key.into(), value.into());
        }
        for attr in reader.get_attr()?.get_list()?.iter() {
            let key = attr.get_key()?.get_text()?;
            let value = attr.get_val()?.get_text()?;
            self.attributes.insert(key.into(), value.into());
        }
        for class in reader.get_class()?.get_list()?.iter() {
            let name = class.get_first()?.get_text()?;
            let set = class.get_second()?.get_bool();
            self.class.insert(name.into(), set);
        }
        for prop in reader.get_property()?.get_list()?.iter() {
            let key = prop.get_key()?.get_text()?;
            let value = prop.get_val()?.get_text()?;
            self.property.insert(key.into(), value.into());
        }
        Ok(())
    }
}

agent! {
    input(input: any),
    output(output: any),
    outarr(output: any),
    state(State => State::new()),
    fn run(&mut self) -> Result<Signal> {
        let mut msg_input = try!(self.input.input.recv());
        if &msg_input.action != "create" && self.state.ty.is_none() {
            self.state.buffer.push(msg_input);
        } else {
            try!(handle_msg(self, msg_input));
        }
        Ok(End)
    }
}

pub fn handle_msg(mut comp: &mut ThisAgent, mut msg_input: Msg) -> Result<()> {
    match &msg_input.action[..] {
        "create" => {
            // Put in the state
            try!(comp.state.build(&mut msg_input));
            // create the create Msg
            {
                let mut builder = try!(msg_input.edit_schema::<ui_js_create::Builder, ui_js_create::Reader>());
                // set the name
                builder.borrow().set_name(&comp.name);
                // set the sender (raw msg to the input port)
                let sender = Box::new(comp.input.input.get_sender());
                builder.borrow().set_sender(Box::into_raw(sender) as u64);
            }
            let _ = send_action!(comp, output, msg_input);
            let buffer = comp.state.buffer.drain(..).collect::<Vec<_>>();
            for msg in buffer {
                try!(handle_msg(&mut comp, msg));
            }
        }
        // CSS
        "set_css" => {
            // Change in state
            let reader = msg_input.read_schema::<ntup_tuple_tt::Reader>()?;
            let key = reader.get_first()?.get_text()?;
            let value = reader.get_second()?.get_text()?;
            comp.state.style.insert(key.into(), value.into());
            // Send outside
            let mut msg = Msg::new();
            msg.action = "forward".into();
            {
                let mut builder = msg.build_schema::<ui_js_create::Builder>();
                builder.borrow().set_name(&comp.name);
                let mut style = builder.borrow().get_style()?.init_list(1);
                style.borrow().get(0).get_key()?.set_text(key);
                style.borrow().get(0).get_val()?.set_text(value);
            }
            try!(send_action!(comp, output, msg));
        }
        "get_css" => {
            let reader = msg_input.read_schema::<ntup_tuple_tt::Reader>()?;
            let key = reader.get_first()?.get_text()?;
            let value = reader.get_second()?.get_text()?;
            let resp = comp.state.style.get(value).map(|resp| resp.as_str())
                .unwrap_or("");
            let mut msg = Msg::new();
            {
                let mut builder = msg.build_schema::<prim_text::Builder>();
                builder.set_text(resp);
            }
            msg.action = key.to_string();
            let _ = send_action!(comp, output, msg);
        }
        // Attributes
        "set_attr" => {
            // Change in state
            let reader = msg_input.read_schema::<ntup_tuple_tt::Reader>()?;
            let key = reader.get_first()?.get_text()?;
            let value = reader.get_second()?.get_text()?;
            comp.state.attributes.insert(key.into(), value.into());
            // Send outside
            let mut msg = Msg::new();
            msg.action = "forward".into();
            {
                let mut builder = msg.build_schema::<ui_js_create::Builder>();
                builder.borrow().set_name(&comp.name);
                let mut attr = builder.borrow().get_attr()?.init_list(1);
                attr.borrow().get(0).get_key()?.set_text(key);
                attr.borrow().get(0).get_val()?.set_text(value);
            }
            try!(send_action!(comp, output, msg));
        }
        "get_attr" => {
            let reader = msg_input.read_schema::<ntup_tuple_tt::Reader>()?;
            let key = reader.get_first()?.get_text()?;
            let value = reader.get_second()?.get_text()?;
            let resp = comp.state.attributes.get(value).map(|resp| resp.as_str())
                .unwrap_or("");
            let mut msg = Msg::new();
            {
                let mut builder = msg.build_schema::<prim_text::Builder>();
                builder.set_text(resp);
            }
            msg.action = key.to_string();
            let _ = send_action!(comp, output, msg);
        }
        // class
        "set_class" => {
            // Change in state
            let reader = msg_input.read_schema::<ntup_tuple_tt::Reader>()?;
            let key = reader.get_first()?.get_text()?;
            let value = reader.get_second()?.get_text()?;
            let value = if value == "true" { true } else { false };
            comp.state.class.insert(key.into(), value);
            // Send outside
            let mut msg = Msg::new();
            msg.action = "forward".into();
            {
                let mut builder = msg.build_schema::<ui_js_create::Builder>();
                builder.borrow().set_name(&comp.name);
                let mut class = builder.borrow().get_class()?.init_list(1);
                class.borrow().get(0).get_first()?.set_text(key);
                class.borrow().get(0).get_second()?.set_bool(value);
            }
            try!(send_action!(comp, output, msg));
        }
        "get_class" => {
            let reader = msg_input.read_schema::<ntup_tuple_tt::Reader>()?;
            let key = reader.get_first()?.get_text()?;
            let value = reader.get_second()?.get_text()?;
            let resp = comp.state.class.get(value).map(|b| b.to_owned()).unwrap_or(false);
            let mut msg = Msg::new();
            {
                let mut builder = msg.build_schema::<prim_bool::Builder>();
                builder.set_bool(resp);
            }
            msg.action = key.to_string();
            let _ = send_action!(comp, output, msg);
        }
        // property
        "set_property" => {
            // Change in state
            let reader = msg_input.read_schema::<ntup_tuple_tt::Reader>()?;
            let key = reader.get_first()?.get_text()?;
            let value = reader.get_second()?.get_text()?;
            comp.state.property.insert(key.into(), value.into());
            // Send outside
            let mut msg = Msg::new();
            msg.action = "forward".into();
            {
                let mut builder = msg.build_schema::<ui_js_create::Builder>();
                builder.borrow().set_name(&comp.name);
                let mut prop = builder.borrow().get_property()?.init_list(1);
                prop.borrow().get(0).get_key()?.set_text(key);
                prop.borrow().get(0).get_val()?.set_text(value);
            }
            try!(send_action!(comp, output, msg));
        }
        "get_property" => {
            let reader = try!(msg_input.read_schema::<ntup_tuple_tt::Reader>());
            let key = reader.get_first()?.get_text()?;
            let value = reader.get_second()?.get_text()?;
            let resp = comp.state.property.get(value).map(|resp| resp.as_str())
                .unwrap_or("");
            let mut msg = Msg::new();
            {
                let mut builder = msg.build_schema::<prim_text::Builder>();
                builder.set_text(resp);
            }
            msg.action = key.to_string();
            let _ = send_action!(comp, output, msg);
        }
        // Content
        "set_text" => {
            let reader = msg_input.read_schema::<prim_text::Reader>()?;
            let new_content = reader.get_text()?;
            // Change in state
            comp.state.text = Some(new_content.to_string());
            // Send new content
            let mut msg = Msg::new();
            msg.action = "forward".to_string();
            {
                let mut builder: ui_js_create::Builder = msg.build_schema();
                builder.borrow().set_name(&comp.name);
                builder.borrow().set_text(new_content);
            }
            send_action!(comp, output, msg);
        }
        "insert_text" => {
            msg_input.action = "forward_create".into();
            {
                let mut builder = try!(msg_input.edit_schema::<ui_js_create::Builder, ui_js_create::Reader>());
                builder.set_append(&comp.name);
            }
            send_action!(comp, output, msg_input);
        }
        "get_text" => {
            let reader = try!(msg_input.read_schema::<prim_text::Reader>());
            let key = try!(reader.get_text());
            let resp = comp.state.text.as_ref().map(|resp| resp.as_str()).unwrap_or("");
            let mut msg = Msg::new();
            {
                let mut builder = msg.build_schema::<prim_text::Builder>();
                builder.set_text(resp);
            }
            msg.action = key.to_string();
            let _ = send_action!(comp, output, msg);
        }
        "input" => {
            {
                let mut reader: prim_text::Reader = try!(msg_input.read_schema());
                comp.state.property.insert("value".into(), try!(reader.get_text()).into());
            }
            let _ = send_action!(comp, output, msg_input);

        }
        "delete" => {
            {
                let mut builder: ui_js_create::Builder = msg_input.build_schema();
                builder.borrow().set_name(&comp.name);
                builder.borrow().set_remove(true);
            }
            msg_input.action = "forward".into();
            let _ = send_action!(comp, output, msg_input);
        }
        _ => { let _ = send_action!(comp, output, msg_input); }
    }

    Ok(())
}
