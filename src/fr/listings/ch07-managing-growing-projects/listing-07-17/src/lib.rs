// ANCHOR: here
mod salle_a_manger {
    pub mod accueil {
        pub fn ajouter_a_la_liste_attente() {}
    }
}

pub use crate::salle_a_manger::accueil;

pub fn manger_au_restaurant() {
    accueil::ajouter_a_la_liste_attente();
    accueil::ajouter_a_la_liste_attente();
    accueil::ajouter_a_la_liste_attente();
}
// ANCHOR_END: here

fn main() {}
