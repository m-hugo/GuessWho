use serde_json::{Deserialize};
//use serde_json::de; 
use std::fs::File;
use std::error::Error;
use std::io::BufReader;
mod parsing {
    #[derive(Clone,Deserialize)]
    struct Personnage {
        nom : String
        ,genre : String
        ,couleurCheveux : String 
        ,numero: String 
    }
    #[derive(Clone,Deserialize)]
        pub struct Personnages {
            personnages : Vec<Personnages>
        }
        impl Iterator for Personnages {
            type Item = Personnage; 
            fn next(&mut self)->Option<Self::Item>{
                self.personnages.next()

            }
        }
    
        impl Personnages {
            pub fn parse() -> Result<Self,Box<Error>> {
                let file = File::open("personnages/personnages.json")?;
                let reader = BufReader::new(file);
                // Read the JSON contents of the file as an instance of `Personnages`.
                let persos = serde_json::from_reader(reader)?;
                // Return the `Personnages`.
                Ok(persos)
            }
            pub fn liens_images(&self) -> Vec<String> {
                self.personnages
                    .map(|Personnage{numero,..}|numero.clone().insert_str(0,"personnages/imageonline-co-split-image-")
                    .push_str(numero).push_str(".png")
                        )
            }
        }


}