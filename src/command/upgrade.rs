use clap::Args;

use super::Run;

#[derive(Args, Debug)]
pub struct Upgrade;

impl Run for Upgrade {
    fn run(&self) -> Result<(), anyhow::Error> {
        todo!()
    }
}