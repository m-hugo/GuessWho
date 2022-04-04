use serde_json::Value;
use std::fs;
use std::io::BufReader;

#[macro_use]
extern crate rocket;

use once_cell::sync::Lazy;
use std::sync::Mutex;

use std::sync::atomic::AtomicBool;
static LOCK: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));

#[get("/qr/lock")]
fn lock() {
	*TXT.lock().unwrap() = "".to_string();
	*LOCK.lock().unwrap() = true;
}

#[get("/qr/unlock")]
fn unlock() {
	*LOCK.lock().unwrap() = false;
}

static TXT: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new("".to_string()));

#[get("/qr/locked")]
fn locked() -> String {
	LOCK.lock().unwrap().to_string()
}

#[get("/qr/get")]
fn get() -> String {
	if *LOCK.lock().unwrap() {
		"Attente".to_string()
	} else {
		TXT.lock().unwrap().to_string()
	}
}

static MODE: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new("".to_string()));

#[get("/qr/mode/<mode>")]
fn mode(mode: &str) {
	*MODE.lock().unwrap() = mode.to_string();
}

#[get("/qr/add/Question/<attr>/<val>")]
fn add(attr: &str, val: &str) {
	let empt = TXT.lock().unwrap().is_empty();
	*TXT.lock().unwrap() += &if empt {
		if *MODE.lock().unwrap() == "XOR" {
			format!("Un seul parmi: {} = {}", attr, val)
		} else {
			format!("{} = {}", attr, val)
		}
	} else {
		if *MODE.lock().unwrap() == "XOR" {
			format!(", {} = {}", attr, val)
		} else {
			format!(" {} {} = {}", *MODE.lock().unwrap(), attr, val)
		}
	};
}

#[get("/qr/addres/<res>")]
fn addres(res: &str) {
	*TXT.lock().unwrap() += &format!(" est {}", res);
}

static VEC: Lazy<Mutex<Vec<bool>>> = Lazy::new(|| Mutex::new(vec![false; 24]));

#[get("/<perso>/couché/<booly>")]
fn couche(perso: usize, booly: bool) -> Option<()> {
	VEC.lock().unwrap().get_mut(perso).map(|p| *p = booly)
}

#[get("/<perso>/estcouché")]
fn estcouche(perso: usize) -> Option<String> {
	VEC.lock().unwrap().get_mut(perso).map(|p| p.to_string())
}

#[launch]
fn rocket() -> _ {
	rocket::build()
		.mount(
			"/j1/jsons",
			routes![couche, estcouche, lock, unlock, locked, get, add, mode, addres],
		)
		.mount(
			"/j2/jsons",
			routes![couche2, estcouche2, lock2, unlock2, locked2, get2, add2, mode2, addres2],
		)
}

static LOCK2: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));

#[get("/qr/lock")]
fn lock2() {
	*TXT2.lock().unwrap() = "".to_string();
	*LOCK2.lock().unwrap() = true;
}

#[get("/qr/unlock")]
fn unlock2() {
	*LOCK2.lock().unwrap() = false;
}

static TXT2: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new("".to_string()));

#[get("/qr/locked")]
fn locked2() -> String {
	LOCK2.lock().unwrap().to_string()
}

#[get("/qr/get")]
fn get2() -> String {
	if *LOCK2.lock().unwrap() {
		"Attente".to_string()
	} else {
		TXT2.lock().unwrap().to_string()
	}
}

static MODE2: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new("".to_string()));

#[get("/qr/mode/<mode>")]
fn mode2(mode: &str) {
	*MODE2.lock().unwrap() = mode.to_string();
}

#[get("/qr/add/Question/<attr>/<val>")]
fn add2(attr: &str, val: &str) {
	let empt = TXT2.lock().unwrap().is_empty();
	*TXT2.lock().unwrap() += &if empt {
		if *MODE2.lock().unwrap() == "XOR" {
			format!("Un seul parmi: {} = {}", attr, val)
		} else {
			format!("{} = {}", attr, val)
		}
	} else {
		if *MODE2.lock().unwrap() == "XOR" {
			format!(", {} = {}", attr, val)
		} else {
			format!(" {} {} = {}", *MODE2.lock().unwrap(), attr, val)
		}
	};
}

#[get("/qr/addres/<res>")]
fn addres2(res: &str) {
	*TXT2.lock().unwrap() += &format!(" est {}", res);
}

static VEC2: Lazy<Mutex<Vec<bool>>> = Lazy::new(|| Mutex::new(vec![false; 24]));

#[get("/<perso>/couché/<booly>")]
fn couche2(perso: usize, booly: bool) -> Option<()> {
	VEC2.lock().unwrap().get_mut(perso).map(|p| *p = booly)
}

#[get("/<perso>/estcouché")]
fn estcouche2(perso: usize) -> Option<String> {
	VEC2.lock().unwrap().get_mut(perso).map(|p| p.to_string())
}
