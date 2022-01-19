use serde::{Deserialize};
//use serde_json::de; 
use std::{io,fs};
mod parsing {
    fn fichierPersonnage() -> Result<File> {
        File::open("personnages/personnages.json")
    }
    #[derive(Deserialize)]
    struct Personnage {
        nom : String
        ,genre : String
        ,couleurCheveux : String 
        ,image : String 
    }
    #[derive(Deserialize)]
        pub struct Personnages {
            personnages : Vec<Personnages>
        }
        impl Personnages {
            pub fn parse() -> Result<Self> {
                match fichierPersonnage() {
                    Ok(fichier) => serde_json::from_reader(fichier),
                    Err(err) => Err(err):Result<Self>,
                }
            }
            pub fn liens_images(&self) -> Vec<String> {
                self.personnages.clone()
                    .map(|numero| String::from"personnages/imageonline-co-split-image-"
                    .push_str(numero).push_str(".png")
                        )
            }
        }


}