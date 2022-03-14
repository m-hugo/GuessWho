use serde_json::Value;
use std::fs;
use std::io::BufReader;
#[macro_use] extern crate rocket;

fn tryfile(f: &str) -> Result<Value, Box<dyn std::error::Error>> {
	let file = fs::File::open(f)?; //txt
	let reader = BufReader::new(file);
	let v: Value = serde_json::from_reader(reader)?;
	Ok(v)
}
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}
//   http://127.0.0.1:8000/wave/Rocketeer/100
#[get("/<name>/<perso>")]
fn jsons(name: &str, perso: &str) -> String  {
    //format!("👋 bonjour, {} tu as {} ans!", name, age)
	let mut trouve = "pas trouve";
	let v = tryfile(&name).unwrap();
				for n in 0..24{
				for (x, y) in v["liste"][n].as_object().unwrap() {
					if x == "nom" { if y == perso {trouve="trouve";}}
				}
				}
	trouve.to_string()
}
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index]).mount("/jsons", routes![jsons])
}