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
    main_layout: Layout,
    layout_list: HashMap<Layout, Vec<Widget>>,
}
//Структура менеждера вкладок, который содержит в себе текущую активную вкладку,
//и список высех вкладок
pub struct TabManager {
    current_tab_uuid: Option<Uuid>,
    tab_hashmap: HashMap<Uuid, Tab>,
}
// Функции для работы с непосредственно менеджером вкладок
impl TabManager {
    //Инициализирование полностью путого менеджера вкладок
    pub fn new() -> io::Result<Self> {
        let manager = TabManager {
            current_tab_uuid: None,
            tab_hashmap: HashMap::with_capacity(5),
        };
        Ok(manager)
    }
    //Добавление новой вкладки с указанным uuid, название вкладки и виджеты, прикреплённые к слоям
    pub fn add_tab(
        &mut self,
        tab_uuid: Uuid,
        tab_name: String,
        tab_layout: Layout,
        tab_layout_list: HashMap<Layout, Vec<Widget>>,
    ) -> io::Result<Uuid> {
        self.tab_hashmap.insert(
            tab_uuid,
            Tab {
                name: tab_name,
                uuid: tab_uuid,
                number: self.tab_hashmap.len(),
                main_layout: tab_layout,
                layout_list: tab_layout_list,
            },
        );
        Ok(tab_uuid)
    }
    //Удаление вкладки из списка вкладок
    pub fn remove_tab_from_list(&mut self, tab_uuid: Uuid) -> io::Result<Uuid> {
        match self.tab_hashmap.remove_entry(&tab_uuid) {
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
    pub fn current_tab(&self) -> Option<Uuid> {
        match self.current_tab_uuid {
            Some(curr_uuid) => Some(curr_uuid),
            None => self
                .tab_hashmap
                .keys()
                .next()
                .map_or(None, |new_curr_tab_uuid| Some(new_curr_tab_uuid.clone())),
        }
    }
    //Получение предыдудщей вкладки от активной по порядковому номеру, если такой не существует, возращается None
    pub fn previous_tab(&self) -> Option<Uuid> {
        let current_tab_uuid = match self.current_tab_uuid {
            Some(tab_uuid) => tab_uuid,
            None => return None,
        };

        let previous_tab_num = self
            .tab_hashmap
            .get(&current_tab_uuid)
            .map(|value| value.number.checked_sub(1).unwrap_or(0))?;

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

impl Widget for TabManager {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
    }
}
