//#![allow(unused_imports)]
//cargo clippy -- -W clippy::pedantic -W clippy::nursery -A clippy::many-single-char-names -A clippy::similar-names -A clippy::unreadable-literal -A clippy::enum-glob-use -W absolute-paths-not-starting-with-crate -W deprecated-in-future -W elided-lifetimes-in-paths -W explicit-outlives-requirements -W keyword-idents -W macro-use-extern-crate -W meta-variable-misuse -W missing-abi -W missing-copy-implementations  -W non-ascii-idents -W noop-method-call -W pointer-structural-match -W rust-2021-incompatible-closure-captures -W rust-2021-incompatible-or-patterns -W rust-2021-prefixes-incompatible-syntax -W rust-2021-prelude-collisions -W single-use-lifetimes -W trivial-casts -W trivial-numeric-casts -W unreachable-pub -W unsafe-op-in-unsafe-fn -W unused-crate-dependencies -W unused-extern-crates -W unused-import-braces -W unused-lifetimes -W unused-qualifications -W variant-size-differences
//#![allow(deprecated)]
//mod parsing;
use fltk::app::Sender;
use fltk::{
	app,
	button::{Button, CheckButton},
	enums::{Color, Shortcut},
	frame::Frame,
	image::PngImage,
	menu,
	prelude::*,
	window::{OverlayWindow, Window},
};
use fltk_theme as _;
//use fltk_theme::{color_themes, ColorTheme};
//use fltk_theme::{SchemeType, WidgetScheme};

use std::fs;
use std::io::BufReader;

use rand::Rng;

use serde_json::Value;

#[derive(Clone, Copy)]
pub enum Message {
	//Question(usize),
	Switch(usize),
	Valider,
	Poweroff,
	Sauvegarde,
	Change,
	Triche,
	Compte,
	Etale(usize),
	Mode(MODE),
}

#[derive(Clone, Copy)]
pub enum MODE {
	ET,
	OR,
	XOR,
}

fn getaddrs() -> Vec<Value> {
	let mut addrs: Vec<Value> = vec![];
	for file in fs::read_dir("./").unwrap() {
		let u = file.as_ref().unwrap().file_name();
		if u.to_str().unwrap().contains('.') && u.to_str().unwrap().split('.').nth(1).unwrap() == "json" {
			if let Ok(v) = tryfile(&u.into_string().unwrap()) {
				addrs.push(v);
			}
		}
	}
	addrs
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

	rester.set_callback(move |_| WidgetBase::delete(windpop.clone()));
}

