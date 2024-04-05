mod cursor;
mod editor;
mod row;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    editor::Editor::react().await
}
