//#![allow(unused_imports)]
//cargo clippy -- -W clippy::pedantic -W clippy::nursery -A clippy::many-single-char-names -A clippy::similar-names -A clippy::unreadable-literal -A clippy::enum-glob-use -W absolute-paths-not-starting-with-crate -W deprecated-in-future -W elided-lifetimes-in-paths -W explicit-outlives-requirements -W keyword-idents -W macro-use-extern-crate -W meta-variable-misuse -W missing-abi -W missing-copy-implementations  -W non-ascii-idents -W noop-method-call -W pointer-structural-match -W rust-2021-incompatible-closure-captures -W rust-2021-incompatible-or-patterns -W rust-2021-prefixes-incompatible-syntax -W rust-2021-prelude-collisions -W single-use-lifetimes -W trivial-casts -W trivial-numeric-casts -W unreachable-pub -W unsafe-op-in-unsafe-fn -W unused-crate-dependencies -W unused-extern-crates -W unused-import-braces -W unused-lifetimes -W unused-qualifications -W variant-size-differences
//#![allow(deprecated)]
//mod parsing;
use fltk::app::Sender;
use fltk::dialog;
use fltk::group::{Scroll, ScrollType};

use fltk::{
	app,
	button::*,
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

use reqwest::blocking::Client;

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
	ChargeSauv,
	ChargeFile,
	Etale(usize),
	Mode(MODE),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE {
	ET,
	OU,
	XOR,
}

use once_cell::sync::Lazy;
use std::sync::atomic::AtomicBool;
use std::sync::Mutex;

static multiplayer: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));
static servme: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new("".to_string()));
static servadv: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new("".to_string()));

