pub use crate::core::sync::Phone;
use crate::core::theme::Theme;
use crate::core::update::{SelfUpdateState, SelfUpdateStatus};
pub use crate::gui::views::about::Message as AboutMessage;
pub use crate::gui::views::list::{List as AppsView, LoadingState as ListLoadingState};
use crate::gui::{style, Message};
use iced::widget::{button, container, pick_list, row, text, tooltip, Space, Text};
use iced::{alignment, font, Alignment, Element, Font, Length, Renderer};

/// resources/assets/icons.ttf, loaded in [`crate::gui::UadGui`]
pub const ICONS: Font = Font {
    family: font::Family::Name("icomoon"),
    ..Font::DEFAULT
};

pub fn nav_menu<'a>(
    device_list: &'a Vec<Phone>,
    selected_device: Option<Phone>,
    apps_view: &AppsView,
    self_update_state: &SelfUpdateState,
) -> Element<'a, Message, Renderer<Theme>> {
    let apps_refresh_btn = button(
        Text::new("\u{E900}")
            .font(ICONS)
            .width(22)
            .horizontal_alignment(alignment::Horizontal::Center),
    )
    .on_press(Message::RefreshButtonPressed)
    .padding(5)
    .style(style::Button::Refresh);

    let apps_refresh_tooltip = tooltip(apps_refresh_btn, "Refresh apps", tooltip::Position::Bottom)
        .style(style::Container::Tooltip)
        .gap(4);

    let reboot_btn = button("Reboot")
        .on_press(Message::RebootButtonPressed)
        .padding(5)
        .style(style::Button::Refresh);

    #[allow(clippy::option_if_let_else)]
    let uad_version_text = if let Some(r) = &self_update_state.latest_release {
        if self_update_state.status == SelfUpdateStatus::Updating {
            Text::new("Updating please wait...")
        } else {
            Text::new(format!(
                "New UAD-ng version available {} -> {}",
                env!("CARGO_PKG_VERSION"),
                r.tag_name
            ))
        }
    } else {
        Text::new(env!("CARGO_PKG_VERSION"))
    };

    let update_btn = if self_update_state.latest_release.is_some() {
        button("Update")
            .on_press(Message::AboutAction(AboutMessage::DoSelfUpdate))
            .padding(5)
            .style(style::Button::SelfUpdate)
    } else {
        button("").height(0).width(0).style(style::Button::Hidden)
    };

    let apps_btn = button("Apps")
        .on_press(Message::AppsPress)
        .padding(5)
        .style(style::Button::Primary);

    let about_btn = button("About")
        .on_press(Message::AboutPressed)
        .padding(5)
        .style(style::Button::Primary);

    let settings_btn = button("Settings")
        .on_press(Message::SettingsPressed)
        .padding(5)
        .style(style::Button::Primary);

    let device_list_text = match apps_view.loading_state {
        ListLoadingState::FindingPhones => text("finding connected phone..."),
        _ => text("no devices/emulators found"),
    };

    let row = match selected_device {
        Some(phone) => row![
            apps_refresh_tooltip,
            reboot_btn,
            pick_list(device_list, Some(phone), Message::DeviceSelected,),
            Space::new(Length::Fill, Length::Shrink),
            uad_version_text,
            update_btn,
            apps_btn,
            about_btn,
            settings_btn,
        ]
        .width(Length::Fill)
        .align_items(Alignment::Center)
        .spacing(10),
        None => row![
            reboot_btn,
            apps_refresh_tooltip,
            device_list_text,
            Space::new(Length::Fill, Length::Shrink),
            uad_version_text,
            update_btn,
            apps_btn,
            about_btn,
            settings_btn,
        ]
        .width(Length::Fill)
        .align_items(Alignment::Center)
        .spacing(10),
    };

    container(row)
        .width(Length::Fill)
        .padding(10)
        .style(style::Container::Frame)
        .into()
}
