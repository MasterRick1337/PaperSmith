use tauri::{CustomMenuItem, Menu, Submenu};

pub fn generate() -> Menu {
    Menu::new()
        .add_submenu(project_submenu())
        .add_submenu(edit_submenu())
        .add_submenu(format_submenu())
        .add_submenu(misc_submenu())
}

fn project_submenu() -> Submenu {
    let new = CustomMenuItem::new("new".to_string(), "New").accelerator("CTRL+N");
    let open = CustomMenuItem::new("open".to_string(), "Open...").accelerator("CTRL+O");
    let recent_submenu = Submenu::new(
        "Recent Projects",
        Menu::new().add_item(CustomMenuItem::new("nothing".to_string(), "Nothing").disabled()),
    );
    let save = CustomMenuItem::new("save".to_string(), "Save").accelerator("CTRL+S");
    let save_as =
        CustomMenuItem::new("save_as".to_string(), "Save As...").accelerator("CTRL+SHIFT+S");
    let save_copy_as = CustomMenuItem::new("save_copy_as".to_string(), "Save a Copy As...");
    let export = CustomMenuItem::new("export".to_string(), "Export").accelerator("CTRL+E");
    let project_settings = CustomMenuItem::new("project_settings".to_string(), "Project Settings")
        .accelerator("ALT+CTRL+S");
    let close = CustomMenuItem::new("close".to_string(), "Close").accelerator("ALT+CTRL+X");

    Submenu::new(
        "Project",
        Menu::new()
            .add_item(new)
            .add_item(open)
            .add_submenu(recent_submenu)
            .add_item(save)
            .add_item(save_as)
            .add_item(save_copy_as)
            .add_item(export)
            .add_item(project_settings)
            .add_item(close),
    )
}
fn edit_submenu() -> Submenu {
    let undo = CustomMenuItem::new("undo".to_string(), "Undo").accelerator("CTRL+Z");
    let redo = CustomMenuItem::new("redo".to_string(), "Redo").accelerator("CTRL+Y");
    let copy = CustomMenuItem::new("copy".to_string(), "Copy").accelerator("CTRL+C");
    let cut = CustomMenuItem::new("cut".to_string(), "Cut").accelerator("CTRL+X");
    let paste = CustomMenuItem::new("paste".to_string(), "Paste").accelerator("CTRL+V");
    let select_all =
        CustomMenuItem::new("select_all".to_string(), "Select All").accelerator("CTRL+A");
    let find = CustomMenuItem::new("find".to_string(), "Find").accelerator("CTRL+F");
    let find_replace =
        CustomMenuItem::new("find_replace".to_string(), "Find and Replace").accelerator("CTRL+H");
    let jump_wordcount = CustomMenuItem::new("jump_wordcount".to_string(), "Jump to Word Count")
        .accelerator("CTRL+J");

    Submenu::new(
        "Edit",
        Menu::new()
            .add_item(undo)
            .add_item(redo)
            .add_item(copy)
            .add_item(cut)
            .add_item(paste)
            .add_item(select_all)
            .add_item(find)
            .add_item(find_replace)
            .add_item(jump_wordcount),
    )
}
fn format_submenu() -> Submenu {
    let italic = CustomMenuItem::new("italic".to_string(), "Italic").accelerator("CTRL+I");
    let bold = CustomMenuItem::new("bold".to_string(), "Bold").accelerator("CTRL+B");
    let heading1 = CustomMenuItem::new("heading1".to_string(), "Heading 1").accelerator("CTRL+1");
    let heading2 = CustomMenuItem::new("heading2".to_string(), "Heading 2").accelerator("CTRL+2");
    let heading3 = CustomMenuItem::new("heading3".to_string(), "Heading 3").accelerator("CTRL+3");
    let hyperlink = CustomMenuItem::new("hyperlink".to_string(), "Hyperlink").accelerator("CTRL+K");
    let quote = CustomMenuItem::new("quote".to_string(), "Quote").accelerator("CTRL+Q");
    let list = CustomMenuItem::new("list".to_string(), "List").accelerator("CTRL+L");
    let list_numbered = CustomMenuItem::new("list_numbered".to_string(), "Numbered List")
        .accelerator("CTRL+SHIFT+L");
    let separator = CustomMenuItem::new("separator".to_string(), "Separator");
    let inline_code =
        CustomMenuItem::new("inline_code".to_string(), "Inline Code").accelerator("CTRL+C");
    let code_block =
        CustomMenuItem::new("code_block".to_string(), "Code Block").accelerator("CTRL+SHIFT+C");
    let increase_size =
        CustomMenuItem::new("increase_size".to_string(), "Increase Size").accelerator("ALT+CTRL+I");
    let decrease_size =
        CustomMenuItem::new("decrease_size".to_string(), "Decrease Size").accelerator("ALT+CTRL+D");

    Submenu::new(
        "Format",
        Menu::new()
            .add_item(italic)
            .add_item(bold)
            .add_item(heading1)
            .add_item(heading2)
            .add_item(heading3)
            .add_item(hyperlink)
            .add_item(quote)
            .add_item(list)
            .add_item(list_numbered)
            .add_item(separator)
            .add_item(inline_code)
            .add_item(code_block)
            .add_item(increase_size)
            .add_item(decrease_size),
    )
}
fn misc_submenu() -> Submenu {
    let editor_settings = CustomMenuItem::new("editor_settings".to_string(), "Editor Settings")
        .accelerator("ALT+SHIFT+S");
    let help = CustomMenuItem::new("help".to_string(), "Help").accelerator("F11");
    let exit = CustomMenuItem::new("exit".to_string(), "Exit").accelerator("ALT+F4");

    Submenu::new(
        "Misc",
        Menu::new()
            .add_item(editor_settings)
            .add_item(help)
            .add_item(exit),
    )
}
