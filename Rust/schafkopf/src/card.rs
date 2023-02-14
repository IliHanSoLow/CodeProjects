#[derive(Clone, Copy)]
pub struct Card {
    farbe: Farbe,
    typ: Typ,
    trumpf: bool,
}

impl Card {
    pub fn new(farbe: Farbe, typ: Typ) -> Card {
        Card {
            farbe,
            typ,
            trumpf: false,
        }
    }
    fn get_farbe(&self) -> &Farbe {
        &self.farbe
    }
    fn get_typ(&self) -> &Typ {
        &self.typ
    }
    pub fn print(&self) {
        let mut sfarbe: (String, String) = (String::from(""), String::from(""));
        match &self.get_farbe() {
            Farbe::Eichel => sfarbe.0 = String::from("Eichel"),
            Farbe::Gras => sfarbe.0 = String::from("Gras"),
            Farbe::Herz => sfarbe.0 = String::from("Herz"),
            Farbe::Schelle => sfarbe.0 = String::from("Schelle"),
        }
        match &self.get_typ() {
            Typ::Ass => sfarbe.1 = String::from("Ass"),
            Typ::Zehn => sfarbe.1 = String::from("Zehn"),
            Typ::König => sfarbe.1 = String::from("König"),
            Typ::Ober => sfarbe.1 = String::from("Ober"),
            Typ::Unter => sfarbe.1 = String::from("Unter"),
            Typ::Neun => sfarbe.1 = String::from("Neun"),
            Typ::Acht => sfarbe.1 = String::from("Acht"),
            Typ::Sieben => sfarbe.1 = String::from("Sieben"),
        }
        println!("{} {}; ", sfarbe.0, sfarbe.1)
    }
    pub fn is_trumpf(&mut self) {
        match &self.farbe {
            Farbe::Herz => self.trumpf = true,
            _ => self.trumpf = false,
        }
        match &self.typ {
            Typ::Unter | Typ::Ober => self.trumpf = true,
            _ => self.trumpf = false,
        }
    }
}

#[derive(Copy, Clone)]
pub enum Farbe {
    Eichel,
    Gras,
    Herz,
    Schelle,
}
#[derive(Copy, Clone)]
pub enum Typ {
    Ass,
    Zehn,
    König,
    Ober,
    Unter,
    Neun,
    Acht,
    Sieben,
}
