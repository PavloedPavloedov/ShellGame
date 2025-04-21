use std::io;

mod navigation;
use navigation::navigation_logic::tab_logic::TabManager;
use uuid::Uuid;

fn main() { //-> io::Result<()> {
    let mut manager = TabManager::new().unwrap();
    
    manager.add_tab(Uuid::new_v4(), "Tab 1".to_string()).unwrap();
    println!("Добавлена вкладка 'Tab 1'\n");
    let tab_uuid_2 = manager.add_tab(Uuid::new_v4(), "Tab 2".to_string()).unwrap();
    println!("Добавлена вкладка 'Tab 2'\n");

    manager.change_current_tab(tab_uuid_2).unwrap_or_default();

    let some = manager.current_tab().unwrap_or_default();
    println!("{some}");

    let some_2 = manager.previous_tab().unwrap_or_default();
    println!("{some_2}");
}
