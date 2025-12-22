#[derive(Debug)]
enum WineRegions {
    Bordeaux,
    Burgundy,
    Champagne,
    Tuscany,
    Rioja,
    NapaValley,
    Annaba,
}

struct Wine {
    name: String,
    region: WineRegions, // wine regions used as a type
}

fn supported_regions(w: WineRegions) {
    match w {
        WineRegions::Rioja => println!("Rioja is supported!"),
        _ => println!("{:?} is not supported!", w),
    }
}

fn poplation_regions(w: WineRegions) {
    match w {
        WineRegions::Bordeaux => println!("Bordeaux has a population of 1.2 million."),
        WineRegions::Burgundy => println!("Burgundy has a population of 1.6 million."),
        WineRegions::Champagne => println!("Champagne has a population of 0.3 million."),
        WineRegions::Tuscany => println!("Tuscany has a population of 3.7 million."),
        WineRegions::Rioja => println!("Rioja has a population of 0.3 million."),
        WineRegions::NapaValley => println!("Napa Valley has a population of 0.14 million."),
        WineRegions::Annaba => println!("Annaba has a population of 257,000."),
        _ => println!("Region not found."),

    }
}

fn main() {
    let wine1 = Wine {
        name: String::from("Chateau Margaux"),
        region: WineRegions::Bordeaux,
    };

    let wine2 = Wine {
        name: String::from("Barolo"),
        region: WineRegions::Tuscany,
    };

    let wine3 = Wine {
        name: String::from("no alchol"),
        region: WineRegions::Annaba,
    };

    println!("Wine 1: {} from {:?}", wine1.name, wine1.region);
    println!("Wine 2: {} from {:?}", wine2.name, wine2.region);
    println!("Wine 3: {} from {:?}", wine3.name, wine3.region);

    supported_regions(wine1.region);
    supported_regions(WineRegions::Rioja);

    poplation_regions(wine2.region);
}
