extern crate capnp;

#[macro_use]
extern crate rustfbp;

use std::thread;

pub struct Portal {
    places: HashMap<String, String>,
    actual: Option<String>,
}

impl Portal {
    fn new() -> Self {
        Portal {
            places: HashMap::new(),
            actual: None,
        }
    }
}

agent! {
    inarr(places: any),
    output(output: any),
    portal(Portal => Portal::new()),
    fn run(&mut self) -> Result<Signal> {
        for (place, recv) in self.inarr.places.iter() {
            let mut msg_place = recv.try_recv();
            if let Ok(mut msg) = msg_place {
                if msg.action == "create" {
                    msg.action = "insert_text".into();
                    insert(&mut msg)?;
                    {
                        let reader = msg.read_schema::<js_create::Reader>()?;
                        self.portal.places.insert(place.into(), reader.get_name()?.get_text()?.into());
                    }
                } else if msg.action == "display" {
                    // Display
                    msg.action = "forward".into();
                    let mut builder = msg.build_schema::<js_create::Builder>();
                    let name = self.portal.places.get(&place).ok_or(result::Error::Misc("Don't get the name".into()))?;
                    builder.borrow().get_name()?.set_text(&name);
                    let mut init = builder.get_style()?.init_list(1);
                    init.borrow().get(0).get_key()?.set_text("display");
                    init.borrow().get(0).get_val()?.set_text("inline");
                    // Hidden if already a visible
                    match self.portal.actual {
                        Some(ref actual) => {
                            let mut msg = Msg::new();
                            msg.action = "forward".into();
                            {
                                let mut builder = msg.build_schema::<js_create::Builder>();
                                let name = try!(self.portal.places.get(actual).ok_or(result::Error::Misc("Don't get the name".into())));
                                builder.borrow().get_name()?.set_text(&name);
                                let mut init = builder.get_style()?.init_list(1);
                                init.borrow().get(0).get_key()?.set_text("display");
                                init.borrow().get(0).get_val()?.set_text("none");
                            }
                            self.output.output.send(msg)?;
                        }
                        _ => {}
                    }
                    // Set the new
                    self.portal.actual = Some(place.into());
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
        let acc: js_create::Reader = msg.read_schema()?;
        let acc_places = acc.get_style()?.get_list()?;
        for i in 0..acc_places.len() {
            let p = acc_places.get(i);
            vec.push((p.get_key()?.get_text()?.into(), p.get_val()?.get_text()?.into()));
        }
    }
    // Add it
    {
        let mut builder = msg.edit_schema::<js_create::Builder, js_create::Reader>()?;
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
