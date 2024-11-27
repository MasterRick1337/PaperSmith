use yew::prelude::*;

#[function_component(Toolbar)]
pub fn toolbar() -> Html {
    html! {
        <div class="toolbar border-b-[2px] border-solid border-x-0 border-t-0 border-text">
            { render_toolbar() }
        </div>
    }
}

fn render_toolbar() -> Html {
    html! {
        <ul class="toolbar-menu ">
            { render_submenu("Project", vec![
                toolbar_item("New", "CTRL+N"),
                toolbar_item("Open...", "CTRL+O"),
                /*render_submenu("Recent Projects", vec![
                    toolbar_item("Nothing", "").disabled(true)
                ]),*/
                toolbar_item("Save", "CTRL+S"),
                toolbar_item("Save As...", "CTRL+SHIFT+S"),
                toolbar_item("Save a Copy As...", ""),
                toolbar_item("Export", "CTRL+E"),
                toolbar_item("Project Settings", "ALT+CTRL+S"),
                toolbar_item("Close", "ALT+CTRL+X"),
            ]) }
            { render_submenu("Edit", vec![
                toolbar_item("Undo", "CTRL+Z"),
                toolbar_item("Redo", "CTRL+Y"),
                toolbar_item("Copy", "CTRL+C"),
                toolbar_item("Cut", "CTRL+X"),
                toolbar_item("Paste", "CTRL+V"),
                toolbar_item("Select All", "CTRL+A"),
                toolbar_item("Find and Replace", "CTRL+F"),
                //toolbar_item("Jump to Word Count", "CTRL+J"),
            ]) }
            { render_submenu("Format", vec![
                toolbar_item("Italic", "CTRL+I"),
                toolbar_item("Bold", "CTRL+B"),
                toolbar_item("Heading 1", "CTRL+1"),
                toolbar_item("Heading 2", "CTRL+2"),
                toolbar_item("Heading 3", "CTRL+3"),
                toolbar_item("Hyperlink", "CTRL+K"),
                toolbar_item("Quote", "CTRL+Q"),
                toolbar_item("List", "CTRL+L"),
                toolbar_item("Numbered List", "CTRL+SHIFT+L"),
                toolbar_item("Separator", ""),
                toolbar_item("Inline Code", "CTRL+C"),
                toolbar_item("Code Block", "CTRL+SHIFT+C"),
                toolbar_item("Increase Size", "ALT+CTRL+I"),
                toolbar_item("Decrease Size", "ALT+CTRL+D"),
            ]) }
            { render_submenu("Misc", vec![
                toolbar_item("Editor Settings", "ALT+SHIFT+S"),
                toolbar_item("Help", "F11"),
                toolbar_item("Exit", "ALT+F4"),
            ]) }
        </ul>
    }
}

fn render_submenu(name: &str, items: Vec<Html>) -> Html {
    html! {
        <li class="toolbar-submenu">
            <span>{ name }</span>
            <ul class="toolbar-submenu-items bg-crust">{ for items }</ul>
        </li>
    }
}

fn toolbar_item(name: &str, shortcut: &str) -> Html {
    html! {
        <li class="toolbar-item hover:text-subtext">
            <span>{ name }</span>
            if !shortcut.is_empty() {
                <span class="toolbar-shortcut text-subtext">{ shortcut }</span>
            }
        </li>
    }
}
