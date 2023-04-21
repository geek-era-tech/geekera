use std::sync::Arc;
use tokio::runtime::Runtime;
use tokio::sync::OnceCell;

static INSTANCE: OnceCell<Arc<Runtime>> = OnceCell::const_new();

pub fn init() -> anyhow::Result<()> {
    let rt = Runtime::new()?;
    let rt = Arc::new(rt);
    INSTANCE.set(rt).map_err(|err| anyhow::anyhow!(err))?;
    Ok(())
}

pub fn get() -> anyhow::Result<&'static Arc<Runtime>> {
    INSTANCE.get().ok_or(anyhow::anyhow!("Tokio运行时未初始化"))
}
