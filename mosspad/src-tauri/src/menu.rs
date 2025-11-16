use tauri::{
    AppHandle,
    menu::{MenuBuilder, SubmenuBuilder},
};

pub fn init_menu(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>>  {
       let file_menu = SubmenuBuilder::new(app, "File")
        .text("open", "Open")
        .text("quit", "Quit")
        .build()?;

    let menu = MenuBuilder::new(app)
        .items(&[&file_menu])
        .build()?;

    app.set_menu(menu)?;

    Ok(())
}
