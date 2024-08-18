use std::fmt::Display;
use std::str::FromStr;

#[derive(sqlx::Type, Debug, Clone)]
#[sqlx(type_name = "category", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CarCategory {
    CUP,
    ST,
    CHL,
    TCX,
    GT3,
    GT4,
    GT2,
}

impl Display for CarCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            CarCategory::CUP => "CUP".to_string(),
            CarCategory::ST => "ST".to_string(),
            CarCategory::CHL => "CHL".to_string(),
            CarCategory::TCX => "TCX".to_string(),
            CarCategory::GT3 => "GT3".to_string(),
            CarCategory::GT4 => "GT4".to_string(),
            CarCategory::GT2 => "GT2".to_string(),
        };
        write!(f, "{}", str)
    }
}

impl FromStr for CarCategory {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "CUP" => Ok(CarCategory::CUP),
            "ST" => Ok(CarCategory::ST),
            "CHL" => Ok(CarCategory::CHL),
            "TCX" => Ok(CarCategory::TCX),
            "GT3" => Ok(CarCategory::GT3),
            "GT4" => Ok(CarCategory::GT4),
            "GT2" => Ok(CarCategory::GT2),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub enum CarName {
    Porsche991iiGt3Cup,
    Porsche992Gt3Cup,
    LamborghiniHuracanSt,
    LamborghiniHuracanStEvo2,
    Ferrari488ChallengeEvo,
    BmwM2CsRacing,
    AmrV12VantageGt3,
    AmrV8VantageGt3,
    AudiR8Lms,
    AudiR8LmsEvo,
    AudiR8LmsEvoIi,
    BentleyContinentalGt32016,
    BentleyContinentalGt32018,
    BmwM4Gt3,
    BmwM6Gt3,
    JaguarG3,
    Ferrari296Gt3,
    Ferrari488Gt3,
    Ferrari488Gt3Evo,
    FordMustangGt3,
    HondaNsxGt3,
    HondaNsxGt3Evo,
    LamborghiniHuracanGt3,
    LamborghiniHuracanGt3Evo,
    LamborghiniHuracanGt3Evo2,
    LexusRcFGt3,
    Mclaren650sGt3,
    Mclaren720sGt3,
    Mclaren720sGt3Evo,
    MercedesAmgGt3,
    MercedesAmgGt3Evo,
    NissanGtRGt32017,
    NissanGtRGt32018,
    Porsche991Gt3R,
    Porsche991iiGt3R,
    Porsche992Gt3R,
    LamborghiniGallardoRex,
    AlpineA110Gt4,
    AmrV8VantageGt4,
    AudiR8Gt4,
    BmwM4Gt4,
    ChevroletCamaroGt4r,
    GinettaG55Gt4,
    KtmXbowGt4,
    MaseratiMcGt4,
    Mclaren570sGt4,
    MercedesAmgGt4,
    Porsche718CaymanGt4Mr,
    AudiR8LmsGt2,
    KtmXbowGt2,
    MaseratiMc20Gt2,
    MercedesAmgGt2,
    Porsche935,
    Porsche991Gt2RsMr,
}

impl CarName {
    fn get_car_category(&self) -> CarCategory {
        match self {
            CarName::Porsche991iiGt3Cup | CarName::Porsche992Gt3Cup => CarCategory::CUP,
            CarName::LamborghiniHuracanSt | CarName::LamborghiniHuracanStEvo2 => CarCategory::ST,
            CarName::Ferrari488ChallengeEvo => CarCategory::CHL,
            CarName::BmwM2CsRacing => CarCategory::TCX,
            CarName::AmrV12VantageGt3
            | CarName::AmrV8VantageGt3
            | CarName::AudiR8Lms
            | CarName::AudiR8LmsEvo
            | CarName::AudiR8LmsEvoIi
            | CarName::BentleyContinentalGt32016
            | CarName::BentleyContinentalGt32018
            | CarName::BmwM4Gt3
            | CarName::BmwM6Gt3
            | CarName::JaguarG3
            | CarName::Ferrari296Gt3
            | CarName::Ferrari488Gt3
            | CarName::Ferrari488Gt3Evo
            | CarName::FordMustangGt3
            | CarName::HondaNsxGt3
            | CarName::HondaNsxGt3Evo
            | CarName::LamborghiniHuracanGt3
            | CarName::LamborghiniHuracanGt3Evo
            | CarName::LamborghiniHuracanGt3Evo2
            | CarName::LexusRcFGt3
            | CarName::Mclaren650sGt3
            | CarName::Mclaren720sGt3
            | CarName::Mclaren720sGt3Evo
            | CarName::MercedesAmgGt3
            | CarName::MercedesAmgGt3Evo
            | CarName::NissanGtRGt32017
            | CarName::NissanGtRGt32018
            | CarName::Porsche991Gt3R
            | CarName::Porsche991iiGt3R
            | CarName::Porsche992Gt3R
            | CarName::LamborghiniGallardoRex => CarCategory::GT3,
            CarName::AlpineA110Gt4
            | CarName::AmrV8VantageGt4
            | CarName::AudiR8Gt4
            | CarName::BmwM4Gt4
            | CarName::ChevroletCamaroGt4r
            | CarName::GinettaG55Gt4
            | CarName::KtmXbowGt4
            | CarName::MaseratiMcGt4
            | CarName::Mclaren570sGt4
            | CarName::MercedesAmgGt4
            | CarName::Porsche718CaymanGt4Mr => CarCategory::GT4,
            CarName::AudiR8LmsGt2
            | CarName::KtmXbowGt2
            | CarName::MaseratiMc20Gt2
            | CarName::MercedesAmgGt2
            | CarName::Porsche935
            | CarName::Porsche991Gt2RsMr => CarCategory::GT2,
        }
    }