fn getaddrs() -> Vec<Value> {
	let mut addrs: Vec<Value> = vec![];
	for file in fs::read_dir("./").unwrap() {
		let u = file.as_ref().unwrap().file_name();
		if u.to_str().unwrap().contains('.') && u.to_str().unwrap().split('.').nth(1).unwrap() == "json" {
			if let Ok(v) = tryfile(&u.into_string().unwrap()) {
				if v["liste"][0]["image"].as_str().is_some() {
					addrs.push(v);
				}
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

fn etalepile(frame: &mut Vec<Frame>, val: &Value) {
	for n in 0..24 {
		frame[n].show();
		if let Some(vv) = val["liste"][n]["image"].as_str() {
			frame[n].set_image_scaled(Some(PngImage::load(vv.to_string()).unwrap())); //deja testé dans showdecks()
			let mut im = PngImage::load(vv.to_string()).unwrap(); //idem
			im.inactive();
			frame[n].set_deimage_scaled(Some(im));
			frame[n].redraw();
		}
	}
}

use reqwest::blocking::get;

fn toggle(frame: &mut Vec<Frame>, n: usize) {
	if frame[n].active() {
		frame[n].deactivate();
		if *multiplayer.lock().unwrap() {
			get(format!("{}/jsons/{}/couché/true", *servme.lock().unwrap(), n)).unwrap();
		}
	} else {
		frame[n].activate();
		if *multiplayer.lock().unwrap() {
			get(format!("{}/jsons/{}/couché/false", *servme.lock().unwrap(), n)).unwrap();
		}
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
	if addr.len() <= 10 {
		for k in 0..addr.len() {
			for n in 0..24 {
				if let Some(vv) = addr[k]["liste"][n]["image"].as_str() {
					piles[k][n].set_image_scaled(Some(PngImage::load(vv.to_string())?));
					piles[k][n].show();
					piles[k][n].redraw();
				} else {
					piles[k][0].set_label("Error");
					println!("image error");
				}
			}
		}
	} else {
		fnerror("erreur: il y a plus de 10 planches dans ./");
	}
	Ok(())
}

fn loadquestions(menu: &mut menu::MenuButton, s: Sender<Message>, v: &Value) -> Option<(Vec<String>, Vec<bool>)> {
	//let linoms = v["attrs"]["nom"].as_array()?;
	//let nom = linoms[rand::thread_rng().gen_range(0..linoms.len())];
	//println!("{}", nom);
	//Nom d'un personnage choisi au hasard

	let linoms = v["attrs"]["Nom"].as_array()?;
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
		Message::Mode(MODE::OU),
	);
	menu.add_emit(
		"Question/Mode \"un seul\"",
		Shortcut::None,
		menu::MenuFlag::Radio,
		s,
		Message::Mode(MODE::XOR),
	);

	Some((qli, rli))
}

use fltk::input::Input;
fn mainmaker(s: Sender<Message>) -> (Scroll, Vec<Vec<Button>>, RadioButton, RadioButton, Input) {
	let mut scrollgrp = Scroll::new(0, 31, 89 * 6, 170 * 4 + 30, "Choisir Planche");
	scrollgrp.set_type(ScrollType::Vertical);

	let mut piles: Vec<Vec<Button>> = vec![];
	for k in 0..10 {
		let mut pile: Vec<Button> = vec![];
		for i in 0..24 {
			let mut tmppile = Button::new(i + 90 / (24 - i), 170 * k + 30, 89, 146, None);
			tmppile.emit(s, Message::Etale(k.try_into().unwrap())); //trop petit pour overflow
			tmppile.hide();
			pile.push(tmppile);
		}
		piles.push(pile);
	}

	let mut multi = CheckButton::new(300, 200, 100, 20, Some("multijoueur"));
	multi.set_callback(|b| *multiplayer.lock().unwrap() = b.is_checked());
	multi.set_label_color(Color::from_hex(0x42A5F5));

	let mut j1 = RadioButton::new(410, 200, 30, 20, Some("j1"));
	j1.set_label_color(Color::from_hex(0x42A5F5));

	let mut j2 = RadioButton::new(410, 230, 30, 20, Some("j2"));
	j2.set_label_color(Color::from_hex(0x42A5F5));

	let mut url = Input::default().with_size(100, 20).with_pos(300, 230).with_label("Url");
	url.set_label_color(Color::from_hex(0x42A5F5));

	let mut fr = Frame::new(300, 300, 100, 10, Some("← Choisissez un tas de cartes"));
	let mut fr2 = Frame::new(300, 350, 100, 10, Some("ou chargez un fichier ↓"));
	fr.set_label_color(Color::from_hex(0x42A5F5));
	fr2.set_label_color(Color::from_hex(0x42A5F5));

	let mut chargesauv = Button::new(280, 500, 150, 40, "Charge Sauvegarde");
	chargesauv.set_color(Color::from_hex(0x42A5F5));
	chargesauv.emit(s, Message::ChargeSauv);

	let mut chargefile = Button::new(280, 600, 150, 40, "Charge Planche");
	chargefile.set_color(Color::from_hex(0x42A5F5));
	chargefile.emit(s, Message::ChargeFile);

	scrollgrp.end();

	(scrollgrp, piles, j1, j2, url)
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
	monaddr: &Value,
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
				MODE::OU => (nbsame > 0 && nbvrai > 0) || (nbvrai == 0 && nbsame == 0), //ou incl
				MODE::XOR => (nbsame == 1 && nbvrai == 1) || (nbvrai == 0 && nbsame == 0), //(nbsame == 1 && tousvrai) || (!tousvrai), //un seul
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
fn valide(qli: &mut Vec<String>, rli: &mut Vec<bool>, menu: &mut menu::MenuButton, mode: MODE, res: &mut Frame) {
	res.set_label("");
	if *multiplayer.lock().unwrap() {
		get(servme.lock().unwrap().to_string() + "/jsons/qr/lock").unwrap();
		get(servme.lock().unwrap().to_string() + &format!("/jsons/qr/mode/{:?}", mode)).unwrap();
	}
	let mut nb = 0_u64;
	let mut tous = true;
	for n in 0..qli.len() {
		let cho = menu.find_item(&qli[n]).unwrap();
		if cho.is_radio() && cho.value() {
			//println!("{} {}", qli[n], if rli[n] { "Vrai" } else { "Faux" });
			if *multiplayer.lock().unwrap() {
				get(servme.lock().unwrap().to_string() + &format!("/jsons/qr/add/{}", qli[n])).unwrap();
			}

			res.set_label(&format!(
				"{}{}",
				res.label(),
				if res.label().is_empty() {
					if mode == MODE::XOR {
						format!("Un seul parmi: {}", qli[n])
					} else {
						format!("{}", qli[n])
					}
				} else {
					if mode == MODE::XOR {
						format!(", {}", qli[n])
					} else {
						format!(" {:?} {}", mode, qli[n])
					}
				}
			));

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
	let mo = if match mode {
		MODE::ET => tous,     //et
		MODE::OU => nb > 0,   //ou incl
		MODE::XOR => nb == 1, //un seul
	} {
		"Vrai"
	} else {
		"Faux"
	};
	res.set_label(&format!("{} est {}", res.label(), mo));

	if *multiplayer.lock().unwrap() {
		get(servme.lock().unwrap().to_string() + &format!("/jsons/qr/addres/{}", mo)).unwrap();
		get(servme.lock().unwrap().to_string() + "/jsons/qr/unlock").unwrap();
	}
}
fn rr(path: &str) -> Result<String, Box<dyn std::error::Error>> {
	let url = servadv.lock()?.to_string() + path;
	Ok(get(url)?.text()?)
}
fn windadv(mapile: &Value) {
	let mut windpop = OverlayWindow::default()
		.with_size(89 * 6, 140 * 4 + 30)
		.center_screen()
		.with_label("Planche Adversaire");

	let mut frame: Vec<Frame> = vec![];
	for j in 0..4 {
		for i in 0..6 {
			frame.push(Frame::new(90 * i, 140 * j, 89, 146, None));
		}
	}

	let mut res = Frame::new(0, 140 * 4, 89 * 6, 30, None);
	res.set_label_color(Color::White);

	etalepile(&mut frame, mapile);

	windpop.make_resizable(false);
	windpop.end();
	windpop.show();

	//res.set_label("A");
	use std::{thread, time::Duration};
	thread::spawn(move || loop {
		let one = Duration::from_secs(1);
		thread::sleep(one);
		for n in 0..frame.len() {
			if let Ok(boo) = rr(&format!("/jsons/{}/estcouché", n)) {
				if boo.parse().unwrap() {
					frame[n].deactivate();
				} else {
					frame[n].activate();
				}
			} else {
				println!("Erreur connection internet");
			}
		}
		if let Ok(bobo) = rr("/jsons/qr/locked") {
			if bobo.parse().unwrap() {
				println!("Transfert en cours...");
			} else {
				if let Ok(resp) = rr("/jsons/qr/get") {
					res.set_label(&resp);
				} else {
					println!("Erreur connection internet");
				}
			}
		} else {
			println!("Erreur connection internet");
		}
	});
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

	let (mut scrollgrp, mut piles, j1, j2, url) = mainmaker(s);
	let (mut menu, check, mut vali, mut compte) = menumaker(s);

	let mut frame: Vec<Frame> = vec![];
	let mut b: Vec<Button> = vec![];
	for j in 0..4 {
		for i in 0..6 {
			frame.push(Frame::new(90 * i, 170 * j + 30, 89, 146, None));
			let mut tmpb = Button::new(90 * i + 2, 170 * j + 148 + 30, 89, 20, "Renverse");
			tmpb.set_color(Color::from_hex(0x42A5F5)); //0x42A5F5
			tmpb.emit(s, Message::Switch(b.len()));
			tmpb.hide();
			b.push(tmpb);
		}
	}

	let pilesaddr = getaddrs();

	let mut res = Frame::new(0, 170 * 4 + 30, 89 * 6, 30, None);
	res.set_label_color(Color::White);

	let mut qli: Vec<String> = vec![];
	let mut rli: Vec<bool> = vec![];
	let mut mode = MODE::ET;
	let mut mapile = &pilesaddr[0];
	let maybesauv = tryfile("./sauvegarde.json");
	let mut ooo;

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
				ChargeSauv => {
					if let Ok(ref v) = maybesauv {
						let arr = v["frames"].as_array().unwrap();
						for n in 0..frame.len() {
							if !arr[n].as_bool().unwrap() {
								frame[n].deactivate();
							}
						}
						mapile = &v["planche"];
						s.send(Etale(99));
					} else {
						fnerror("./sauvegarde.json n'existe pas");
					}
				}
				ChargeFile => {
					let mut dlg = dialog::FileDialog::new(dialog::FileDialogType::BrowseFile);
					dlg.set_option(dialog::FileDialogOptions::NoOptions);
					dlg.set_filter("*.{json}");
					dlg.show();
					let filename = dlg.filename().to_string_lossy().to_string();
					ooo = tryfile(&filename).unwrap();
					mapile = &ooo;
					s.send(Etale(99));
				}
				Sauvegarde => {
					let mut vecrenverse: Vec<bool> = vec![];
					for f in &frame {
						vecrenverse.push(f.active());
					}
					let frames: Value = vecrenverse.into();
					let mut m = serde_json::Map::new();
					m.insert("frames".to_string(), frames);
					m.insert("planche".to_string(), mapile.clone());
					fs::write("./sauvegarde.json", serde_json::to_string_pretty(&m).unwrap()).expect("Unable to write file");
					break;
				}
				//Question(n) => toggli[n]^=true,
				Switch(n) => toggle(&mut frame, n),
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
						&(comm(true, mapile, &mut rli, &mut frame, &mut mode, &mut menu).to_string() + " vont etre renversés"),
					);
				}
				Valider => {
					if check.value() {
						let _ = comm(false, mapile, &mut rli, &mut frame, &mut mode, &mut menu);
					}
					valide(&mut qli, &mut rli, &mut menu, mode, &mut res);
				}
				Etale(num) => {
					if num != 99 {
						mapile = &pilesaddr[num];
					}

					if *multiplayer.lock().unwrap() {
						*servme.lock().unwrap() = url.value();
						*servadv.lock().unwrap() = url.value();

						if j1.is_toggled() {
							*servme.lock().unwrap() += "/j1";
							*servadv.lock().unwrap() += "/j2";
						} else if j2.is_toggled() {
							*servme.lock().unwrap() += "/j2";
							*servadv.lock().unwrap() += "/j1";
						} else {
							fnerror("Choisissez j1 ou j2");
							continue;
						}
						if get(servme.lock().unwrap().to_string()).is_err() {
							fnerror("Url invalide");
						} else {
							windadv(mapile);

							etalepile(&mut frame, mapile);
							for n in 0..24 {
								b[n].show();
							}
							hidevec(&mut piles);
							compte.show();
							vali.show();
							scrollgrp.hide();
							match loadquestions(&mut menu, s, mapile) {
								Some((x, y)) => {
									qli = x;
									rli = y;
								}
								None => fnerror("json incorrect"),
							}
						}
					} else {
						etalepile(&mut frame, mapile);
						for n in 0..24 {
							b[n].show();
						}
						hidevec(&mut piles);
						compte.show();
						vali.show();
						scrollgrp.hide();
						match loadquestions(&mut menu, s, mapile) {
							Some((x, y)) => {
								qli = x;
								rli = y;
							}
							None => fnerror("json incorrect"),
						}
					}
				}
				Change => {
					menu.remove(menu.find_index("Question"));
					activate(&mut frame);
					hide(&mut b);
					hide(&mut frame);
					compte.hide();
					vali.hide();
					scrollgrp.show();
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
