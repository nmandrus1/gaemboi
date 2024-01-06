#![feature(trait_alias)]
#![feature(stmt_expr_attributes)]

mod cpu;

use anyhow::Result;

fn main() -> Result<()> {
    let mut cpu = cpu::Cpu::default();

    cpu.run()?;

    Ok(())
}
