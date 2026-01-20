#[derive(Debug)]
enum Equip1a {
    Alaves,
    AtMadrid,
    AthBilbao,
    Barcelona,
    Betis,
    CeltaDeVigo,
    Elx,
    Espanyol,
    Getafe,
    Girona,
    Llevant,
    Mallorca,
    Oviedo,
    Osasuna,
    RayoVallecano,
    RealMadrid,
    RealSociedad,
    Sevilla,
    Valencia,
    Vilareal,
}

#[derive(Debug)]
struct Resultat {
    marcador: String,
    jornada: u32,
    equipcasa: Equip1a,
    equipfora: Equip1a,
}

fn main() {
    let my_box = Box::new(String::from("Quim"));
    let mut get_var = {
        let a = my_box;
        println!("{a}");
        a
    };

    t1(&mut get_var);
    println!("{get_var}");
}

fn t1(p1: &mut Box<String>) {
    println!("{p1}");
}
