//#![allow(unused_imports)]
#![allow(deprecated)]
//mod parsing;
use fltk::app::Sender;
use fltk::{
    app,
    button::Button,
    enums::{Color, Shortcut},
    frame::Frame,
    image, menu,
    prelude::*,
    window::Window,
};
//use fltk_theme::{color_themes, ColorTheme};
//use fltk_theme::{SchemeType, WidgetScheme};

use std::fs;
use std::io;

use rand::Rng;

use serde_json::Value;

#[derive(Clone, Copy)]
pub enum Message {
    File(&'static str),
    Question(&'static str, &'static str, &'static str),
    Switch(usize),
    TxtError(&'static str),
    //TrueError(Box<dyn std::error::Error>),
}

fn loadfile(
    frame: &mut [Frame; 24],
    txt: &'static str,
) -> Result<Value, Box<dyn std::error::Error>> {
    let file = fs::File::open(txt)?; //txt
    let reader = io::BufReader::new(file);
    let v: Value = serde_json::from_reader(reader)?;
    for n in 0..24 {
        if let Some(vv) = v["liste"][n]["image"].as_str() {
            //println!("{}", vv);
            frame[n].set_image(Some(image::PngImage::load(vv.to_string())?));
            let mut im = image::PngImage::load(vv.to_string())?;
            im.inactive();
            frame[n].set_deimage(Some(im));
            frame[n].redraw();
        }
    }
    Ok(v)
}
fn loadquestions(
    menu: &mut menu::MenuButton,
    s: Sender<Message>,
    v: Value,
) -> std::option::Option<()> {
    //let linoms = v["attrs"]["nom"].as_array()?;
    //let nom = linoms[rand::thread_rng().gen_range(0..linoms.len())];
    //println!("{}", nom);
    //Nom d'un personnage choisi au hasard

    let linoms = v["attrs"]["nom"].as_array()?;
    let id = rand::thread_rng().gen_range(0..linoms.len());
    let perso = &v["liste"].as_array()?[id];
    //println!("{}", perso);

    let _ = menu.clear_submenu(menu.find_index("Question"));
    //println!("{}", v["attrs"]);
    for x in v["attrs"].as_object()? {
        //"Question/Nom/Samuel"
        //println!("{}", x.1);
        for y in x.1.as_array()? {
            menu.add_emit(
                &("Question/".to_owned() + x.0 + "/" + &y.as_str()?.to_owned()),
                Shortcut::None,
                menu::MenuFlag::Normal,
                s,
                Message::Question(
                    Box::leak(x.0.to_string().into_boxed_str()),
                    Box::leak(y.as_str()?.to_string().into_boxed_str()),
                    Box::leak(perso.to_string().into_boxed_str()),
                ),
            );
        }
    }

    Some(())
}

fn main() {
    let a = app::App::default(); //.with_scheme(app::Scheme::Plastic);
    app::background(226, 208, 177);
    //res.set_color(Color::Red); marche pas utiliser app::background(255, 100, 100); mais rouge sous bouttons
    //app::set_font_size(20);

    //let widget_scheme = WidgetScheme::new(SchemeType::Aqua);
    //widget_scheme.apply();
    //let theme = ColorTheme::new(color_themes::SHAKE_THEME);
    //theme.apply();

    let (s, r) = app::channel::<Message>();
    //let (s2, r2) = app::channel::<Message>();

    let mut wind = Window::default()
        .with_size(89 * 6, 170 * 4 + 20)
        .center_screen()
        .with_label("Click droit pour commencer");

    let mut frame: [Frame; 24] = unsafe { ::std::mem::uninitialized() };

    let mut b: [Button; 24] = unsafe { ::std::mem::uninitialized() };

    //let mut rb: [Button; 24]= unsafe { ::std::mem::uninitialized() };

    let mut n: usize = 0;
    for j in 0..4 {
        for i in 0..6 {
            frame[n] = Frame::new(90 * i, 170 * j, 89, 146, None);
            //frame[n].draw(move |f| {image::PngImage::load(format!("/home/hugo/Downloads/Exemples dimages-20220117/personnages/imageonline-co-split-image-{}.png", n+1)).unwrap().draw(f.x(), f.y(), f.w(), f.h());});
            b[n] = Button::new(90 * i + 2, 170 * j + 148, 89, 20, "Renverse");
            b[n].set_color(Color::from_hex(0x42A5F5));
            b[n].emit(s, Message::Switch(n));
            //b[n] = Button::new(90*i+3, 170*j+149, 80, 20, "Renverse");
            //rb[n] = Button::new(90*i+83, 170*j+149, 9, 20, "↻");
            n += 1;
        }
    }

    let mut res = Frame::new(0, 170 * 4, 89 * 6, 20, None);
    res.set_label_color(Color::Red);

    let mut menu = menu::MenuButton::default()
        .size_of(&wind)
        .center_of(&wind)
        .with_type(menu::MenuButtonType::Popup3);
    menu.set_label_color(Color::White);
    menu.set_color(Color::from_hex(0x42A5F5));
    menu.set_selection_color(Color::from_hex(0x2196F3));

    for file in fs::read_dir("./").unwrap() {
        let u = file.as_ref().unwrap().file_name();
        if u.to_str().unwrap().contains('.') {
            //println!("{}", u.to_str().unwrap());
            //println!("{}", u.to_str().unwrap().split(".").nth(1).unwrap());
            if u.to_str().unwrap().split('.').nth(1).unwrap() == "json" {
                let x = Box::leak(u.into_string().unwrap().into_boxed_str());
                menu.add_emit(
                    &("Charge Fichier/".to_owned() + x),
                    Shortcut::None,
                    menu::MenuFlag::Normal,
                    s,
                    Message::File(x),
                );
            }
        }
    }

    wind.make_resizable(false);
    wind.end();
    wind.show();

    while a.wait() {
        use Message::*;
        if let Some(msg) = r.recv() {
            match msg {
                File(txt) => {
                    match loadfile(&mut frame, txt) {
                        Err(e) => {
                            s.send(Message::TxtError(Box::leak(e.to_string().into_boxed_str())));
                        }
                        Ok(v) => {
                            if loadquestions(&mut menu, s, v).is_none() {
                                s.send(Message::TxtError("json incorrect")); //
                            }
                        }
                    }
                }
                Question(cat, txt, perso) => {
                    let g: Value = serde_json::from_str(perso).unwrap();
                    //g.as_object().unwrap()[txt].as_object().unwrap().1.as_str().unwrap()
                    //println!("{:?}", txt);
                    //println!("{:?}", g[cat]==txt);//.as_object().unwrap()
                    res.set_label(
                        &(cat.to_owned()
                            + " est "
                            + txt
                            + " ? "
                            + if g[cat] == txt { "Vrai" } else { "Faux" }),
                    )
                }
                Switch(n) => {
                    if frame[n].active() {
                        frame[n].deactivate()
                    } else {
                        frame[n].activate()
                    }
                }
                TxtError(txt) => {
                    res.set_label(txt);
                    //app::background(255, 100, 100);
                    //app::redraw();
                    //thread::sleep(time::Duration::from_secs(5));//let _=app::wait_for(5.0);//thread::wait(time::Duration::from_secs(5));
                    //app::background(226, 208, 177);
                    //app::redraw();
                } /*TrueError(e) => {
                      res.set_label(e.to_string());
                  }*/
            }
        }
    }
}
