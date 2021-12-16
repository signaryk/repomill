extern crate config;
extern crate serde;

#[macro_use]
extern crate serde_derive;

mod repoconf;
use repoconf::RepoConf;

fn main() {
    let rc = match RepoConf::new() {
        Ok(rc) => rc,
        Err(e) => panic!("Error: {}", e),
    };

    println!("Number of repos: {}", rc.len());
}
