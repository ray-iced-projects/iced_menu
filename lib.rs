
mod widgets;
use std::collections::HashMap;

use widgets::popover::{Popover, Position};

#[derive(Debug, Clone)]
pub struct Menu {
    pub id: usize,
    pub bar_labels: Vec<String>,
    pub bar_widths: Option<Vec<f32>>,
    pub padding: Option<Vec<f32>>,
    pub spacing: Option<f32>,
    pub position: Position,
    pub height: Option<f32>,
    pub close_on_bar_item_click: Option<bool>,
    pub close_on_bar_background_click: Option<bool>,
    pub items_close_on_click_global: Option<bool>,
    pub items_close_on_background_click_global: Option<bool>,
    pub show: bool,
    pub is_open: Vec<bool>,
    pub dropdown_is_open: Vec<HashMap<String, bool>>,
}

impl Menu {

    pub fn construct<'a>(
        &'a self,
        grouped_content: Vec<(usize, Vec<GroupedItem<'a>>)>,
        widgets: &'a HashMap<usize, Widgets>,
        containers: &HashMap<usize, Containers>,
    ) -> Option<Element<'a, app::Message, Theme, Renderer>> {
        let mut bar_columns = vec![];

        let mut index = 0;

        for (menu_bar_item_id, group) in grouped_content.into_iter() {

            let dropdown_is_open = self.dropdown_is_open.get(index).cloned().unwrap_or_default();
            let dropdowns = build_dropdown_items(
                self,
                widgets,
                group,
                containers,
                menu_bar_item_id,
                index,
                &dropdown_is_open,
                "",
            );
            bar_columns.push(
                container(column(dropdowns).boxed())
                    // .width(bar_widths[index])
                    .style(move |theme| container::bordered_box(theme))
                    .boxed(),
            );
            index += 1;
        }

        // Constructs the menu bar
        let bar_widths = match self.bar_widths.clone() {
            Some(widths) => {
                if widths.len() == 1 {
                    vec![widths[0]; self.bar_labels.len()]
                } else {
                    widths
                }
            }
            None => vec![],
        };

        let bar_text_labels = configure_bar_labels(
            self,
            &self.bar_labels,
            self.bar_labels_text_style_id,
            self.bar_labels_text_font_id,
            widgets,
        );

        let id = self.id;
        let rw = row(bar_text_labels
            .into_iter()
            .zip(bar_columns.into_iter())
            .zip(bar_widths.iter())
            .enumerate()
            .map(|(idx, ((label_element, popover_element), width))| {
                let pop = Popover::new(
                    button(label_element)
                        .on_press(Message::Menu(id, MenuMessage::OpenPopover(idx)))
                        .width(*width)
                        .style(move |theme: &Theme, status| 
                            get_popover_btn_style(self, widgets, theme, status, true)
                        )
                        .boxed(),
                    // menu bar popover
                    self.is_open[idx].then_some(popover_element),
                )
                .position(get_menu_position(self, true))
                .on_close(Message::Menu(id, MenuMessage::OnClose));
                mouse_area(pop)
                    .on_enter(Message::Menu(id, MenuMessage::OnBarEnter(idx)))
                    .boxed()
            }));

        // Places a container around the menu bar for styling
        let cont_style_opt = self
            .lookup_widgets(widgets, self.bar_container_style_id)
            .and_then(Widgets::as_container_style)
            .cloned();

        let cont_rw = container(rw.boxed())
            .style(move |theme| {
                if let Some(st) = &cont_style_opt {
                    st.to_iced(theme, &self.bar_container_style_std)
                } else {
                    match &self.bar_container_style_std {
                        Some(std) => std.to_iced(theme),
                        None => container::bordered_box(theme),
                    }
                }
            })
            .padding(get_padding(&self.padding))
            .boxed();

        Some(cont_rw)
    }
}

