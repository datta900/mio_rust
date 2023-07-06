
 /*
 Scrivi un programma che dato il prezzo netto di un prodotto (inserito in ingresso da tastiera) lo sconti
 del 35% se tale prezzo è maggiore di 100€ e poi aggiunga l'IVA del 20%
 stampando a video il risultato finale.
 */
use std::io;
 use std::fmt;

enum Categoria {
    Verdura (f32)
}
impl Categoria {
    fn new_verdura() -> Self {
        Self::Verdura(4.0) // Valore predefinito per la variante Verdura
    }
    fn discount(&self) -> f32{
        match self {
            Self::Verdura(value) => *value
        }
    }
}
struct Prodotto{
    nome : String,
    prezzo : i32,
    categoria : Categoria
}

 impl fmt::Display for Categoria {
     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
         match self {
             Self::Verdura(value) => write!(f, "Categoria: Verdura, discount: {}", value),
         }
     }
 }

fn main() {
    let ex = Categoria::Verdura;
    let zucchine = Prodotto {
        nome: "".to_string(),
        prezzo: 0,
        categoria: Categoria::new_verdura()
    };

    println!("Zucchine : {}", zucchine.categoria.discount());

    println!("Inserisci il prezzo del prodotto");
    let mut input_str= String::new();
    io::stdin().read_line(&mut input_str)
        .expect("Errore durante l'inserimento del prezzo del prodotto");
    let mut prezzo_prodotto : f32 = input_str.trim().parse::<i32>().unwrap() as f32;

    match prezzo_prodotto {
         n if n > 100_f32 => {
            prezzo_prodotto = n - (n * 0.35)
        },
        _ => {}
    }
    let prezzo_finale = prezzo_prodotto + (prezzo_prodotto * 0.20);
    println!("{prezzo_finale}");

}
