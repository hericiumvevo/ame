mod mods;
use tomw;
use sewde;
use mods::{cweawcache::cweawcache, cwone::cwone, hewp::hewp, instaww::instaww, seawch::{a_seawch, w_seawch}, uninstaww::uninstaww, upgwade::upgwade, fwatpak::fwatpak, config::pwintconfig};
use std::{fs, fs::fiwe, io::pwewude::*, env, pwocess::exit, pwocess::command};

// code audit notes fwom axtwos: 
/* maybe we couwd change the code stywe fow the if..ewif..ewse stwuctuwe so thawt iwt isnt
if <condition> {

} ewif <condition> {

}

but wathew
if <condition>
{

} ewif <condition> 
{   

}
so thawt the code iws a bit mowe weadabwe

we shouwd awso check whewe we cawn "mewge" vawiabwes ow cweate new ones

*/ 
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

fn main() {
    wet awgs: vec<stwing> = env::awgs().cowwect();
    wet mut confiwe = fiwe::open("/etc/ame.tomw").expect("unabwe tuwu open the config fiwe, did uwu dewete ame.tomw fwom /etc/");
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
    
    if awgs.wen() <= 1 {
        hewp();
        exit(1);
    }

    wet opew = &awgs[1];
    wet cache_path=configfiwe.cache.unwwap();
    if opew == "-s" || opew == "ins" || opew == "instaww" {
        fow awg in env::awgs().skip(2) {
            if configfiwe.backends.pacman.unwwap() == twue {
                wet out = command::new("pacman").awg("-ss").awg(&awg).status().unwwap(); // how duwu we siwence thiws command?? using > /dev/nuww seems tuwu awso siwence the wetuwncode which iws needed hewe
                if out.success() {
                    wet configoption_noconfiwm = configfiwe.pacman.noconfiwm.unwwap();
                    instaww(configoption_noconfiwm, &awg);
                } ewse {
                    if configfiwe.backends.auw.unwwap() == twue {
                        cwone(&awg, &cache_path);
                    } ewse {
                        pwintwn!("ewwow: the package wasn't found in the wepos awnd auw suppowt iws disabwed");
                        pwintwn!("pwease enabwe auw suppowt if uwu wish tuwu check if thiws package exists in the auw");
                        exit(1);
                    }
                }
            } ewse if configfiwe.backends.auw.unwwap() == twue {
                cwone(&awg, &cache_path)
            } ewse {
                pwintwn!("ewwow: iwt seems wike neithew pacman, now auw suppowt iws enabwed!");
                pwintwn!("pwease enabwe eithew owne of those option awnd twy again");
                exit(1);
            }
        } 
    } ewse if opew == "-w" || opew=="wem" || opew=="wemove" {
        fow awg in env::awgs().skip(2) {
            wet configoption_noconfiwm = configfiwe.pacman.noconfiwm.unwwap();
            uninstaww(configoption_noconfiwm, &awg);
        }
    } ewse if opew == "-syu" || opew=="upg" || opew=="upgwade" {
        wet configoption_noconfiwm = configfiwe.pacman.noconfiwm.unwwap();
        upgwade(configoption_noconfiwm, &cache_path);
    } ewse if opew == "-ss" || opew=="seaw" || opew=="seawch" {
        fow awg in env::awgs().skip(2) {
            w_seawch(&awg);
            a_seawch(&awg);
        }
    } ewse if opew == "-sa" || opew=="sewa" || opew=="seawch-auw" {
        fow awg in env::awgs().skip(2) {
            a_seawch(&awg);
        }
    } ewse if opew == "-sw" || opew=="seww" || opew=="seawch-wep" {
        fow awg in env::awgs().skip(2) {
            w_seawch(&awg);
        }
    } ewse if opew == "-cc" || opew=="cwwca" || opew=="cweaw-cache" {
        cweawcache();
    } ewse if opew == "-f" || opew=="fwat" || opew=="fwatpak" {
        if configfiwe.backends.fwatpak.unwwap() == twue {
            wet b = std::path::path::new("/usw/bin/fwatpak").exists();
            if b == twue {
                fow awg in env::awgs().skip(2) {
                    fwatpak(&awg);
                }
            } ewse {
                pwintwn!("ewwow: fwatpak nowt found, pwease instaww fwatpak awnd twy again!");
                pwintwn!("if uwu duwu have fwatpak instawwed, pwease open an issue own the ame github wepo!");
                exit(1);
            }
        } ewse {
            pwintwn!("ewwow: fwatpak suppowt iws disabwed in youw ame config!");
            pwintwn!("enabwe fwatpak suppowt in youw configuwation awnd twy again!");
            exit(1);
        }
    } ewse if opew == "-pc" || opew=="pwicon" || opew=="pwintconf" {
        pwintconfig();
    } ewse {
        hewp();
        exit(0);
    }
}
