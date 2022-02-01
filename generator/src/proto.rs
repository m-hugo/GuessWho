use fltk::{
	app,
	button::Button,
	group::{Group, Pack, Tabs},
	input::Input,
	menu::{Choice, MenuButton},
	output::Output,
	prelude::{GroupExt, InputExt, WidgetBase, WidgetExt, WindowExt},
	window::Window,
};
use fltk::browser::HoldBrowser;
//use serde_json;
use serde_json::Value;
//use serde::{Deserialize, Serialize};
//use serde_json::Result;

fn main() {
	let app = app::App::default().with_scheme(app::Scheme::Gtk);
	//app::background(221, 221, 221);

	let mut wind = Window::default().with_size(500, 450).with_label("Tabs").center_screen();
	let mut pack = Pack::new(15, 45, 150, 450 - 45, "");
	pack.set_spacing(10);
	let inp = Input::default().with_size(0, 30).with_label("");
	let inp2 = Input::default().with_size(0, 30).with_label("");
	let inp3 = Input::default().with_size(0, 30).with_label("");
	let _inp4 = Input::default().with_size(0, 30).with_label("");
	let mut but1 = Button::default().with_size(0, 30).with_label("Valider");
	let list_browser = HoldBrowser::default();//.with_size(0, 30);
	let selected_name = list_browser.text(list_browser.value()).unwrap();
	pack.end();
	wind.make_resizable(false);
	wind.end();
	wind.show();
	but1.set_callback(move |_| {
		//print_an_address();
		let address: Value = vec![inp.value(), inp2.value(), inp3.value()].into();
		println!("{}", serde_json::to_string_pretty(&address).unwrap());
	});
	app.run().unwrap();
}
