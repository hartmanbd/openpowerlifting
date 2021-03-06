//! Defines the `Federation` field for the `meets` table.

use std::fmt;

#[derive(Copy, Clone, Deserialize, PartialEq, Serialize, EnumIterator)]
pub enum Federation {
    #[serde(rename = "365Strong")]
    _365Strong,
    AAPF,
    AAU,
    ADFPA,
    AEP,
    AfricanPF,
    APA,
    APC,
    APF,
    AsianPF,
    BB,
    BPU,
    BP,
    CAPO,
    CommonwealthPF,
    CPF,
    CPL,
    CPU,
    DSF,
    EPA,
    EPF,
    FEMEPO,
    FESUPO,
    FFForce,
    FPO,
    FPR,
    GPA,
    GPC,
    #[serde(rename = "GPC-AUS")]
    GPCAUS,
    #[serde(rename = "GPC-GB")]
    GPCGB,
    #[serde(rename = "GPC-NZ")]
    GPCNZ,
    HERC,
    IDFPF,
    IPA,
    IPF,
    IPL,
    IrishPF,
    IrishPO,
    KRAFT,
    MHP,
    MM,
    NAPF,
    NASA,
    NIPF,
    NPA,
    NSF,
    NZPF,
    OceaniaPF,
    ParaPL,
    PA,
    PLZS,
    ProRaw,
    PZKFiTS,
    RAW,
    RAWU,
    RPS,
    RUPC,
    ScottishPL,
    SCT,
    SPA,
    SPF,
    SVNL,
    THSPA,
    UPA,
    USAPL,
    USPF,
    USPA,
    WABDL,
    WDFPF,
    WelshPA,
    WPA,
    WPAU,
    WPC,
    #[serde(rename = "WPC-RUS")]
    WPCRUS,
    WNPF,
    WRPF,
    #[serde(rename = "WRPF-AUS")]
    WRPFAUS,
    #[serde(rename = "WRPF-CAN")]
    WRPFCAN,
    WUAP,
    XPC,
}

impl fmt::Display for Federation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Federation::_365Strong => write!(f, "365Strong"),
            Federation::AAPF => write!(f, "AAPF"),
            Federation::AAU => write!(f, "AAU"),
            Federation::ADFPA => write!(f, "ADFPA"),
            Federation::AEP => write!(f, "AEP"),
            Federation::AfricanPF => write!(f, "AfricanPF"),
            Federation::APA => write!(f, "APA"),
            Federation::APC => write!(f, "APC"),
            Federation::APF => write!(f, "APF"),
            Federation::AsianPF => write!(f, "AsianPF"),
            Federation::BB => write!(f, "BB"),
            Federation::BPU => write!(f, "BPU"),
            Federation::BP => write!(f, "BP"),
            Federation::CAPO => write!(f, "CAPO"),
            Federation::CommonwealthPF => write!(f, "CommonwealthPF"),
            Federation::CPF => write!(f, "CPF"),
            Federation::CPL => write!(f, "CPL"),
            Federation::CPU => write!(f, "CPU"),
            Federation::DSF => write!(f, "DSF"),
            Federation::EPA => write!(f, "EPA"),
            Federation::EPF => write!(f, "EPF"),
            Federation::FEMEPO => write!(f, "FEMEPO"),
            Federation::FESUPO => write!(f, "FESUPO"),
            Federation::FFForce => write!(f, "FFForce"),
            Federation::FPO => write!(f, "FPO"),
            Federation::FPR => write!(f, "FPR"),
            Federation::GPA => write!(f, "GPA"),
            Federation::GPC => write!(f, "GPC"),
            Federation::GPCGB => write!(f, "GPC-GB"),
            Federation::GPCAUS => write!(f, "GPC-AUS"),
            Federation::GPCNZ => write!(f, "GPC-NZ"),
            Federation::HERC => write!(f, "HERC"),
            Federation::IDFPF => write!(f, "IDFPF"),
            Federation::IPA => write!(f, "IPA"),
            Federation::IPF => write!(f, "IPF"),
            Federation::IPL => write!(f, "IPL"),
            Federation::IrishPF => write!(f, "IrishPF"),
            Federation::IrishPO => write!(f, "IrishPO"),
            Federation::KRAFT => write!(f, "KRAFT"),
            Federation::MHP => write!(f, "MHP"),
            Federation::MM => write!(f, "MM"),
            Federation::NAPF => write!(f, "NAPF"),
            Federation::NASA => write!(f, "NASA"),
            Federation::NIPF => write!(f, "NIPF"),
            Federation::NPA => write!(f, "NPA"),
            Federation::NSF => write!(f, "NSF"),
            Federation::NZPF => write!(f, "NZPF"),
            Federation::OceaniaPF => write!(f, "OceaniaPF"),
            Federation::ParaPL => write!(f, "ParaPL"),
            Federation::PA => write!(f, "PA"),
            Federation::PLZS => write!(f, "PLZS"),
            Federation::ProRaw => write!(f, "ProRaw"),
            Federation::PZKFiTS => write!(f, "PZKFiTS"),
            Federation::RAW => write!(f, "RAW"),
            Federation::RAWU => write!(f, "RAWU"),
            Federation::RPS => write!(f, "RPS"),
            Federation::RUPC => write!(f, "RUPC"),
            Federation::ScottishPL => write!(f, "ScottishPL"),
            Federation::SCT => write!(f, "SCT"),
            Federation::SPA => write!(f, "SPA"),
            Federation::SPF => write!(f, "SPF"),
            Federation::SVNL => write!(f, "SVNL"),
            Federation::THSPA => write!(f, "THSPA"),
            Federation::UPA => write!(f, "UPA"),
            Federation::USAPL => write!(f, "USAPL"),
            Federation::USPF => write!(f, "USPF"),
            Federation::USPA => write!(f, "USPA"),
            Federation::WABDL => write!(f, "WABDL"),
            Federation::WDFPF => write!(f, "WDFPF"),
            Federation::WelshPA => write!(f, "WelshPA"),
            Federation::WPA => write!(f, "WPA"),
            Federation::WPAU => write!(f, "WPAU"),
            Federation::WPC => write!(f, "WPC"),
            Federation::WPCRUS => write!(f, "WPC-RUS"),
            Federation::WNPF => write!(f, "WNPF"),
            Federation::WRPF => write!(f, "WRPF"),
            Federation::WRPFAUS => write!(f, "WRPF-AUS"),
            Federation::WRPFCAN => write!(f, "WRPF-CAN"),
            Federation::WUAP => write!(f, "WUAP"),
            Federation::XPC => write!(f, "XPC"),
        }
    }
}

