use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum TrackName {
    Barcelona,
    BrandsHatch,
    Cota,
    Donington,
    Hungaroring,
    Imola,
    Indianapolis,
    Kyalami,
    LagunaSeca,
    Misano,
    Monza,
    MountPanorama,
    Nurburgring,
    Nurburgring24h,
    OultonPark,
    PaulRicard,
    RedBullRing,
    Silverstone,
    Snetterton,
    Spa,
    Suzuka,
    Valencia,
    WatkinsGlen,
    Zandvoort,
    Zolder,
}

impl fmt::Display for TrackName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            TrackName::Barcelona => "Barcelona",
            TrackName::BrandsHatch => "Brands Hatch",
            TrackName::Cota => "Circuit of the Americas",
            TrackName::Donington => "Donington",
            TrackName::Hungaroring => "Hungaroring",
            TrackName::Imola => "Imola",
            TrackName::Indianapolis => "Indianapolis Motor Speedway",
            TrackName::Kyalami => "Kyalami",
            TrackName::LagunaSeca => "Laguna Seca",
            TrackName::Misano => "Misano",
            TrackName::Monza => "Monza",
            TrackName::MountPanorama => "Mount Panorama",
            TrackName::Nurburgring => "Nürburgring GP",
            TrackName::Nurburgring24h => "Nürburgring GP 24 hours",
            TrackName::OultonPark => "Oulton Park",
            TrackName::PaulRicard => "Paul Ricard",
            TrackName::RedBullRing => "RedBull Ring",
            TrackName::Silverstone => "Silverstone",
            TrackName::Snetterton => "Snetterton",
            TrackName::Spa => "Spa",
            TrackName::Suzuka => "Suzuka",
            TrackName::Valencia => "Valencia",
            TrackName::WatkinsGlen => "Watkins Glen",
            TrackName::Zandvoort => "Zandvoort",
            TrackName::Zolder => "Zolder",
        };
        write!(f, "{}", label)
    }
}

impl FromStr for TrackName {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "barcelona" => Ok(TrackName::Barcelona),
            "brands_hatch" => Ok(TrackName::BrandsHatch),
            "cota" => Ok(TrackName::Cota),
            "donington" => Ok(TrackName::Donington),
            "hungaroring" => Ok(TrackName::Hungaroring),
            "imola" => Ok(TrackName::Imola),
            "indianapolis" => Ok(TrackName::Indianapolis),
            "kyalami" => Ok(TrackName::Kyalami),
            "laguna_seca" => Ok(TrackName::LagunaSeca),
            "misano" => Ok(TrackName::Misano),
            "monza" => Ok(TrackName::Monza),
            "mount_panorama" => Ok(TrackName::MountPanorama),
            "nurburgring" => Ok(TrackName::Nurburgring),
            "nurburgring_24h" => Ok(TrackName::Nurburgring24h),
            "oulton_park" => Ok(TrackName::OultonPark),
            "paul_ricard" => Ok(TrackName::PaulRicard),
            "red_bull_ring" => Ok(TrackName::RedBullRing),
            "silverstone" => Ok(TrackName::Silverstone),
            "snetterton" => Ok(TrackName::Snetterton),
            "spa" => Ok(TrackName::Spa),
            "suzuka" => Ok(TrackName::Suzuka),
            "valencia" => Ok(TrackName::Valencia),
            "watkins_glen" => Ok(TrackName::WatkinsGlen),
            "zandvoort" => Ok(TrackName::Zandvoort),
            "zolder" => Ok(TrackName::Zolder),
            _ => Err(()),
        }
    }
}
