extern crate capnp;

#[macro_use]
extern crate rustfbp;

use std::thread;

agent! {
    inarr(places: any),
    output(output: any),
    fn run(&mut self) -> Result<Signal> {
        for (place, recv) in self.inarr.places.iter() {
            let mut ip_place = recv.try_recv();
            if let Ok(mut ip) = ip_place {
                if ip.action == "create" {
                    ip.action = "insert_text".into();
                    // Add the css
                    let mut vec: Vec<(String, String)> = vec![];
                    {
                        let acc: ui_js_create::Reader = try!(ip.read_schema());
                        let acc_places = acc.get_style()?.get_list()?;
                        for i in 0..acc_places.len() {
                            let p = acc_places.get(i);
                            vec.push((p.get_key()?.get_text()?.into(), p.get_val()?.get_text()?.into()));
                        }
                    }
                    // Add it
                    {
                        let mut builder = try!(ip.edit_schema::<ui_js_create::Builder, ui_js_create::Reader>());
                        let mut init = builder.get_style()?.init_list((vec.len() + 1) as u32);
                        let mut i = 0;
                        for p in vec {
                            init.borrow().get(i).get_key()?.set_text(&p.0);
                            init.borrow().get(i).get_val()?.set_text(&p.1);
                            i += 1;
                        }
                        init.borrow().get(i).get_key()?.set_text("order");
                        init.borrow().get(i).get_val()?.set_text(place);
                    }
                }
                try!(self.output.output.send(ip));
            }
        }

        Ok(End)
    }
}
