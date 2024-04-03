mod cursor;
mod editor;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    editor::Editor::react().await
}
