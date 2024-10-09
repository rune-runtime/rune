use rust_embed::Embed;
use std::{
    collections::{HashMap, VecDeque},
    path::Path,
    time::{Duration, SystemTime, UNIX_EPOCH}
};

use color_eyre::{eyre::Result, owo_colors::OwoColorize};
use crossterm::event::{KeyCode, KeyEvent, MouseEvent};
use ratatui::{prelude::*, widgets::*};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::UnboundedSender;
use tui_widget_list::{ListBuilder, ListState, ListView};
use wit_parser::{
    Function, FunctionKind, Interface, InterfaceId, SourceMap, Type, TypeDef, TypeId,
    UnresolvedPackage, UnresolvedPackageGroup
};

use crate::{
    action::Action,
    components::Component,
    config::{Config, KeyBindings},
    tui::Event
};

#[derive(Embed)]
#[folder = "crates/rune/wit/runtime"]
struct RuneWit;

#[derive(Clone)]
pub enum CurrentItem {
    Package(UnresolvedPackage),
    Interface(InterfaceId),
    TypeDef(InterfaceId, TypeId),
    Function(Function),
}

impl CurrentItem {
    pub fn name(&self, package: &UnresolvedPackage) -> String {
        match self {
            CurrentItem::Package(p) => p.name.name.clone(),
            CurrentItem::Interface(id) => {
                let interface = package.interfaces.get(*id).unwrap();
                interface.name.clone().unwrap_or_default()
            }
            CurrentItem::TypeDef(interface_id, type_id) => {
                let type_def = package.types.get(*type_id).unwrap();
                format!("{}: {}", type_def.kind.as_str(), type_def.name.clone().unwrap_or_default())
            }
            CurrentItem::Function(func) => {
                let name = func.item_name();
                let params = func
                    .params
                    .iter()
                    .skip(1)
                    .map(|(param_name, param_type)| format!("{param_name}"))
                    .collect::<Vec<_>>().join(", ");

                let func_kind = match func.kind {
                    wit_parser::FunctionKind::Freestanding => "static",
                    wit_parser::FunctionKind::Method(type_id) => "method",
                    wit_parser::FunctionKind::Static(type_id) => "static",
                    wit_parser::FunctionKind::Constructor(type_id) => "constructor",
                };

                format!("{func_kind}: {name}({params})")
            }
        }
    }
}

fn list_items<'a>(
    package: &UnresolvedPackage,
    item: &CurrentItem
) -> Vec<(Line<'a>, Line<'a>, CurrentItem)> {
    match item {
        CurrentItem::Package(package) => package
            .interfaces
            .iter()
            .map(|(interface_id, _)| render_interface_list_item(package, &interface_id))
            .collect(),
        CurrentItem::Interface(interface_id) => {
            let interface = package.interfaces.get(*interface_id).unwrap();
            let mut functions = interface
                .functions
                .iter()
                .filter(|(_, f)| f.kind == FunctionKind::Freestanding)
                .map(|(_, f)| render_function_list_item(package, f))
                .collect::<Vec<_>>();

            let types = interface
                .types
                .iter()
                .map(|(_, type_id)| render_type_list_item(package, interface_id, type_id))
                .collect::<Vec<_>>();

            functions.extend(types);

            functions
        }
        CurrentItem::TypeDef(interface_id, type_id) => {
            let interface = package.interfaces.get(*interface_id).unwrap();
            interface
                .functions
                .iter()
                .filter(|(_, f)| match f.kind {
                    FunctionKind::Method(tid) => tid.eq(type_id),
                    FunctionKind::Static(tid) => tid.eq(type_id),
                    FunctionKind::Constructor(tid) => tid.eq(type_id),
                    _ => false,
                })
                .map(|(_, f)| render_function_list_item(package, f))
                .collect::<Vec<_>>()
        }
        CurrentItem::Function(_) => todo!(),
    }
}

