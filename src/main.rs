use std::path::PathBuf;

use structopt::StructOpt;

use crate::{
    hw::Hardware,
    loader::{load_rom, Loader},
};

mod alu;
mod cartridge;
mod cpu;
mod hw;
mod loader;
mod mmu;
mod opcodes;
mod timer;
mod utils;

fn main() {
    let opt = Opt::from_args();

    let hw = Hardware::new(opt.ram.clone());
    let hw1 = hw.clone();

    std::thread::spawn(move || {
        let (rom, hw1) = if opt.rom.is_dir() {
            let mut ldr = Loader::new(&opt.rom);

            utils::select(&mut ldr, hw1)
        } else {
            (load_rom(&opt.rom), hw1)
        };

        set_affinity();

        rgy::run(to_cfg(opt), &rom, hw1);
    });

    hw.run();
}

#[derive(Debug, StructOpt)]
pub struct Opt {
    /// Cpu frequency
    #[structopt(short = "f", long = "freq", default_value = "4200000")]
    freq: u64,
    /// Sampling rate for cpu frequency controller
    #[structopt(short = "s", long = "sample", default_value = "4200")]
    sample: u64,
    /// Delay unit for cpu frequency controller
    #[structopt(short = "u", long = "delayunit", default_value = "50")]
    delay_unit: u64,
    /// Don't adjust cpu frequency
    #[structopt(short = "n", long = "native")]
    native_speed: bool,
    /// Enable debug mode
    #[structopt(short = "d", long = "debug")]
    debug: bool,
    /// RAM file name
    #[structopt(short = "r", long = "ram")]
    ram: Option<String>,
    /// ROM file name or directory
    #[structopt(name = "ROM")]
    rom: PathBuf,
}

fn to_cfg(opt: Opt) -> rgy::Config {
    rgy::Config::new()
        .freq(opt.freq)
        .sample(opt.sample)
        .delay_unit(opt.delay_unit)
        .native_speed(opt.native_speed)
}

fn set_affinity() {
    let set = || {
        let core_ids = core_affinity::get_core_ids()?;
        core_affinity::set_for_current(*core_ids.get(0)?);
        Some(())
    };

    match set() {
        _ => {}
    }
}
