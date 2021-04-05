pub fn get_priorities<'a>() -> Vec<(&'a str, u64)> {
    return vec![
        ("aerialway.cable_car", 100),
        ("aerialway.chair_lift", 100),
        ("aeroway.aerodrome", 100),
        ("amenity.*", 60),
        ("amenity.university", 100),
        ("boundary.*", 5),
        ("building.*", 20),
        ("building.yes", 20),
        ("craft.*", 10),
        ("landuse.*", 10),
        ("man_made.lighthouse", 70),
        ("military.*", 70),
        ("place.*", 10),
        ("power.*", 100),
        ("public_transport.*", 50),
        ("railway.*", 100),
        ("route.*", 10),
        ("sport.*", 1),
    ];
}
