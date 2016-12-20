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
                        let acc: js_create::Reader = try!(ip.read_schema());
                        let acc_places = try!(acc.get_style());
                        for i in 0..acc_places.len() {
                            let p = acc_places.get(i);
                            vec.push((try!(p.get_key()).into(), try!(p.get_val()).into()));
                        }
                    }
                    // Add it
                    {
                        let mut builder = try!(ip.edit_schema::<js_create::Builder, js_create::Reader>());
                        let mut init = builder.init_style((vec.len() + 1) as u32);
                        let mut i = 0;
                        for p in vec {
                            init.borrow().get(i).set_key(&p.0);
                            init.borrow().get(i).set_val(&p.1);
                            i += 1;
                        }
                        init.borrow().get(i).set_key("order");
                        init.borrow().get(i).set_val(place);
                    }
                }
                try!(self.output.output.send(ip));
            }
        }

        Ok(End)
    }
}
