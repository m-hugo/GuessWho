use fltk::menu::*;
use fltk::frame::*;
use fltk::{app::*, browser::*, button::*, enums::*, group::*, input::*, prelude::*, window::*};
use std::collections::HashMap;

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
}

fn main() {
	let app = App::default().with_scheme(Scheme::Gtk);
	let mut wind = Window::default().with_label("Generateur");

	let (sender, receiver) = channel::<Message>();

	//let mut pack = Pack::new(15, 45, 150, 450 - 45, "");
	//pack.set_spacing(10);
	let tab = Tabs::new(10, 10, 800, 300 - 20, "");
	let grp1 = Group::new(10, 35, 500 - 20, 450 - 45, "Attributs\t\t");

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
	let mut grp2 = Scroll::new(10, 35,delete_button2.x() + delete_button2.width() + WIDGET_PADDING,
	create_button.y() + create_button.height() + WIDGET_PADDING, "Personnages\t\t");
	grp2.set_type(ScrollType::Both);
	grp2.end();
	tab.end();
	//pack.end();
	let mut attrslis: HashMap<String, Vec<String>> = HashMap::new();
	attrslis.insert("".to_string(), vec![]);
	let mut model = vec!["Nom".to_string(), "Chauve".to_string(), "Lunettes".to_string(), "Lunettes2".to_string()];
	attrslis.insert("Nom".to_string(), vec![]);
	attrslis.insert("Chauve".to_string(), vec![]);
	attrslis.insert("Lunettes".to_string(), vec![]);
	attrslis.insert("Lunettes2".to_string(), vec![]);

	let mut attractu = "".to_string();
	//println!("{}", model[attractu as usize]);
	sender.send(Message::Show);

	wind.set_size(
		delete_button2.x() + delete_button2.width() + WIDGET_PADDING+10,
		create_button.y() + create_button.height() + WIDGET_PADDING+10,
	);
	let export = Button::default()
		.with_pos(wind.width() - WIDGET_WIDTH - WIDGET_PADDING / 2, WIDGET_PADDING / 2)
		.with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
		.with_label("Export");
	let mut _import = Button::default()
		.with_pos(export.x() - WIDGET_WIDTH - WIDGET_PADDING / 2, WIDGET_PADDING / 2)
		.with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
		.with_label("Import");

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
					list_browser.insert(list_browser.value(), &attr_input.value());
					list_browser.select(list_browser.value() - 1);
					list_browser.remove(list_browser.value() + 1);
					attr_input.set_value("");
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
				} else {
					attractu = list_browser.text(list_browser.value()).unwrap();
					update_button.activate();
					create_button2.activate();
					delete_button.activate();
				}
				sender.send(Message::Show2);
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
				sender.send(Message::Select2);
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
				for u in 1..25 {
				let jj = Box::leak(u.to_string().into_boxed_str());
				let fr = Frame::new(10  + WIDGET_PADDING, 10+(40 + WIDGET_PADDING)  * (u as i32), 10, WIDGET_HEIGHT, Some(&*jj));
				for n in 0..model.len() {
					let mut chce = MenuButton::default()
						.with_pos(50 + 150 * (n as i32) + WIDGET_PADDING, 10+(40 + WIDGET_PADDING)  * (u as i32))
						.with_size(150, WIDGET_HEIGHT)
						.with_label(&model[n]);
					grp2.add(&chce);
					grp2.add(&fr);
					for y in &attrslis[&model[n]] {
						chce.add_choice(y);
					}
				}
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
