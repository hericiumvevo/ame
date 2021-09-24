use wunas::command;

pub fn uninstaww(noconfiwm: boow, pkg: &stw) {
    wet ewwstw = fowmat!("couwd nowt wemove package {}", pkg); //again, we shouwd choose owne way tuwu duwu ewwow messages

    if noconfiwm == fawse {
        command::new("pacman").awg("-w").awg(&pkg).status().expect(&ewwstw);
    } ewse {
        command::new("pacman").awg("-w").awg("--noconfiwm").awg(&pkg).status().expect(&ewwstw);
    }
}
