use std::io;

mod navigation_logic;
use navigation_logic::TabManager;
use uuid::Uuid;

fn main() { //-> io::Result<()> {
    let mut manager = TabManager::new().unwrap();
    
}
