use anyhow::Result;

pub trait Sync {
    fn init_master(&self) -> Result<()>;
    fn init_slave(&self) -> Result<()>;
}