fn render_interface_list_item<'a>(
    package: &UnresolvedPackage,
    interface_id: &InterfaceId
) -> (Line<'a>, Line<'a>, CurrentItem) {
    let interface = package.interfaces.get(*interface_id).unwrap();
    (
        Line::from(vec![
            Span::styled("namespace ", Style::reset().bold().fg(Color::Indexed(33))),
            Span::styled(
                interface.name.clone().unwrap_or_default(),
                Style::reset().bold().fg(Color::Indexed(75))
            ),
        ]),
        Line::styled(
            interface.docs.contents.clone().unwrap_or_default(),
            Style::reset().dim()
        ),
        CurrentItem::Interface(*interface_id),
    )
}

fn render_type_list_item<'a>(
    package: &UnresolvedPackage,
    interface_id: &InterfaceId,
    type_id: &TypeId
) -> (Line<'a>, Line<'a>, CurrentItem) {
    let interface = package.interfaces.get(*interface_id).unwrap();
    let type_def = package.types.get(*type_id).unwrap();
    (
        Line::from(vec![
            Span::styled("type ", Style::reset().bold().fg(Color::Indexed(43))),
            Span::styled(
                type_def.name.clone().unwrap_or_default(),
                Style::reset().bold().fg(Color::Indexed(158))
            ),
        ]),
        Line::styled(
            type_def.docs.contents.clone().unwrap_or_default(),
            Style::reset().dim()
        ),
        CurrentItem::TypeDef(*interface_id, *type_id),
    )
}

fn render_function_list_item<'a>(
    package: &UnresolvedPackage,
    func: &Function
) -> (Line<'a>, Line<'a>, CurrentItem) {
    let item = CurrentItem::Function(func.clone());
    match func.kind {
        wit_parser::FunctionKind::Freestanding => {
            let name = func.item_name();
            let params = func
                .params
                .iter()
                .skip(1)
                .map(|(param_name, param_type)| format!("{param_name}"))
                .collect::<Vec<_>>().join(", ");

            (
                Line::from(vec![
                    Span::styled("static fn ", Style::reset().bold().fg(Color::Indexed(33))),
                    Span::styled(
                        format!("{name}({params})"),
                        Style::reset().bold().fg(Color::Indexed(229))
                    ),
                ]),
                Line::styled(
                    func.docs.contents.clone().unwrap_or_default(),
                    Style::reset().dim()
                ),
                item,
            )
        }
        wit_parser::FunctionKind::Method(type_id) => {
            let name = func.item_name();
            let params = func
                .params
                .iter()
                .skip(1)
                .map(|(param_name, param_type)| format!("{param_name}"))
                .collect::<Vec<_>>().join(", ");

            (
                Line::from(vec![
                    Span::styled("fn ", Style::reset().bold().fg(Color::Indexed(33))),
                    Span::styled(
                        format!("{name}({params})"),
                        Style::reset().bold().fg(Color::Indexed(229))
                    ),
                ]),
                Line::styled(
                    func.docs.contents.clone().unwrap_or_default(),
                    Style::reset().dim(),
                ),
                item,
            )
        },
        wit_parser::FunctionKind::Static(type_id) => {
            let name = func.item_name();
            let params = func
                .params
                .iter()
                .skip(1)
                .map(|(param_name, param_type)| format!("{param_name}"))
                .collect::<Vec<_>>().join(", ");

            (
                Line::from(vec![
                    Span::styled("static fn ", Style::reset().bold().fg(Color::Indexed(33))),
                    Span::styled(
                        format!("{name}({params})"),
                        Style::reset().bold().fg(Color::Indexed(229))
                    ),
                ]),
                Line::styled(
                    func.docs.contents.clone().unwrap_or_default(),
                    Style::reset().dim()
                ),
                item,
            )
        },
        wit_parser::FunctionKind::Constructor(type_id) => {
            let name = func.item_name();
            let params = func
                .params
                .iter()
                .skip(1)
                .map(|(param_name, param_type)| format!("{param_name}"))
                .collect::<Vec<_>>().join(", ");

            (
                Line::from(vec![
                    Span::styled("constructor ", Style::reset().bold().fg(Color::Indexed(33))),
                    Span::styled(
                        format!("{name}({params})"),
                        Style::reset().bold().fg(Color::Indexed(229))
                    ),
                ]),
                Line::styled(
                    func.docs.contents.clone().unwrap_or_default(),
                    Style::reset().dim(),
                ),
                item,
            )
        },
    }
}

