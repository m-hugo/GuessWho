//#![allow(unused_imports)]
//cargo clippy -- -W clippy::pedantic -A clippy::many-single-char-names -A clippy::unreadable-literal
#![allow(deprecated)]
//mod parsing;
use fltk::app::Sender;
use fltk::{
    app,
    button::{Button, CheckButton},
    enums::{Color, Shortcut},
    frame::Frame,
    image, menu,
    prelude::*,
    window::{OverlayWindow, Window},
};
//use fltk_theme::{color_themes, ColorTheme};
//use fltk_theme::{SchemeType, WidgetScheme};

use std::fs;
use std::io;

use rand::Rng;

use serde_json::Value;

#[derive(Clone, Copy)]
pub enum Message {
    Question(&'static str),
    Switch(usize),
    Valider,
    Poweroff,
    Sauvegarde,
    Change,
    Etale(usize),
}

fn quitter(s: Sender<Message>) {
    let mut windpop = OverlayWindow::default()
        .with_size(400, 90)
        .center_screen()
        .with_label("Au Revoir");
    let mut framepop = Frame::new(0, 0, 400, 60, "Voulez-vous quitter ?");
    framepop.set_label_color(Color::White);

    let mut bouisans = Button::default();
    bouisans.set_pos(0, 60);
    bouisans.set_label("Quitter");
    bouisans.set_size(133, 30);
    bouisans.set_label_color(Color::White);
    bouisans.set_color(Color::from_hex(0x42A5F5));
    bouisans.emit(s, Message::Poweroff);

    let mut bouisauv = Button::default();
    bouisauv.set_pos(134, 60);
    bouisauv.set_label("Sauvegarder");
    bouisauv.set_size(133, 30);
    bouisauv.set_label_color(Color::White);
    bouisauv.set_color(Color::from_hex(0x42A5F5));
    bouisauv.emit(s, Message::Sauvegarde);

    let mut rester = Button::default();
    rester.set_pos(267, 60);
    rester.set_label("Rester");
    rester.set_size(133, 30);
    rester.set_label_color(Color::White);
    rester.set_color(Color::from_hex(0x42A5F5));

    windpop.make_resizable(false);
    windpop.end();
    windpop.show();

    rester.set_callback(move |_| fltk::prelude::WidgetBase::delete(windpop.clone()));
}

fn etalepile(frame: &mut [Frame; 24], val: &mut Value, b: &mut [Button; 24]) {
    for n in 0..24 {
        b[n].show();
        frame[n].show();
        if let Some(vv) = val["liste"][n]["image"].as_str() {
            frame[n].set_image(Some(image::PngImage::load(vv.to_string()).unwrap()));
            let mut im = image::PngImage::load(vv.to_string()).unwrap();
            im.inactive();
            frame[n].set_deimage(Some(im));
            frame[n].redraw();
        }
    }
}

fn toggle(frame: &mut Frame) {
    if frame.active() {
        frame.deactivate();
    } else {
        frame.activate();
    }
}

fn hidevec(p: &mut Vec<[impl fltk::prelude::WidgetExt; 24]>) {
    for k in p {
        hide(k);
    }
}

fn hide(k: &mut [impl fltk::prelude::WidgetExt; 24]) {
    for fr in k {
        fr.hide();
    }
}

fn tryfile(f: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let file = fs::File::open(f)?; //txt
    let reader = io::BufReader::new(file);
    let v: Value = serde_json::from_reader(reader)?;
    Ok(v)
}

fn fnerror(e: &'static str) {
    let mut windpoperr = OverlayWindow::default()
        .with_size(400, 60)
        .center_screen()
        .with_label("Erreur");
    let mut framepop = Frame::new(0, 0, 400, 60, e);
    framepop.set_label_color(Color::White);
    windpoperr.make_resizable(false);
    windpoperr.end();
    windpoperr.show();
}

fn showdecks(
    piles: &mut Vec<[Button; 24]>,
    addr: &[Value],
) -> Result<(), Box<dyn std::error::Error>> {
    for k in 0..addr.len() {
        for n in 0..24 {
            if let Some(vv) = addr[k]["liste"][n]["image"].as_str() {
                piles[k][n].set_image(Some(image::PngImage::load(vv.to_string())?));
                piles[k][n].show();
                piles[k][n].redraw();
            } else {
                piles[k][0].set_label("Error");
                println!("image error");
            }
        }
    }
    Ok(())
}

fn loadquestions(
    menu: &mut menu::MenuButton,
    s: Sender<Message>,
    v: &Value,
) -> std::option::Option<()> {
    //let linoms = v["attrs"]["nom"].as_array()?;
    //let nom = linoms[rand::thread_rng().gen_range(0..linoms.len())];
    //println!("{}", nom);
    //Nom d'un personnage choisi au hasard

    let linoms = v["attrs"]["nom"].as_array()?;
    let id = rand::thread_rng().gen_range(0..linoms.len());
    let perso = &v["liste"].as_array()?[id];

    let _ = menu.clear_submenu(menu.find_index("Question"));

    let g: Value = serde_json::from_str(&perso.to_string()).unwrap();
    for x in v["attrs"].as_object()? {
        for y in x.1.as_array()? {
            let txt = y.as_str()?;
            menu.add_emit(
                &("Question/".to_owned() + x.0 + "/" + &y.as_str()?.to_owned()),
                Shortcut::None,
                menu::MenuFlag::Radio,
                s,
                Message::Question(Box::leak(
                    (x.0.to_string()
                        + " est "
                        + txt
                        + " ? "
                        + if g[x.0] == txt { "Vrai" } else { "Faux" })
                    .into_boxed_str(),
                )),
            );
        }
    }
    Some(())
}

fn main() {
    let a = app::App::default(); //.with_scheme(app::Scheme::Plastic);
    app::background(0, 0, 0); //app::background(226, 208, 177);
                              //res.set_color(Color::Red); marche pas utiliser app::background(255, 100, 100); mais rouge sous bouttons
                              //app::set_font_size(20);
                              //let widget_scheme = WidgetScheme::new(SchemeType::Aqua);
                              //widget_scheme.apply();
                              //let theme = ColorTheme::new(color_themes::SHAKE_THEME);
                              //theme.apply();

    let (s, r) = app::channel::<Message>();

    let mut wind = Window::default()
        .with_size(89 * 6, 170 * 4 + 30 + 30)
        .center_screen()
        .with_label("Qui est-ce ?");

    let mut n: usize = 0;
    let mut frame: [Frame; 24] = unsafe { ::std::mem::uninitialized() };
    let mut b: [Button; 24] = unsafe { ::std::mem::uninitialized() };
    for j in 0..4 {
        for i in 0..6 {
            frame[n] = Frame::new(90 * i, 170 * j + 30, 89, 146, None);
            b[n] = Button::new(90 * i + 2, 170 * j + 148 + 30, 89, 20, "Renverse");
            b[n].set_color(Color::from_hex(0x42A5F5));
            b[n].emit(s, Message::Switch(n));
            b[n].hide();
            n += 1;
        }
    }

    let mut n: usize = 0;
    let mut piles: Vec<[Button; 24]> = vec![];
    for k in 0..4 {
        let mut pile: [Button; 24] = unsafe { ::std::mem::uninitialized() };
        for i in 0..24 {
            pile[n] = Button::new(i + 90 / (24 - i), 170 * k + 30, 89, 146, None);
            pile[n].emit(s, Message::Etale(k.try_into().unwrap()));
            pile[n].hide();
            n += 1;
        }
        n = 0;
        piles.push(pile);
    }

    let mut res = Frame::new(0, 170 * 4 + 30, 89 * 6, 30, None);
    res.set_label_color(Color::White);

    let mut menu = menu::MenuButton::default();
    menu.set_pos(0, 0);
    menu.set_label("Menu");
    menu.set_size(90, 30);
    menu.set_color(Color::from_hex(0x42A5F5));
    menu.set_selection_color(Color::from_hex(0x2196F3));

    let mut check = CheckButton::default();
    check.set_pos(89 * 6 - 90, 0);
    check.set_label("Triche");
    check.set_size(90, 30);
    check.set_label_color(Color::White);

    let mut vali = Button::default();
    vali.set_pos(91, 0);
    vali.set_label("Valider");
    vali.set_size(90, 30);
    vali.emit(s, Message::Valider);
    vali.set_color(Color::from_hex(0x42A5F5));

    let mut pilesaddr: Vec<Value> = vec![];
    for file in fs::read_dir("./").unwrap() {
        let u = file.as_ref().unwrap().file_name();
        if u.to_str().unwrap().contains('.') {
            if u.to_str().unwrap().split('.').nth(1).unwrap() == "json" {
                if let Ok(v) = tryfile(&u.into_string().unwrap()) {
                    pilesaddr.push(v);
                }
            }
        }
    }

    menu.add(
        "Quitter",
        Shortcut::None,
        menu::MenuFlag::Normal,
        move |_| quitter(s),
    );

    menu.add_emit(
        "Change Cartes",
        Shortcut::None,
        menu::MenuFlag::Normal,
        s,
        Message::Change,
    );

    wind.make_resizable(false);
    wind.end();
    wind.show();

    s.send(Message::Change);
    //fnerror("test");
    while a.wait() {
        use Message::*;
        if let Some(msg) = r.recv() {
            match msg {
                Question(txt) => res.set_label(txt),
                Switch(n) => toggle(&mut frame[n]),
                Valider | Sauvegarde => {}
                Etale(num) => {
                    etalepile(&mut frame, &mut pilesaddr[num], &mut b);
                    hidevec(&mut piles);
                    if loadquestions(&mut menu, s, &pilesaddr[num]).is_none() {
                        fnerror("json incorrect");
                    }
                }
                Change => {
                    menu.remove(menu.find_index("Question"));
                    hide(&mut b);
                    hide(&mut frame);
                    if let Err(e) = showdecks(&mut piles, &pilesaddr) {
                        println!("{:?}", e);
                        fnerror("Les cartes ne peuvent pas être affichées, voir terminal");
                    }
                }
                Poweroff => break,
            }
        }
    }
}
