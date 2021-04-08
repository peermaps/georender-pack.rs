pub fn get_priorities<'a>() -> Vec<(&'a str, u64)> {
    return vec![
        ("aerialway.cable_car", 97),
        ("aerialway.chair_lift", 96),
        ("aeroway.aerodrome", 98),
        ("amenity.*", 60),
        ("amenity.university", 99),
        ("boundary.*", 5),
        ("building.*", 20),
        ("craft.*", 8),
        ("landuse.*", 10),
        ("man_made.lighthouse", 85),
        ("military.*", 70),
        ("place.*", 9),
        ("power.*", 100),
        ("public_transport.*", 51),
        ("railway.*", 95),
        ("route.*", 7),
        ("sport.*", 1),
    ];
}
