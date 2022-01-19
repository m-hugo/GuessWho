#![allow(unused_imports)]
#![allow(deprecated)]

use fltk::{
    app,
    button::Button,
    enums::{Align, Color, Font, FrameType, Shortcut},
    frame::Frame,
    image, menu,
    prelude::*,
    window::Window,
};
use fltk_theme::{color_themes, ColorTheme};
use fltk_theme::{SchemeType, WidgetScheme};

#[derive(Clone, Copy)]
pub enum Message {
    File(&'static str),
    Question(&'static str, &'static str),
    Switch(usize),
}

fn main() {
    let mut wind = Window::default().with_size(89 * 6, 170 * 4 + 20);
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
            frame[n].set_image(Some(
                image::PngImage::load(format!(
                    "./src/personnages/imageonline-co-split-image-{}.png",
                    n + 1
                ))
                .unwrap(),
            ));
            let mut im = image::PngImage::load(format!(
                "./src/personnages/imageonline-co-split-image-{}.png",
                n + 1
            ))
            .unwrap();
            im.inactive();
            frame[n].set_deimage(Some(im));
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

    menu.add_emit(
        "Charge Fichier/Juste Léon",
        Shortcut::None,
        menu::MenuFlag::Normal,
        s,
        Message::File("Juste Léon"),
    );
    menu.add_emit(
        "Charge Fichier/Sam & Léons",
        Shortcut::None,
        menu::MenuFlag::Normal,
        s,
        Message::File("Sam & Léons"),
    );
    menu.add_emit(
        "Question/Nom/Samuel",
        Shortcut::None,
        menu::MenuFlag::Normal,
        s,
        Message::Question("Nom", "Samuel"),
    );

    wind.make_resizable(false);
    wind.end();
    wind.show();

    while a.wait() {
        use Message::*;
        if let Some(msg) = r.recv() {
            match msg {
                File(txt) => {}
                Question(cat, txt) => res.set_label(
                    &(cat.to_owned()
                        + " est "
                        + txt
                        + " ? "
                        + match txt {
                            "vegetaux" => "delicieux",
                            "Homme" => "Non",
                            "Femme" | "Samuel" => "Oui",
                            "Bleus" => "Non",
                            "Léon" => "Non",
                            x => x,
                        }),
                ),
                Message::Switch(n) => {
                    if frame[n].active() {
                        frame[n].deactivate()
                    } else {
                        frame[n].activate()
                    }
                }
            }
        }
    }
}