fn etalepile(frame: &mut Vec<Frame>, val: &mut Value, b: &mut Vec<Button>) {
	for n in 0..24 {
		b[n].show();
		frame[n].show();
		if let Some(vv) = val["liste"][n]["image"].as_str() {
			frame[n].set_image(Some(PngImage::load(vv.to_string()).unwrap())); //deja testé dans showdecks()
			let mut im = PngImage::load(vv.to_string()).unwrap(); //idem
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

fn hidevec(p: &mut Vec<Vec<impl WidgetExt>>) {
	for k in p {
		hide(k);
	}
}

fn hide(k: &mut Vec<impl WidgetExt>) {
	for fr in k {
		fr.hide();
	}
}

fn activate(k: &mut Vec<impl WidgetExt>) {
	for fr in k {
		fr.activate();
	}
}

fn tryfile(f: &str) -> Result<Value, Box<dyn std::error::Error>> {
	let file = fs::File::open(f)?; //txt
	let reader = BufReader::new(file);
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

fn showdecks(piles: &mut Vec<Vec<Button>>, addr: &[Value]) -> Result<(), Box<dyn std::error::Error>> {
	for k in 0..addr.len() {
		for n in 0..24 {
			if let Some(vv) = addr[k]["liste"][n]["image"].as_str() {
				piles[k][n].set_image(Some(PngImage::load(vv.to_string())?));
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

fn loadquestions(menu: &mut menu::MenuButton, s: Sender<Message>, v: &Value) -> Option<(Vec<String>, Vec<bool>)> {
	//let linoms = v["attrs"]["nom"].as_array()?;
	//let nom = linoms[rand::thread_rng().gen_range(0..linoms.len())];
	//println!("{}", nom);
	//Nom d'un personnage choisi au hasard

	let linoms = v["attrs"]["nom"].as_array()?;
	let id = rand::thread_rng().gen_range(0..linoms.len());
	let perso = &v["liste"].as_array()?[id];
	let _pasgrave = menu.clear_submenu(menu.find_index("Question"));

	let mut qli: Vec<String> = vec![];
	let mut rli: Vec<bool> = vec![];
	let g: Value = serde_json::from_str(&perso.to_string()).unwrap();
	for x in v["attrs"].as_object()? {
		for y in x.1.as_array()? {
			let txt = y.as_str()?;
			menu.add(
				&("Question/".to_owned() + x.0 + "/" + &y.as_str()?.to_owned()),
				Shortcut::None,
				menu::MenuFlag::Radio,
				|_| (), //s, Message::Question(60),
			);
			qli.push("Question/".to_owned() + x.0 + "/" + &y.as_str()?.to_owned());
			rli.push(g[x.0] == txt);
		}
	}

	menu.add_emit(
		"Question/Mode \"et\"",
		Shortcut::None,
		menu::MenuFlag::Radio,
		s,
		Message::Mode(MODE::ET),
	);
	menu.find_item("Question/Mode \"et\"").unwrap().set();
	menu.add_emit(
		"Question/Mode \"ou incl\"",
		Shortcut::None,
		menu::MenuFlag::Radio,
		s,
		Message::Mode(MODE::OR),
	);
	menu.add_emit(
		"Question/Mode \"ou excl\"",
		Shortcut::None,
		menu::MenuFlag::Radio,
		s,
		Message::Mode(MODE::XOR),
	);

	Some((qli, rli))
}

fn mainmaker(s: Sender<Message>) -> (Vec<Frame>, Vec<Button>, Vec<Vec<Button>>, Button) {
	let mut frame: Vec<Frame> = vec![];
	let mut b: Vec<Button> = vec![];
	for j in 0..4 {
		for i in 0..6 {
			frame.push(Frame::new(90 * i, 170 * j + 30, 89, 146, None));
			let mut tmpb = Button::new(90 * i + 2, 170 * j + 148 + 30, 89, 20, "Renverse");
			tmpb.set_color(Color::from_hex(0x3cf2fc)); //0x42A5F5
			tmpb.emit(s, Message::Switch(b.len()));
			tmpb.hide();
			b.push(tmpb);
		}
	}

	let mut piles: Vec<Vec<Button>> = vec![];
	for k in 0..4 {
		let mut pile: Vec<Button> = vec![];
		for i in 0..24 {
			let mut tmppile = Button::new(i + 90 / (24 - i), 170 * k + 30, 89, 146, None);
			tmppile.emit(s, Message::Etale(k.try_into().unwrap())); //trop petit pour overflow
			tmppile.hide();
			pile.push(tmppile);
		}
		piles.push(pile);
	}

	let mut charge = Button::new(200, 600, 120, 40, "Charge Partie");
	charge.set_color(Color::from_hex(0x42A5F5));

	(frame, b, piles, charge)
}

fn menumaker(s: Sender<Message>) -> (menu::MenuButton, CheckButton, Button, Button) {
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
	check.emit(s, Message::Triche);

	let mut vali = Button::default();
	vali.set_pos(91, 0);
	vali.set_label("Valider");
	vali.set_size(90, 30);
	vali.emit(s, Message::Valider);
	vali.set_color(Color::from_hex(0x42A5F5));

	let mut compte = Button::default();
	compte.set_pos(181, 0);
	compte.set_label("Compte");
	compte.set_size(90, 30);
	compte.emit(s, Message::Compte);
	compte.set_color(Color::from_hex(0x42A5F5));
	compte.deactivate();

	menu.add("Quitter", Shortcut::None, menu::MenuFlag::Normal, move |_| quitter(s));

	menu.add_emit("Change Cartes", Shortcut::None, menu::MenuFlag::Normal, s, Message::Change);

	(menu, check, vali, compte)
}
fn comm(
	first: bool,
	monaddr: &mut Value,
	rli: &mut Vec<bool>,
	frame: &mut Vec<Frame>,
	mode: &mut MODE,
	menu: &mut menu::MenuButton,
) -> u64 {
	let mut nu: u64 = 0;
	let mut numframe = 0_usize;
	let persos = monaddr["liste"].as_array().unwrap();
	for unpers in persos {
		if numframe < frame.len() && frame[numframe].active() {
			let mut nbsame = 0_u64;
			let mut nbvrai = 0_u64;
			let mut toussame = true;
			let mut tousvrai = true;
			let mut n = 0_usize;
			for x in monaddr["attrs"].as_object().unwrap() {
				for y in x.1.as_array().unwrap() {
					let txt = y.as_str().unwrap();
					let cho = menu
						.find_item(&("Question/".to_owned() + x.0 + "/" + &y.as_str().unwrap().to_owned()))
						.unwrap();
					if cho.is_radio() && cho.value() {
						if rli[n] {
							nbvrai += 1;
						} else {
							tousvrai = false;
						}
						if unpers[x.0] == txt {
							nbsame += 1;
						} else {
							toussame = false;
						}
					}
					n += 1;
				}
			}
			if !match mode {
				MODE::ET => (!toussame || tousvrai) && (toussame || !tousvrai), //et
				MODE::OR => (nbsame > 0 && nbvrai > 0) || (nbvrai == 0 && nbsame == 0), //ou incl
				MODE::XOR => (nbsame == 1 && nbvrai == 1) || (nbvrai == 0 && nbsame == 0), //(nbsame == 1 && tousvrai) || (!tousvrai), //ou excl
			} {
				if first {
					nu += 1;
				} else {
					frame[numframe].deactivate();
				}
			}

			numframe += 1;
		}
	}
	nu
}
fn valide(qli: &mut Vec<String>, rli: &mut Vec<bool>, menu: &mut menu::MenuButton, mode: &mut MODE) -> bool {
	let mut nb = 0_u64;
	let mut tous = true;
	for n in 0..qli.len() {
		let cho = menu.find_item(&qli[n]).unwrap();
		if cho.is_radio() && cho.value() {
			println!("{} {}", qli[n], if rli[n] { "Vrai" } else { "Faux" });
			if rli[n] {
				nb += 1;
			//println!("test");//cho.label().unwrap()
			} else {
				tous = false;
			}
		}
	}
	for el in qli {
		menu.find_item(el).unwrap().clear();
	}
	match mode {
		MODE::ET => tous,     //et
		MODE::OR => nb > 0,   //ou incl
		MODE::XOR => nb == 1, //ou excl
	}
}

fn main() {
	let a = app::App::default(); //.with_scheme(app::Scheme::Plastic);
	app::background(0, 0, 0); //app::background(19, 33, 221); //app::background(226, 208, 177);
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

	let (mut frame, mut b, mut piles, mut charge) = mainmaker(s);
	let (mut menu, check, mut vali, mut compte) = menumaker(s);

	let mut pilesaddr = getaddrs();

	let mut res = Frame::new(0, 170 * 4 + 30, 89 * 6, 30, None);
	res.set_label_color(Color::White);

	let mut qli: Vec<String> = vec![];
	let mut rli: Vec<bool> = vec![];
	let mut mode = MODE::ET;
	let mut monnum: usize = 0;

	wind.make_resizable(false);
	wind.end();
	wind.show();
	wind.wait_for_expose();
	a.wait();
	app::sleep(0.02);
	//check.set_checked(true);
	//s.send(Message::Triche);
	//wind.flush();//a.redraw();
	s.send(Message::Change);
	//a.redraw();

	//fnerror("test");

	while a.wait() {
		use Message::*;
		if let Some(msg) = r.recv() {
			match msg {
				Sauvegarde => {}
				//Question(n) => toggli[n]^=true,
				Switch(n) => toggle(&mut frame[n]),
				Mode(m) => mode = m,
				Triche => {
					if check.value() {
						compte.activate();
					} else {
						compte.deactivate();
					}
				}
				Compte => {
					res.set_label(
						&(comm(true, &mut pilesaddr[monnum], &mut rli, &mut frame, &mut mode, &mut menu).to_string()
							+ " vont etre renversés"),
					);
				}
				Valider => {
					if check.value() {
						let _ = comm(false, &mut pilesaddr[monnum], &mut rli, &mut frame, &mut mode, &mut menu);
					}
					res.set_label(if valide(&mut qli, &mut rli, &mut menu, &mut mode) {
						"Vrai"
					} else {
						"Faux"
					});
				}
				Etale(num) => {
					monnum = num;
					etalepile(&mut frame, &mut pilesaddr[num], &mut b);
					hidevec(&mut piles);
					compte.show();
					vali.show();
					charge.hide();
					match loadquestions(&mut menu, s, &pilesaddr[num]) {
						Some((x, y)) => {
							qli = x;
							rli = y;
						}
						None => fnerror("json incorrect"),
					}
				}
				Change => {
					menu.remove(menu.find_index("Question"));
					activate(&mut frame);
					hide(&mut b);
					hide(&mut frame);
					compte.hide();
					vali.hide();
					charge.show();
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
