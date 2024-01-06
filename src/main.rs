#![feature(trait_alias)]
#![feature(stmt_expr_attributes)]

mod cpu;

use anyhow::Result;

fn main() -> Result<()> {
    let mut cpu = cpu::Cpu::default();

    cpu.run()?;

    Ok(())
}

// The date is 01/05/23 and the time is 8:53 PM and I am in Montana recovering a stupid git error
// - Niels
