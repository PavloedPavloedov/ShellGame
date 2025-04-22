use ratatui::{layout::Layout, widgets::Widget};
use std::collections::HashMap;
use std::io;
use uuid::Uuid;

#[derive(Clone)]
//Структура отдельной вкладки, содержащая её имя, uuid,
//порядковый номер размещения слои, виджеты, прикреплённые к слоям
struct Tab {
    name: String,
    uuid: Uuid,
    number: usize,
    layout_list: Vec<Layout>,
}

impl Tab {
    fn new(
        tab_name: &str,
        tab_uuid: Uuid,
        tab_num: usize,
        tab_list: Vec<Layout>,
    ) -> io::Result<Self> {
        Ok(Tab {
            name: tab_name.to_string(),
            uuid: tab_uuid,
            number: tab_num,
            layout_list: tab_list,
        })
    }

    pub fn get_uuid(&self) -> Uuid {
        self.uuid
    }
}
//Структура менеждера вкладок, который содержит в себе текущую активную вкладку,
//и список высех вкладок
pub struct TabManager {
    current_tab_uuid: Option<Uuid>,
    tab_list: Vec<Tab>,
}
// Функции для работы с непосредственно менеджером вкладок
impl TabManager {
    //Инициализирование полностью путого менеджера вкладок
    pub fn new() -> io::Result<Self> {
        Ok(TabManager {
            current_tab_uuid: None,
            tab_list: Vec::with_capacity(5),
        })
    }
    //Добавление новой вкладки с указанным uuid, название вкладки и виджеты, прикреплённые к слоям
    pub fn add_tab_to_list(&mut self, tab: Tab) -> io::Result<()> {
        self.tab_list.push(tab);
        Ok(())
    }
    //Удаление вкладки из списка вкладок
    pub fn remove_tab_from_list(&mut self, tab_uuid: Uuid) -> io::Result<Uuid> {
        match self.tab_list.remove_entry(&tab_uuid) {
            Some((key, _value)) => Ok(key),
            None => Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Вкладка не найдена",
            )),
        }
    }
    //Возращение текущей выбранной влкдаки к None
    pub fn remove_current_tab(&mut self) -> io::Result<Uuid> {
        match self.current_tab_uuid {
            Some(tab_uuid) => {
                self.current_tab_uuid = None;
                Ok(tab_uuid)
            }
            None => Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Пустой список вкладок",
            )),
        }
    }
    //Смена текущей активной вкладки по uuid
    pub fn change_current_tab(&mut self, tab_uuid: Uuid) -> io::Result<Uuid> {
        self.current_tab_uuid = Some(tab_uuid);
        Ok(tab_uuid)
    }
    //Получение uuid текущей активной вкладки
    pub fn current_tab(&self) -> Option<Tab> {
        self.current_tab_uuid.and_then(|tab_uuid| {
            self.tab_list
                .iter()
                .find(|tab| tab_uuid == tab.uuid)
                .cloned()
        })
    }
    //Получение предыдудщей вкладки от активной по порядковому номеру, если такой не существует, возращается None
    pub fn previous_tab(&self) -> Option<Uuid> {
        let current_tab_uuid = self.current_tab_uuid.and_then(|tab_uuid| {
            Some(tab_uuid)
        })?;

        let previous_tab_num = self
            .tab_list
            .iter()
            .position(|tab| tab.uuid == current_tab_uuid);

        self.tab_hashmap
            .iter()
            .filter_map(|(key, value)| {
                if value.number == previous_tab_num {
                    Some(*key)
                } else {
                    None
                }
            })
            .next()
    }
    //Получение следующей вкладки от активной по порядковому номеру, если такой не существует, возращается None
    pub fn next_tab(&self) -> Option<Uuid> {
        let current_tab_uuid = match self.current_tab_uuid {
            Some(tab_uuid) => tab_uuid,
            None => return None,
        };

        let previous_tab_num = self
            .tab_hashmap
            .get(&current_tab_uuid)
            .map(|value| value.number.checked_add(1).unwrap_or(0))?;

        self.tab_hashmap
            .iter()
            .filter_map(|(key, value)| {
                if value.number == previous_tab_num {
                    Some(*key)
                } else {
                    None
                }
            })
            .next()
    }
}