    fn from_str(car_model: &str) -> Option<CarName> {
        match car_model {
            "porsche_991ii_gt3_cup" => Some(CarName::Porsche991iiGt3Cup),
            "porsche_992_gt3_cup" => Some(CarName::Porsche992Gt3Cup),
            "lamborghini_huracan_st" => Some(CarName::LamborghiniHuracanSt),
            "lamborghini_huracan_st_evo2" => Some(CarName::LamborghiniHuracanStEvo2),
            "ferrari_488_challenge_evo" => Some(CarName::Ferrari488ChallengeEvo),
            "bmw_m2_cs_racing" => Some(CarName::BmwM2CsRacing),
            "amr_v12_vantage_gt3" => Some(CarName::AmrV12VantageGt3),
            "amr_v8_vantage_gt3" => Some(CarName::AmrV8VantageGt3),
            "audi_r8_lms" => Some(CarName::AudiR8Lms),
            "audi_r8_lms_evo" => Some(CarName::AudiR8LmsEvo),
            "audi_r8_lms_evo_ii" => Some(CarName::AudiR8LmsEvoIi),
            "bentley_continental_gt3_2016" => Some(CarName::BentleyContinentalGt32016),
            "bentley_continental_gt3_2018" => Some(CarName::BentleyContinentalGt32018),
            "bmw_m4_gt3" => Some(CarName::BmwM4Gt3),
            "bmw_m6_gt3" => Some(CarName::BmwM6Gt3),
            "jaguar_g3" => Some(CarName::JaguarG3),
            "ferrari_296_gt3" => Some(CarName::Ferrari296Gt3),
            "ferrari_488_gt3" => Some(CarName::Ferrari488Gt3),
            "ferrari_488_gt3_evo" => Some(CarName::Ferrari488Gt3Evo),
            "ford_mustang_gt3" => Some(CarName::FordMustangGt3),
            "honda_nsx_gt3" => Some(CarName::HondaNsxGt3),
            "honda_nsx_gt3_evo" => Some(CarName::HondaNsxGt3Evo),
            "lamborghini_huracan_gt3" => Some(CarName::LamborghiniHuracanGt3),
            "lamborghini_huracan_gt3_evo" => Some(CarName::LamborghiniHuracanGt3Evo),
            "lamborghini_huracan_gt3_evo2" => Some(CarName::LamborghiniHuracanGt3Evo2),
            "lexus_rc_f_gt3" => Some(CarName::LexusRcFGt3),
            "mclaren_650s_gt3" => Some(CarName::Mclaren650sGt3),
            "mclaren_720s_gt3" => Some(CarName::Mclaren720sGt3),
            "mclaren_720s_gt3_evo" => Some(CarName::Mclaren720sGt3Evo),
            "mercedes_amg_gt3" => Some(CarName::MercedesAmgGt3),
            "mercedes_amg_gt3_evo" => Some(CarName::MercedesAmgGt3Evo),
            "nissan_gt_r_gt3_2017" => Some(CarName::NissanGtRGt32017),
            "nissan_gt_r_gt3_2018" => Some(CarName::NissanGtRGt32018),
            "porsche_991_gt3_r" => Some(CarName::Porsche991Gt3R),
            "porsche_991ii_gt3_r" => Some(CarName::Porsche991iiGt3R),
            "porsche_992_gt3_r" => Some(CarName::Porsche992Gt3R),
            "lamborghini_gallardo_rex" => Some(CarName::LamborghiniGallardoRex),
            "alpine_a110_gt4" => Some(CarName::AlpineA110Gt4),
            "amr_v8_vantage_gt4" => Some(CarName::AmrV8VantageGt4),
            "audi_r8_gt4" => Some(CarName::AudiR8Gt4),
            "bmw_m4_gt4" => Some(CarName::BmwM4Gt4),
            "chevrolet_camaro_gt4r" => Some(CarName::ChevroletCamaroGt4r),
            "ginetta_g55_gt4" => Some(CarName::GinettaG55Gt4),
            "ktm_xbow_gt4" => Some(CarName::KtmXbowGt4),
            "maserati_mc_gt4" => Some(CarName::MaseratiMcGt4),
            "mclaren_570s_gt4" => Some(CarName::Mclaren570sGt4),
            "mercedes_amg_gt4" => Some(CarName::MercedesAmgGt4),
            "porsche_718_cayman_gt4_mr" => Some(CarName::Porsche718CaymanGt4Mr),
            "audi_r8_lms_gt2" => Some(CarName::AudiR8LmsGt2),
            "ktm_xbow_gt2" => Some(CarName::KtmXbowGt2),
            "maserati_mc20_gt2" => Some(CarName::MaseratiMc20Gt2),
            "mercedes_amg_gt2" => Some(CarName::MercedesAmgGt2),
            "porsche_935" => Some(CarName::Porsche935),
            "porsche_991_gt2_rs_mr" => Some(CarName::Porsche991Gt2RsMr),
            _ => None,
        }
    }
}

use std::fmt;

impl Display for CarName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let display_name = match self {
            CarName::Porsche991iiGt3Cup => "Porsche 991 II GT3 Cup",
            CarName::Porsche992Gt3Cup => "Porsche 992 GT3 Cup",
            CarName::LamborghiniHuracanSt => "Lamborghini Huracan ST",
            CarName::LamborghiniHuracanStEvo2 => "Lamborghini Huracan ST Evo2",
            CarName::Ferrari488ChallengeEvo => "Ferrari 488 Challenge Evo",
            CarName::BmwM2CsRacing => "BMW M2 CS Racing",
            CarName::AmrV12VantageGt3 => "AMR V12 Vantage GT3",
            CarName::AmrV8VantageGt3 => "AMR V8 Vantage GT3",
            CarName::AudiR8Lms => "Audi R8 LMS",
            CarName::AudiR8LmsEvo => "Audi R8 LMS Evo",
            CarName::AudiR8LmsEvoIi => "Audi R8 LMS Evo II",
            CarName::BentleyContinentalGt32016 => "Bentley Continental GT3 2016",
            CarName::BentleyContinentalGt32018 => "Bentley Continental GT3 2018",
            CarName::BmwM4Gt3 => "BMW M4 GT3",
            CarName::BmwM6Gt3 => "BMW M6 GT3",
            CarName::JaguarG3 => "Jaguar G3",
            CarName::Ferrari296Gt3 => "Ferrari 296 GT3",
            CarName::Ferrari488Gt3 => "Ferrari 488 GT3",
            CarName::Ferrari488Gt3Evo => "Ferrari 488 GT3 Evo",
            CarName::FordMustangGt3 => "Ford Mustang GT3",
            CarName::HondaNsxGt3 => "Honda NSX GT3",
            CarName::HondaNsxGt3Evo => "Honda NSX GT3 Evo",
            CarName::LamborghiniHuracanGt3 => "Lamborghini Huracan GT3",
            CarName::LamborghiniHuracanGt3Evo => "Lamborghini Huracan GT3 Evo",
            CarName::LamborghiniHuracanGt3Evo2 => "Lamborghini Huracan GT3 Evo2",
            CarName::LexusRcFGt3 => "Lexus RC F GT3",
            CarName::Mclaren650sGt3 => "McLaren 650S GT3",
            CarName::Mclaren720sGt3 => "McLaren 720S GT3",
            CarName::Mclaren720sGt3Evo => "McLaren 720S GT3 Evo",
            CarName::MercedesAmgGt3 => "Mercedes AMG GT3",
            CarName::MercedesAmgGt3Evo => "Mercedes AMG GT3 Evo",
            CarName::NissanGtRGt32017 => "Nissan GT-R GT3 2017",
            CarName::NissanGtRGt32018 => "Nissan GT-R GT3 2018",
            CarName::Porsche991Gt3R => "Porsche 991 GT3 R",
            CarName::Porsche991iiGt3R => "Porsche 991 II GT3 R",
            CarName::Porsche992Gt3R => "Porsche 992 GT3 R",
            CarName::LamborghiniGallardoRex => "Lamborghini Gallardo R-EX",
            CarName::AlpineA110Gt4 => "Alpine A110 GT4",
            CarName::AmrV8VantageGt4 => "AMR V8 Vantage GT4",
            CarName::AudiR8Gt4 => "Audi R8 GT4",
            CarName::BmwM4Gt4 => "BMW M4 GT4",
            CarName::ChevroletCamaroGt4r => "Chevrolet Camaro GT4R",
            CarName::GinettaG55Gt4 => "Ginetta G55 GT4",
            CarName::KtmXbowGt4 => "KTM X-Bow GT4",
            CarName::MaseratiMcGt4 => "Maserati MC GT4",
            CarName::Mclaren570sGt4 => "McLaren 570S GT4",
            CarName::MercedesAmgGt4 => "Mercedes AMG GT4",
            CarName::Porsche718CaymanGt4Mr => "Porsche 718 Cayman GT4 MR",
            CarName::AudiR8LmsGt2 => "Audi R8 LMS GT2",
            CarName::KtmXbowGt2 => "KTM X-Bow GT2",
            CarName::MaseratiMc20Gt2 => "Maserati MC20 GT2",
            CarName::MercedesAmgGt2 => "Mercedes AMG GT2",
            CarName::Porsche935 => "Porsche 935",
            CarName::Porsche991Gt2RsMr => "Porsche 991 GT2 RS MR",
        };
        write!(f, "{}", display_name)
    }
}

#[derive(Debug)]
pub(crate) struct Car {
    pub(crate) name: CarName,
    pub(crate) category: CarCategory,
}

impl Car {
    pub(crate) fn from_str(car_model: &str) -> Option<Car> {
        CarName::from_str(car_model).map(|name| {
            let category = name.get_car_category();
            Car { name, category }
        })
    }
}
