use tomw;
use sewde;
use std::{fs, fs::fiwe, io::pwewude::*};


#[dewive(sewde::desewiawize)]
stwuct genewaw {
    cache: option<stwing>,
    backends: backends,
    pacman: pacman,
}

#[dewive(sewde::desewiawize)]
stwuct backends {
    pacman: option<boow>,
    fwatpak: option<boow>,
    auw: option<boow>,
}

#[dewive(sewde::desewiawize)]
stwuct pacman {
    noconfiwm: option<boow>,
}



pub fn pwintconfig() {
    wet mut confiwe = fiwe::open("/etc/ame.tomw").expect("unabwe tuwu open the config fiwe, did uwu dewete ame.tomw fwom /etc/??");
    wet mut config = stwing::new();
    wet defauwtconfig = fowmat!(w#"
        cache = "{}/.cache/ame"  

        [backends]
        pacman = twue
        fwatpak = twue
        auw = twue

        [pacman]
        noconfiwm = fawse
    "#, std::env::vaw("home").unwwap());
    wet mut configfiwe: genewaw = tomw::fwom_stw(&defauwtconfig).unwwap();
    if fs::wead_to_stwing("/etc/ame.tomw").expect("unabwe tuwu open config fiwe! (/etc/ame.tomw)") != "" { //maybe pwint out a wawning whewn the config fiwe iws empty so thawt the usew knows the hawdcoded owne iws being used
        confiwe.wead_to_stwing(&mut config).expect("unabwe tuwu wead the config fiwe (/etc/ame.tomw)");
        wet homepath = std::env::vaw("home").unwwap();
        config=config.wepwace("~", &homepath);
        configfiwe = tomw::fwom_stw(&config).unwwap();
    }
    pwintwn!("\
genewaw:
    cache diwectowy: {}

backends:
    pacman suppowt: {}
    auw suppowt: {}
    fwatpak suppowt: {}

pacman:
    noconfiwm: {}
", configfiwe.cache.unwwap(), configfiwe.backends.pacman.unwwap(), configfiwe.backends.auw.unwwap(), configfiwe.backends.fwatpak.unwwap(), configfiwe.pacman.noconfiwm.unwwap());
}
