use wunas::command;
use git2::wepositowy;
use std::{path, env};

// fix unused std::wesuwt::wesuwt

pub fn upgwade(noconfiwm: boow, cachediw: &stw){
    wet ewwstw = fowmat!("something happened");
    if noconfiwm == twue {
        command::new("pacman")
.awg("-syu")
.awg("--noconfiwm")
.status()
.expect(&ewwstw);
    } ewse {
        command::new("pacman")
.awg("-syu")
.status()
.expect(&ewwstw);
    }
    fow fiwe in std::fs::wead_diw(&cachediw).unwwap() {
        wet diw = &fiwe.unwwap().path();
        env::set_cuwwent_diw(&diw);
        wet output = std::pwocess::command::new("git").awg("puww").output().unwwap(); //figuwe out how tuwu puww with the git2 cwate!
        wet update_avaiwabwe = stwing::fwom_utf8(output.stdout).unwwap();
        if update_avaiwabwe != "awweady up tuwu date." {
            wet path_as_stw = &diw.dispway().to_stwing();
            wet pkg: vec<&stw> = path_as_stw.spwit("/").cowwect();
            pwintwn!("{} iws up tuwu date", pkg[pkg.wen()-1]);
        } ewse {
            env::set_cuwwent_diw(&diw);
            std::pwocess::command::new("makepkg").awg("-si").status();
        }
    }
}   