impl Federation {
    pub fn from_url_str(s: &str) -> Result<Self, ()> {
        match s {
            "365strong" => Ok(Federation::_365Strong),
            "aapf" => Ok(Federation::AAPF),
            "aau" => Ok(Federation::AAU),
            "adfpa" => Ok(Federation::ADFPA),
            "aep" => Ok(Federation::AEP),
            "africanpf" => Ok(Federation::AfricanPF),
            "apa" => Ok(Federation::APA),
            "apc" => Ok(Federation::APC),
            "apf" => Ok(Federation::APF),
            "asianpf" => Ok(Federation::AsianPF),
            "bb" => Ok(Federation::BB),
            "bpu" => Ok(Federation::BPU),
            "bp" => Ok(Federation::BP),
            "capo" => Ok(Federation::CAPO),
            "commonwealthpf" => Ok(Federation::CommonwealthPF),
            "cpf" => Ok(Federation::CPF),
            "cpl" => Ok(Federation::CPL),
            "cpu" => Ok(Federation::CPU),
            "dsf" => Ok(Federation::DSF),
            "epa" => Ok(Federation::EPA),
            "epf" => Ok(Federation::EPF),
            "femepo" => Ok(Federation::FEMEPO),
            "fesupo" => Ok(Federation::FESUPO),
            "ffforce" => Ok(Federation::FFForce),
            "fpo" => Ok(Federation::FPO),
            "fpr" => Ok(Federation::FPR),
            "gpa" => Ok(Federation::GPA),
            "gpc" => Ok(Federation::GPC),
            "gpcgb" => Ok(Federation::GPCGB),
            "gpcaus" => Ok(Federation::GPCAUS),
            "gpcnz" => Ok(Federation::GPCNZ),
            "herc" => Ok(Federation::HERC),
            "idfpf" => Ok(Federation::IDFPF),
            "ipa" => Ok(Federation::IPA),
            "ipf" => Ok(Federation::IPF),
            "ipl" => Ok(Federation::IPL),
            "irishpf" => Ok(Federation::IrishPF),
            "irishpo" => Ok(Federation::IrishPO),
            "kraft" => Ok(Federation::KRAFT),
            "mhp" => Ok(Federation::MHP),
            "mm" => Ok(Federation::MM),
            "napf" => Ok(Federation::NAPF),
            "nasa" => Ok(Federation::NASA),
            "nipf" => Ok(Federation::NIPF),
            "npa" => Ok(Federation::NPA),
            "nsf" => Ok(Federation::NSF),
            "nzpf" => Ok(Federation::NZPF),
            "oceaniapf" => Ok(Federation::OceaniaPF),
            "parapl" => Ok(Federation::ParaPL),
            "pa" => Ok(Federation::PA),
            "plzs" => Ok(Federation::PLZS),
            "proraw" => Ok(Federation::ProRaw),
            "pzkfits" => Ok(Federation::PZKFiTS),
            "100raw" => Ok(Federation::RAW),
            "rawu" => Ok(Federation::RAWU),
            "rps" => Ok(Federation::RPS),
            "rupc" => Ok(Federation::RUPC),
            "scottishpl" => Ok(Federation::ScottishPL),
            "sct" => Ok(Federation::SCT),
            "spa" => Ok(Federation::SPA),
            "spf" => Ok(Federation::SPF),
            "svnl" => Ok(Federation::SVNL),
            "thspa" => Ok(Federation::THSPA),
            "upa" => Ok(Federation::UPA),
            "usapl" => Ok(Federation::USAPL),
            "uspf" => Ok(Federation::USPF),
            "uspa" => Ok(Federation::USPA),
            "wabdl" => Ok(Federation::WABDL),
            "wdfpf" => Ok(Federation::WDFPF),
            "welshpa" => Ok(Federation::WelshPA),
            "wpa" => Ok(Federation::WPA),
            "wpau" => Ok(Federation::WPAU),
            "wpc" => Ok(Federation::WPC),
            "wpcrus" => Ok(Federation::WPCRUS),
            "wnpf" => Ok(Federation::WNPF),
            "wrpf" => Ok(Federation::WRPF),
            "wrpfaus" => Ok(Federation::WRPFAUS),
            "wrpfcan" => Ok(Federation::WRPFCAN),
            "wuap" => Ok(Federation::WUAP),
            "xpc" => Ok(Federation::XPC),
            _ => Err(()),
        }
    }
}