/// Recursively build popover elements for all Sub items at every depth.
fn build_dropdown_items<'a>(
    menu: &Menu,
    widgets: &'a HashMap<usize, Widgets>,
    items: Vec<GroupedItem<'a>>,
    containers: &HashMap<usize, Containers>,
    menu_bar_item_id: usize,
    bar_idx: usize,
    sub_is_open: &HashMap<String, bool>,
    parent_path: &str,
) -> Vec<Element<'a, app::Message>> {
    items
        .into_iter()
        .map(|item| match item {
            GroupedItem::Plain(el) => el.boxed(),
            GroupedItem::Sub {
                children, sub_item_id, ..
            } => {
                let label = containers
                    .get(&sub_item_id)
                    .and_then(Containers::as_menu_sub_item)
                    .map(|sm| sm.label.clone())
                    .unwrap_or_else(|| "Sub".to_string());

                let path = if parent_path.is_empty() {
                    label.clone()
                } else {
                    format!("{parent_path}/{label}")
                };

                let menu_bar_item = menu
                    .lookup(containers, Some(menu_bar_item_id))
                    .and_then(Containers::as_menu_bar_item);

                let is_open_now = sub_is_open.get(&path).copied().unwrap_or(false);
                // inner sub-items use plain text style and open to the right
                let sub_items = build_dropdown_items(
                    menu,
                    widgets,
                    children,
                    containers,
                    menu_bar_item_id,
                    bar_idx,
                    sub_is_open,
                    &path,
                );
                let sub_dropdown = container(column(sub_items).boxed())
                    .style(move |theme| container::bordered_box(theme))
                    .boxed();

                let style_opt = menu_bar_item
                    .and_then(|mbi| mbi.style_id)
                    .and_then(|id| menu.lookup_widgets(widgets, Some(id)))
                    .and_then(Widgets::as_button_style)
                    .cloned()
                    .or_else(|| {
                        menu.lookup_widgets(widgets, menu.bar_pop_btn_style_id)
                            .and_then(Widgets::as_button_style)
                            .cloned()
                    });

                let style_std_opt = menu_bar_item
                    .and_then(|mbi| mbi.style_std.clone())
                    .or_else(|| {
                        None
                    });

                let pal_opt = menu_bar_item
                    .and_then(|mbi| mbi.palette_id)
                    .and_then(|id| menu.lookup_widgets(widgets, Some(id)))
                    .and_then(Widgets::as_palette)
                    .cloned()
                    .or_else(|| {
                        menu.lookup_widgets(widgets, menu.bar_pop_btn_palette_id)
                            .and_then(Widgets::as_palette)
                            .cloned()
                    });

                let font_opt = menu_bar_item
                    .and_then(|mbi| mbi.font_id)
                    .and_then(|id| menu.lookup_widgets(widgets, Some(id)))
                    .and_then(Widgets::as_font)
                    .cloned()
                    .or_else(|| {
                        menu.lookup_widgets(widgets, menu.bar_pop_btn_font_id)
                            .and_then(Widgets::as_font)
                            .cloned()
                    });

                let position_opt = menu_bar_item
                    .and_then(|mbi| Some(mbi.position))
                    .or_else(|| {
                        None
                    });

                let position = position_opt.unwrap_or(Position::Right);
                
                // Build the menu bar button dropdowns to all
                Popover::new(
                    button(text(label))
                        .on_press(Message::Menu(
                            menu.id,
                            MenuMessage::OpenPopoverSub(bar_idx, path.clone()),
                        ))
                        .style(move |theme: &Theme, status| {
                            if style_opt.is_some() || pal_opt.is_some() {
                                let btn_st = ButtonStyle::default();
                                let st = style_opt.as_ref().unwrap_or(&btn_st);
                                st.to_iced(theme, status, &pal_opt, &style_std_opt)
                            } else {
                                match &style_std_opt {
                                    Some(std) => std.to_iced(theme, status),
                                    None => button::text(theme, status),
                                }
                            }
                        })
                        .boxed(),
                    is_open_now.then_some(sub_dropdown),
                )
                .position(position)
                .on_close(Message::Menu(menu.id, MenuMessage::CloseSubPopover(bar_idx, path)))
                .boxed()
            }
        })
        .collect()
}

fn configure_bar_labels<'a>(
    menu: &Menu,
    labels: &'a Vec<String>,
    style_id: Option<usize>,
    font_id: Option<usize>,
    widgets: &'a HashMap<usize, Widgets>,
) -> Vec<Element<'a, app::Message>> {
    let style_opt = menu.lookup_widgets(widgets, style_id).and_then(Widgets::as_text_style);

    let mut text_labels = vec![];

    for label in labels.iter() {
        let txt = match style_opt {
            Some(style) => style
                .construct(label.clone(), font_id, widgets, Some(MenuStyleOverrides::default()))
                .boxed(),
            None => text(label)
                .align_x(alignment::Horizontal::Center)
                .align_y(alignment::Vertical::Center)
                .width(Length::Fill)
                .boxed(),
        };
        text_labels.push(txt)
    }

    text_labels
}
