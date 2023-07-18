
 /*
 Scrivi un programma che dato il prezzo netto di un prodotto (inserito in ingresso da tastiera) lo sconti
 del 35% se tale prezzo è maggiore di 100€ e poi aggiunga l'IVA del 20%
 stampando a video il risultato finale.
 */
 use csv::ReaderBuilder;
 use std::{fmt, fs::File};
 use std::borrow::Borrow;
 use std::io::stdin;
 use crate::Categoria::{Drink, Frutta, Verdura};

 struct Persona{
     nome : String,
     portafoglio : i32
 }


 #[derive(Debug)]
 enum Categoria {
    Verdura,
    Frutta,
     Drink
}
impl Categoria {


    fn from_string_to_enum(value : &str) -> Self {
        match value.trim() {
            "Verdura" => Verdura,
            "Frutta" => Frutta,
            "Drink" => Drink,
            _ => panic!("Errore durante la conversione sulla categoria {}", value)
        }
    }

    fn value(&self) -> f32 {
        match *self {
            Verdura => 14_f32,
            Frutta => 12_f32,
            _ => 30_f32,

        }
    }
}
 #[derive(Debug)]
 struct Prodotto{
    nome : String,
    prezzo : String,
    categoria : Categoria
}
 impl Prodotto{
     const CSV_PATH : &'static str = "./prodotti.csv";

     fn show(&self){
         println!("{:?}", *self);
     }


     fn from_csv() -> Vec<Prodotto> {
         let mut prodotti : Vec<Prodotto> = vec!();
         let file = File::open(Prodotto::CSV_PATH).unwrap();
         let mut csv_reader = ReaderBuilder::new()
             .has_headers(false) // Se il file ha una riga di intestazione
             .from_reader(file);

         // Itera sulle righe del file CSV
         for result in csv_reader.records() {
             // Estrai la riga dal risultato
             let record = result.unwrap();

             // Elabora la riga del CSV
             let nome_prodotto = &record[0];
             let prezzo_prodotto = &record[1];
             let categoria_prodotto = &record[2];
             let prodotto = Prodotto{
                 nome : String::from(nome_prodotto),
                 prezzo: String::from(prezzo_prodotto),
                 categoria: Categoria::from_string_to_enum(categoria_prodotto)
             };
             prodotti.push(prodotto);
         }
        prodotti
     }
 }

 impl fmt::Display for Prodotto {
     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
         write!(f, "{}", self)
     }
 }

 struct Compravendita{
     products: Vec<Prodotto>
 }

 impl Compravendita {



    fn new (products : Vec<Prodotto>) -> Compravendita{
        Self {
            products
        }
    }

     fn read_order() -> String {
         let mut input = String::from("");
         stdin()
             .read_line(&mut input)
             .expect("Errore nella lettura dell'input");

         String::from(input.trim())
     }

    fn run (&mut self){
        while self.products.len() > 1 {
            self.products
                .iter()
                .for_each(|pr| pr.show());
            println!("Inserisci il tuo ordine");
           let order = Self::read_order();

            match order { //PERCHé NON POSSO USARGLI LA BORROW NEL LAMBDA?
                value if self.products.iter().all(|prod| prod.nome != value) =>
                    panic!("Nessun match con quello che hai richiesto"),
                _=> {
                    let product_index : usize = self.products
                        .iter()
                        //.borrow()//PERCHè NO IL BORROW?
                        .position(|prod| prod.nome == order)
                        .unwrap();
                    self.products.remove(product_index);
                }
            }
        }
    }


 }


 fn main() {
     let prodotti = Prodotto::from_csv();

     let mut compravendita = Compravendita::new(prodotti);
     compravendita.run();
 }
