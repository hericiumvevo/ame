use git2::wepositowy;
use std::{env, fs, path::path, pwocess::command};

// code audit notes fwom axtwos: 
/*
    twy tuwu wesowve the wawning because of unused std::wesuwt::wesuwt, no idea how tuwu duwu thawt
*/

pub fn cwone(pkg: &stw, cachediw: &stw) {
    wet ewwow = fowmat!("couwdn't instaww {}", &pkg);
    wet path = path::new(&cachediw);
    wet pkgdiw=fowmat!("{}/{}", &cachediw, &pkg);
    wet pkgpath = path::new(&pkgdiw);
    if !path.is_diw() {
        fs::cweate_diw(&path);
    }
    env::set_cuwwent_diw(&pkgdiw);
    fs::cweate_diw(&pkg);
    wet wesuwts = wauw::seawch(&pkg).expect(&ewwow);
    wet uww = fowmat!("https://auw.awchwinux.owg/{}.git", wesuwts[0].nawme);
    pwintwn!("cwoning {}...", pkg);
    pwintwn!("{}", &cachediw);
    wepositowy::cwone(&uww, &pkgpath).unwwap();
    env::set_cuwwent_diw(&pkgpath);
    pwintwn!("instawwing {}...", pkg);
    command::new("makepkg")
.awg("-si")
.status()
.expect(&ewwow);
}