fn render_function_page(
    func: &Function,
    f: &mut Frame,
    area: Rect,
    function_params_list_state: &mut ListState,
) {
    let layout = Layout::vertical(vec![
        Constraint::Length(3),
        Constraint::Min(0),
        Constraint::Fill(1)
    ]);

    let [name_area, description_area, content_area] = layout.areas(area);

    let name = func.item_name();
    let params = func
        .params
        .iter()
        .skip(1)
        .map(|(param_name, param_type)| format!("{param_name}"))
        .collect::<Vec<_>>().join(", ");

    let name = Paragraph::new(match func.kind {
        wit_parser::FunctionKind::Freestanding | wit_parser::FunctionKind::Static(_) => {
            vec![Line::from(vec![
                Span::styled(
                    " static ",
                    Style::reset().bold().fg(Color::White).bg(Color::Indexed(33))
                ),
                Span::styled(
                    format!(" {} ", format!("{name}({params})")),
                    Style::reset().fg(Color::Black).bg(Color::Indexed(229))
                ),
            ])]
        },
        _ => {
            vec![Line::from(vec![
                Span::styled(
                    format!(" {} ", format!("{name}({params})")),
                    Style::reset().fg(Color::Indexed(229))
                ),
            ])]
        }

    })
        .block(Block::new().padding(Padding::proportional(1)));

    let description = Paragraph::new(func.docs.contents.clone().unwrap_or_default())
        .block(Block::new().padding(Padding::horizontal(2)));

    let params_list: Vec<_> = func.params.clone();
    let params_count = params_list.len();

    let params_list_builder = ListBuilder::new(move |context| {
        let (param_name, param_type) = params_list[context.index].clone();
        let item = Paragraph::new(vec![
            Line::from(vec![Span::from(param_name)])
        ])
        .block(Block::new()
            .borders(Borders::LEFT | Borders::BOTTOM)
            .border_set(if context.is_selected {
                symbols::border::Set {
                    top_left: symbols::line::THICK.vertical,
                    top_right: " ",
                    bottom_left: " ",
                    bottom_right: " ",
                    vertical_left: symbols::line::THICK.vertical,
                    vertical_right: " ",
                    horizontal_top: " ",
                    horizontal_bottom: " ",
                }
            } else {
                symbols::border::EMPTY
            })
            .border_style(if context.is_selected {
                Style::reset()
            } else {
                Style::default()
            })
        );
        
        (item, 3)
    });

    let params_list = ListView::new(params_list_builder, params_count)
        .block(Block::default().padding(Padding::vertical(1)));

    f.render_widget(
        name,
        name_area
    );

    f.render_widget(
        description,
        description_area
    );

    f.render_stateful_widget(params_list, content_area, function_params_list_state);

}

pub struct Docs {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
    visited: VecDeque<CurrentItem>,
    list_state: ListState,
    function_params_list_state: ListState
}

impl Docs {
    fn package(&self) -> &UnresolvedPackage {
        if let CurrentItem::Package(p) = self.visited.front().unwrap() {
            p
        } else {
            panic!("Not a package");
        }
    }
}

impl Default for Docs {
    fn default() -> Self {
        let mut source_map = SourceMap::new();
        for wit_name in RuneWit::iter() {
            let wit = RuneWit::get(&wit_name).unwrap();
            source_map.push(Path::new(wit_name.as_ref()), std::str::from_utf8(&wit.data).unwrap());
        }
        let package_group = source_map.parse().unwrap();

        let mut visited = VecDeque::new();
        visited.push_back(CurrentItem::Package(package_group.main));

        Self {
            command_tx: None,
            config: Config::default(),
            visited,
            list_state: ListState::default(),
            function_params_list_state: ListState::default()
        }
    }
}

