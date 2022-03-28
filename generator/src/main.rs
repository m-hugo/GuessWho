use fltk::image::PngImage;
use fltk::menu::*;
use fltk::{app::*, browser::*, button::*, dialog, enums::*, group::*, input::*, prelude::*, window::*};
use once_cell::sync::Lazy;
use serde_json::Map;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::io::BufReader;
use std::sync::Mutex;
const WIDGET_WIDTH: i32 = 70;
const WIDGET_HEIGHT: i32 = 25;
const WIDGET_PADDING: i32 = 10;

#[derive(Clone, Copy)]
enum Message {
	Create,
	Update,
	Delete,
	Create2,
	Update2,
	Delete2,
	Select,
	Select2,
	Show,
	Show2,
	Export,
	Import,
}
fn tryfile(f: &str) -> Result<Value, Box<dyn std::error::Error>> {
	let file = fs::File::open(f)?; //txt
	let reader = BufReader::new(file);
	let v: Value = serde_json::from_reader(reader)?;
	Ok(v)
}
static CHCEMAP: Lazy<Mutex<HashMap<(usize, String), String>>> = Lazy::new(|| Mutex::new(HashMap::new()));

fn main() {
	let app = App::default().with_scheme(Scheme::Gtk);
	let mut wind = Window::default().with_label("Generateur");

	let (sender, receiver) = channel::<Message>();

	//let mut pack = Pack::new(15, 45, 150, 450 - 45, "");
	//pack.set_spacing(10);
	let tab = Tabs::new(10, 10, 800, 450 - 45, "");
	let grp1 = Group::new(10, 35, 800, 450 - 45, "Attributs\t\t");

	//filter_input.set_trigger(CallbackTrigger::Changed);

	let mut attr_input = Input::default()
		.with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
		.with_pos(50 + WIDGET_PADDING, 40 + WIDGET_PADDING)
		.with_label("Ajout:");

	let mut list_browser = HoldBrowser::default()
		.with_pos(WIDGET_PADDING, attr_input.y() + attr_input.height() + WIDGET_PADDING)
		.with_size(WIDGET_WIDTH * 3, WIDGET_HEIGHT * 10);
	list_browser.emit(sender, Message::Select);

	let mut create_button = Button::default()
		.with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
		.with_pos(WIDGET_PADDING, list_browser.y() + list_browser.height() + WIDGET_PADDING)
		.with_label("Create");
	create_button.emit(sender, Message::Create);

	let mut update_button = Button::default()
		.with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
		.right_of(&create_button, WIDGET_PADDING)
		.with_label("Update");
	update_button.emit(sender, Message::Update);
	update_button.deactivate();

	let mut delete_button = Button::default()
		.with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
		.right_of(&update_button, WIDGET_PADDING)
		.with_label("Delete");
	delete_button.emit(sender, Message::Delete);
	delete_button.deactivate();

	let mut list_browser2 = HoldBrowser::default()
		.with_pos(list_browser.x() + list_browser.width() + 70, list_browser.y())
		.with_size(list_browser.width(), list_browser.height());
	list_browser2.emit(sender, Message::Select2);

	let mut attr_input2 = Input::default()
		.with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
		.with_pos(50 + WIDGET_PADDING + list_browser2.x(), attr_input.y())
		.with_label("Ajout:");

	let mut create_button2 = Button::default()
		.with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
		.with_pos(list_browser2.x(), list_browser2.y() + list_browser2.height() + WIDGET_PADDING)
		.with_label("Create");
	create_button2.emit(sender, Message::Create2);
	create_button2.deactivate();

	let mut update_button2 = Button::default()
		.with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
		.right_of(&create_button2, WIDGET_PADDING)
		.with_label("Update");
	update_button2.emit(sender, Message::Update2);
	update_button2.deactivate();

	let mut delete_button2 = Button::default()
		.with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
		.right_of(&update_button2, WIDGET_PADDING)
		.with_label("Delete");
	delete_button2.emit(sender, Message::Delete2);
	delete_button2.deactivate();

	grp1.end();
	let mut grp2 = Scroll::new(
		10,
		35,
		delete_button2.x() + delete_button2.width() + WIDGET_PADDING,
		create_button.y() + create_button.height() + WIDGET_PADDING - 25,
		"Personnages\t\t",
	);
	grp2.set_type(ScrollType::Both);
	grp2.end();
	tab.end();
	//pack.end();
	let mut attrslis: HashMap<String, Vec<String>> = HashMap::new();
	attrslis.insert("".to_string(), vec![]);
	let mut model = vec![];
	/*let mut model = vec![
		"Nom".to_string(),
		"Chauve".to_string(),
		"Lunettes".to_string(),
		"Lunettes2".to_string(),
	];
	attrslis.insert("Nom".to_string(), vec![]);
	attrslis.insert("Chauve".to_string(), vec![]);
	attrslis.insert("Lunettes".to_string(), vec![]);
	attrslis.insert("Lunettes2".to_string(), vec![]);*/

	let mut attractu = "".to_string();
	sender.send(Message::Show);

	wind.set_size(
		delete_button2.x() + delete_button2.width() + WIDGET_PADDING + 10,
		create_button.y() + create_button.height() + WIDGET_PADDING + 10,
	);
	let mut export = Button::default()
		.with_pos(wind.width() - WIDGET_WIDTH - WIDGET_PADDING / 2, WIDGET_PADDING / 2)
		.with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
		.with_label("Export");
	export.emit(sender, Message::Export);
	let mut import = Button::default()
		.with_pos(export.x() - WIDGET_WIDTH - WIDGET_PADDING / 2, WIDGET_PADDING / 2)
		.with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
		.with_label("Import");
	import.emit(sender, Message::Import);

	wind.end();
	wind.show();
	while app.wait() {
		match receiver.recv() {
			Some(Message::Create) => {
				if attr_input.value().len() > 0 {
					model.push(attr_input.value());
					attrslis.insert(attr_input.value(), vec![]);
					attr_input.set_value("");
					sender.send(Message::Show);
				}
			}
			Some(Message::Update) => {
				if attr_input.value().len() > 0 {
					let selected_name = list_browser.text(list_browser.value()).unwrap();
					let index = model.iter().position(|s| s == &selected_name).unwrap();
					model[index] = attr_input.value();
					let oldcont = attrslis.remove(&selected_name).unwrap();
					attrslis.insert(attr_input.value(), oldcont);
					attractu = attr_input.value();
					list_browser.insert(list_browser.value(), &attr_input.value());
					list_browser.select(list_browser.value() - 1);
					list_browser.remove(list_browser.value() + 1);
					attr_input.set_value("");
					println!("{:?}", CHCEMAP.lock().unwrap());
					sender.send(Message::Show);
				}
			}
			Some(Message::Delete) => {
				let selected_name = list_browser.text(list_browser.value()).unwrap();
				let index = model.iter().position(|s| s == &selected_name).unwrap();
				model.remove(index);
				attrslis.remove(&selected_name);
				sender.send(Message::Show);
				sender.send(Message::Select);
			}
			Some(Message::Select) => {
				if list_browser.value() == 0 {
					attractu = "".to_string();
					update_button.deactivate();
					create_button2.deactivate();
					delete_button.deactivate();
					sender.send(Message::Select2);
				} else {
					attractu = list_browser.text(list_browser.value()).unwrap();
					update_button.activate();
					create_button2.activate();
					delete_button.activate();
					sender.send(Message::Show2);
				}
			}
			Some(Message::Show) => {
				list_browser.clear();
				for item in &model {
					list_browser.add(item);
				}
				sender.send(Message::Select);
			}
			Some(Message::Create2) => {
				if attr_input2.value().len() > 0 {
					attrslis.get_mut(&attractu).unwrap().push(attr_input2.value());
					attr_input2.set_value("");
					sender.send(Message::Show2);
				}
			}
			Some(Message::Update2) => {
				let selected_name = list_browser2.text(list_browser2.value()).unwrap();
				let index = attrslis[&attractu].iter().position(|s| s == &selected_name).unwrap();
				attrslis.get_mut(&attractu).unwrap()[index] = attr_input2.value();
				sender.send(Message::Show2);
			}
			Some(Message::Delete2) => {
				let selected_name = list_browser2.text(list_browser2.value()).unwrap();
				let index = attrslis[&attractu].iter().position(|s| s == &selected_name).unwrap();
				attrslis.get_mut(&attractu).unwrap().remove(index);
				sender.send(Message::Show2);
			}
			Some(Message::Export) => {
				println!("Code json");
				let mut map: serde_json::Map<String, Value> = Map::new();
				for n in 0..model.len() {
					map.insert(model[n].clone(), attrslis[&model[n]].clone().into());
				}
				let v: Value = map.into();
				let mut map2: serde_json::Map<String, Value> = Map::new();
				map2.insert("attrs".to_string(), v);

				let mut vecval: Vec<serde_json::Map<String, Value>> = vec![];
				if let Ok(c) = CHCEMAP.lock() {
					for n in 0..24 {
						let mut mapval: serde_json::Map<String, Value> = Map::new();
						if let Some(ff) = c.get(&(n, "image".to_string())) {
							mapval.insert("image".to_string(), ff.clone().into());
						} else {
							println!("erreur export image, {}", n);
						}
						for mv in &model {
							if let Some(ff) = c.get(&(n, mv.to_string())) {
								mapval.insert(mv.to_string(), ff.clone().into());
							} else {
								println!("erreur export {}, {} in {:?}", n, mv, c);
							}
						}
						vecval.push(mapval);
					}
				}
				map2.insert("liste".to_string(), vecval.into());
				let v2: Value = map2.into();
				let mut dlg = dialog::FileDialog::new(dialog::FileDialogType::BrowseSaveFile);
				dlg.set_option(dialog::FileDialogOptions::SaveAsConfirm);
				dlg.show();
				if !dlg.filename().to_string_lossy().to_string().is_empty() {
					fs::write(
						&dlg.filename().to_string_lossy().to_string(),
						serde_json::to_string_pretty(&v2).unwrap(),
					)
					.expect("Unable to write file");
				}
			}
			Some(Message::Import) => {
				let mut dlg = dialog::FileDialog::new(dialog::FileDialogType::BrowseFile);
				dlg.set_option(dialog::FileDialogOptions::NoOptions);
				dlg.set_filter("*.{json}");
				dlg.show();
				let filename = dlg.filename().to_string_lossy().to_string();
				if let Ok(v) = tryfile(&filename) {
					model.clear();
					attrslis.clear();
					for (x, y) in v["attrs"].as_object().unwrap() {
						let xs = x.as_str().to_string();
						model.push(xs.clone());
						attrslis.insert(
							xs,
							y.as_array()
								.unwrap()
								.iter()
								.map(|a| a.as_str().unwrap().to_string())
								.collect(),
						);
					}
					for n in 0..24 {
						for (x, y) in v["liste"][n].as_object().unwrap() {
							let xs = x.as_str();
							CHCEMAP
								.lock()
								.unwrap()
								.insert((n, xs.to_string()), y.as_str().unwrap().to_string());
						}
					}
					sender.send(Message::Show);
				}
			}
			Some(Message::Select2) => {
				if list_browser2.value() == 0 {
					update_button2.deactivate();
					delete_button2.deactivate();
				} else {
					update_button2.activate();
					delete_button2.activate();
				}
				grp2.clear();
				for u in 0..24 {
					let yeq = 10 + (1 + (model.len() / 2)) * (30 + (WIDGET_PADDING as usize)) * (u);
					//let jj = Box::leak(u.to_string().into_boxed_str());
					let mut fr = Button::new(10, (yeq).try_into().unwrap(), 50, 75, None);
					for n in 0..model.len() {
						let mut chce = Choice::default()
							.with_pos(
								110 + 250 * ((n % 2) as i32) + WIDGET_PADDING,
								(yeq + 30 * (n / 2)).try_into().unwrap(),
							)
							.with_size(150, WIDGET_HEIGHT)
							.with_label(&model[n]);
						for y in &attrslis[&model[n]] {
							chce.add(y, Shortcut::None, MenuFlag::Normal, move |c| {
								CHCEMAP.lock().unwrap().insert((u, c.label()), c.choice().unwrap());
							});
						}
						if let Some(val) = CHCEMAP.lock().unwrap().get(&(u, chce.label())) {
							for x in chce.clone() {
								if let Some(l) = x.label() {
									if &l == val {
										chce.set_item(&x);
									}
								}
							}
						}
						grp2.add(&chce);
					}

					if let Some(val) = CHCEMAP.lock().unwrap().get(&(u, "image".to_string())) {
						fr.set_image_scaled(Some(PngImage::load("../".to_string() + val).unwrap()));
						fr.redraw();
					} else {
						fr.set_image::<PngImage>(None);
					}

					fr.set_callback(move |b| {
						let mut dlg = dialog::FileDialog::new(dialog::FileDialogType::BrowseFile);
						dlg.set_option(dialog::FileDialogOptions::NoOptions);
						dlg.set_filter("*.{png}");
						dlg.show();
						let path = dlg.filename();
						let prefix = fs::canonicalize("../").unwrap();
						if let Ok(p) = path.strip_prefix(&prefix) {
							let filename = "./".to_string() + &p.to_string_lossy();
							println!("{}", filename);
							b.set_image_scaled(Some(PngImage::load("../".to_string() + &filename).unwrap()));
							CHCEMAP.lock().unwrap().insert((u, "image".to_string()), filename);
							b.redraw();
						} else {
							println!("{:?} pas dans {:?}", path, prefix);
						}
					});
					grp2.add(&fr);
				}
			}
			Some(Message::Show2) => {
				list_browser2.clear();
				for item in &attrslis[&attractu] {
					list_browser2.add(item);
				}
				sender.send(Message::Select2);
			}
			None => {}
		}
	}
}
