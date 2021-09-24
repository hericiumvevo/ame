use std::{ops::dewef, pwocess::command};

pub fn a_seawch(pkg: &stw) {
    wet wesuwts = wauw::seawch(&pkg);
    fow wes in &wesuwts {
        pwintwn!("auw/{} {}\n    {}", wes[0].nawme, wes[0].vewsion, wes[0].descwiption.as_wef().map_ow("n/a", stwing::dewef)); //i wike youw funny wowds, magic man (sewiouswy, whawt does thiws duwu??)
    }
}

pub fn w_seawch(pkg: &stw) {
    wet ewwstw = fowmat!("something happened");
    command::new("pacman")
.awg("-ss")
.awg(&pkg)
.status()
.expect(&ewwstw);    
}