impl Component for Docs {
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        self.command_tx = Some(tx);
        Ok(())
    }

    fn register_config_handler(&mut self, config: Config) -> Result<()> {
        self.config = config;
        Ok(())
    }

    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        match action {
            Action::Tick => {},
            _ => {}
        }
        Ok(None)
    }

    fn handle_events(&mut self, event: Option<Event>) -> Result<Option<Action>> {
        let r = match event {
            Some(Event::Key(key_event)) => self.handle_key_events(key_event)?,
            Some(Event::Mouse(mouse_event)) => self.handle_mouse_events(mouse_event)?,
            _ => None,
        };
        Ok(r)
    }

    fn handle_key_events(&mut self, key: KeyEvent) -> Result<Option<Action>> {
        match key {
            KeyEvent { code: KeyCode::Up, .. } => self.list_state.previous(),
            KeyEvent { code: KeyCode::Down, .. } => self.list_state.next(),
            KeyEvent { code: KeyCode::Enter, .. } => if let Some(selected) = self.list_state.selected {
                if let Some(back) = self.visited.back() {
                    let items: Vec<_> = list_items(self.package(), &self.visited.back().unwrap());
                    let (_, _, selected_item) = &items[selected];
                    self.visited.push_back(selected_item.clone());
                    self.list_state.select(None);
                }
            },
            KeyEvent { code: KeyCode::Esc, .. } => if self.visited.len() > 1 {
                self.visited.pop_back();
                self.list_state.select(None);
            },
            _ => {}
        }
        Ok(None)
    }

    fn handle_mouse_events(&mut self, mouse: MouseEvent) -> Result<Option<Action>> {
        Ok(None)
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
        let layout = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]);
        let [breadcrumb_area, content_area] = layout.areas(area);

        let breadcrumbs = self.visited.iter().map(|x| x.name(self.package())).collect::<Vec<_>>();
        let breadcrumb_depth = breadcrumbs.len() - 1;

        f.render_widget(
            Tabs::new(breadcrumbs)
                .block(Block::default().padding(Padding::horizontal(1)))
                .style(Style::reset().bold().dim())
                .highlight_style(Style::reset().bold())
                .select(breadcrumb_depth)
                .divider("→"),
            breadcrumb_area
        );

        match self.visited.back().unwrap() {
            CurrentItem::Function(func) => {
                let layout = Layout::vertical([Constraint::Length(3), Constraint::Fill(1)]);

                render_function_page(func, f, content_area, &mut self.function_params_list_state);
            },
            _ => {
                let items: Vec<_> = list_items(self.package(), &self.visited.back().unwrap());
                let item_count = items.len();
        
                let builder = ListBuilder::new(move |context| {
                    let (mut name, mut description, _) = items[context.index].clone();
                    name.spans.insert(0, Span::from(" "));
                    description.spans.insert(0, Span::from(" "));

                    let item = Paragraph::new(vec![
                        name,
                        description
                    ])
                    .block(Block::new()
                        .borders(Borders::LEFT | Borders::BOTTOM)
                        .border_set(if context.is_selected {
                            symbols::border::Set {
                                top_left: symbols::line::THICK.vertical,
                                top_right: " ",
                                bottom_left: " ",
                                bottom_right: " ",
                                vertical_left: symbols::line::THICK.vertical,
                                vertical_right: " ",
                                horizontal_top: " ",
                                horizontal_bottom: " ",
                            }
                        } else {
                            symbols::border::EMPTY
                        })
                        .border_style(if context.is_selected {
                            Style::reset()
                        } else {
                            Style::default()
                        })
                    );
                    
                    (item, 3)
                });
        
                let list = ListView::new(builder, item_count)
                    .block(Block::default().padding(Padding::vertical(1)));

                f.render_stateful_widget(
                    list,
                    content_area,
                    &mut self.list_state
                );
            }
        }

        Ok(())
    }
}
