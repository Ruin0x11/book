struct Utilisateur {
    pseudo: String,
    email: String,
    nombre_de_connexions: u64,
    actif: bool,
}

fn main() {
    // ANCHOR: here
    let mut utilisateur1 = Utilisateur {
        email: String::from("quelquun@example.com"),
        pseudo: String::from("pseudoquelconque123"),
        actif: true,
        nombre_de_connexions: 1,
    };
    
    utilisateur1.email = String::from("unautremail@example.com");
    // ANCHOR_END: here
}
