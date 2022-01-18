#![allow(unused_imports)]
#![allow(deprecated)]

use fltk::{app, image, menu, button::Button, frame::Frame, prelude::*, window::Window, enums::{Align, Color, Font, FrameType},};
use fltk_theme::{WidgetScheme, SchemeType};
use fltk_theme::{ColorTheme, color_themes};

fn main() {
    let app = app::App::default();//.with_scheme(app::Scheme::Plastic);
    //let widget_scheme = WidgetScheme::new(SchemeType::Aqua);
    //widget_scheme.apply();
    //let theme = ColorTheme::new(color_themes::SHAKE_THEME);
	//theme.apply();
    let mut wind = Window::default().with_size(89*6, 170*4+20);
    
	let mut frame: [Frame; 24]= unsafe { ::std::mem::uninitialized() };
	let mut b: [Button; 24]= unsafe { ::std::mem::uninitialized() };
	//let mut rb: [Button; 24]= unsafe { ::std::mem::uninitialized() };
	let mut n:usize=0;
	for j in 0..4 {
		for i in 0..6 {
			frame[n] = Frame::new(90*i, 170*j, 89, 146, None);
			frame[n].set_image(Some(image::PngImage::load(format!("./src/personnages/imageonline-co-split-image-{}.png", n+1)).unwrap()));
			let mut im = image::PngImage::load(format!("./src/personnages/imageonline-co-split-image-{}.png", n+1)).unwrap();
			im.inactive();
			frame[n].set_deimage(Some(im));
			//frame[n].draw(move |f| {image::PngImage::load(format!("/home/hugo/Downloads/Exemples dimages-20220117/personnages/imageonline-co-split-image-{}.png", n+1)).unwrap().draw(f.x(), f.y(), f.w(), f.h());});
			b[n] = Button::new(90*i+2, 170*j+148, 89, 20, "Renverse");
			b[n].set_color(Color::from_hex(0x42A5F5));
			//b[n] = Button::new(90*i+3, 170*j+149, 80, 20, "Renverse");
			//rb[n] = Button::new(90*i+83, 170*j+149, 9, 20, "↻");
			n+=1;
    	}
    }
    let mut res = Frame::new(0, 170*4, 89*6, 20, None);
    //res.set_color(Color::Red); marche pas utiliser app::background(255, 100, 100); mais rouge sous bouttons
    app::background(226, 208, 177);
    res.set_label_color(Color::Red);
    
    
    let mut menu = menu::MenuButton::default()
        .size_of(&wind)
        .center_of(&wind)
        .with_type(menu::MenuButtonType::Popup3);
    //menu.set_color(Color::from_hex(0xFFFFFF));
    menu.add_choice("Charge Fichier/Animaux|Charge Fichier/vegetaux|Question/Nom/Samuel|Question/Nom/Léon|Question/Sexe/Homme|Question/Sexe/Femme|Question/Yeux/Bleus|3rd menu item"
    );
    menu.set_label_color(Color::White);
    menu.set_color(Color::from_hex(0x42A5F5));
    menu.set_selection_color( Color::from_hex(0x2196F3));
    menu.set_callback(move |m| match m.choice().unwrap().as_str() {
    	"vegetaux" => {println!("delicieux")},
    	"Homme" => {res.set_label("Non")},
    	"Femme" => {res.set_label("Oui")},
    	"Bleus" => {res.set_label("Non")},
    	"Samuel" => {res.set_label("Oui")},
    	"Léon" => {res.set_label("Non")},
    	x => println!("{}", x),
    });
    
    wind.make_resizable(false);
    wind.end();
    wind.show();
    
    let mut n:usize=0;
    for mut x in frame {
    	b[n].set_callback( move |_| if x.active() {x.deactivate()} else {x.activate()});
    	//rb[n].set_callback( move |_| x.activate());
    	n+=1;
    }
    app.run().unwrap();
}
