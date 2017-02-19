extern crate capnp;

#[macro_use]
extern crate rustfbp;

use std::thread;

pub struct State {
    places: HashMap<String, String>,
    actual: Option<String>,
}

impl State {
    fn new() -> Self {
        State {
            places: HashMap::new(),
            actual: None,
        }
    }
}

agent! {
    inarr(places: any),
    output(output: any),
    state(State => State::new()),
    fn run(&mut self) -> Result<Signal> {
        for (place, recv) in self.inarr.places.iter() {
            let mut msg_place = recv.try_recv();
            if let Ok(mut msg) = msg_place {
                if msg.action == "create" {
                    msg.action = "insert_text".into();
                    insert(&mut msg)?;
                    {
                        let reader = msg.read_schema::<ui_js_create::Reader>()?;
                        self.state.places.insert(place.into(), reader.get_name()?.into());
                    }
                } else if msg.action == "display" {
                    // Display
                    msg.action = "forward".into();
                    let mut builder = msg.build_schema::<ui_js_create::Builder>();
                    let name = self.state.places.get(&place).ok_or(result::Error::Misc("Don't get the name".into()))?;
                    builder.borrow().set_name(&name);
                    let mut init = builder.get_style()?.init_list(1);
                    init.borrow().get(0).get_key()?.set_text("display");
                    init.borrow().get(0).get_val()?.set_text("inline");
                    // Hidden if already a visible
                    match self.state.actual {
                        Some(ref actual) => {
                            let mut msg = Msg::new();
                            msg.action = "forward".into();
                            {
                                let mut builder = msg.build_schema::<ui_js_create::Builder>();
                                let name = try!(self.state.places.get(actual).ok_or(result::Error::Misc("Don't get the name".into())));
                                builder.borrow().set_name(&name);
                                let mut init = builder.get_style()?.init_list(1);
                                init.borrow().get(0).get_key()?.set_text("display");
                                init.borrow().get(0).get_val()?.set_text("none");
                            }
                            self.output.output.send(msg)?;
                        }
                        _ => {}
                    }
                    // Set the new
                    self.state.actual = Some(place.into());
                }
                self.output.output.send(msg)?;
            }
        }

        Ok(End)
    }
}

fn insert(mut msg: &mut Msg) -> Result<()> {
    let mut vec: Vec<(String, String)> = vec![];
    {
        let acc: ui_js_create::Reader = msg.read_schema()?;
        let acc_places = acc.get_style()?.get_list()?;
        for i in 0..acc_places.len() {
            let p = acc_places.get(i);
            vec.push((p.get_key()?.get_text()?.into(), p.get_val()?.get_text()?.into()));
        }
    }
    // Add it
    {
        let mut builder = msg.edit_schema::<ui_js_create::Builder, ui_js_create::Reader>()?;
        let mut init = builder.get_style()?.init_list((vec.len() + 1) as u32);
        let mut i = 0;
        for p in vec {
            init.borrow().get(i).get_key()?.set_text(&p.0);
            init.borrow().get(i).get_val()?.set_text(&p.1);
            i += 1;
        }
        init.borrow().get(i).get_key()?.set_text("display");
        init.borrow().get(i).get_val()?.set_text("none");
    }
    Ok(())
}
