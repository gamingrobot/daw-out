use crossbeam_channel::Sender;
use nih_plug::debug::*;
use nih_plug::prelude::Editor;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::ViziaTheming;
use nih_plug_vizia::{assets, create_vizia_editor, ViziaState};
use std::sync::Arc;

use crate::subviews::{ParamView, SettingsView};
use crate::{DawOutParams, OscAddressBaseType, OscChannelMessageType, OscConnectionType};

/// VIZIA uses points instead of pixels for text
const POINT_SCALE: f32 = 0.75;

#[derive(Lens)]
struct DawOutEditor {
    sender: Arc<Sender<OscChannelMessageType>>,
    params: Arc<DawOutParams>,
    settings: OscSettings,
}

pub struct OscSettings {
    pub osc_server_address: String,
    pub osc_server_port: u16,
    pub osc_address_base: String,
}

pub enum DawOutEditorEvent {
    SetOscServerAddress(String),
    SetOscServerPort(u16),
    SetOscAddressBase(String),
    ConnectionChange,
    AddressBaseChange,
}

impl Model for DawOutEditor {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            DawOutEditorEvent::SetOscServerAddress(ip) => {
                nih_trace!("Edit Event {}", ip);
                self.settings.osc_server_address = ip.clone();
                *self.params.osc_server_address.write() = self.settings.osc_server_address.clone();
            }
            DawOutEditorEvent::SetOscServerPort(port) => {
                nih_trace!("Edit Event {}", port);
                self.settings.osc_server_port = port.clone();
                *self.params.osc_server_port.write() = self.settings.osc_server_port.clone();
            }
            DawOutEditorEvent::SetOscAddressBase(address) => {
                nih_trace!("Edit Event {}", address);
                self.settings.osc_address_base = address.clone();
                *self.params.osc_address_base.write() = self.settings.osc_address_base.clone();
            }
            DawOutEditorEvent::ConnectionChange => {
                nih_trace!(
                    "Connection Changed {}:{}",
                    self.settings.osc_server_address,
                    self.settings.osc_server_port
                );
                let send_result =
                    self.sender
                    .send(OscChannelMessageType::ConnectionChange(OscConnectionType {
                        ip: self.settings.osc_server_address.clone(),
                        port: self.settings.osc_server_port,
                    }));
                if send_result.is_err() {
                    nih_error!("Failed to send ConnectionChange update {:?}", send_result.unwrap_err());
                }
            }
            DawOutEditorEvent::AddressBaseChange => {
                nih_trace!("AddressBase Changed: {}", self.settings.osc_address_base);
                let send_result = self.sender.send(OscChannelMessageType::AddressBaseChange(
                    OscAddressBaseType {
                        address: self.settings.osc_address_base.clone(),
                    },
                ));
                if send_result.is_err() {
                    nih_error!("Failed to send AddressBaseChange update {:?}", send_result.unwrap_err());
                }
            }
        });
    }
}

// Makes sense to also define this here, makes it a bit easier to keep track of
pub(crate) fn default_state() -> Arc<ViziaState> {
    ViziaState::new(|| (700, 400))
}

pub(crate) fn create(
    params: Arc<DawOutParams>,
    sender: Arc<Sender<OscChannelMessageType>>,
    editor_state: Arc<ViziaState>,
) -> Option<Box<dyn Editor>> {
    create_vizia_editor(editor_state, ViziaTheming::Custom, move |cx, _| {
        assets::register_noto_sans_light(cx);
        assets::register_noto_sans_thin(cx);

        DawOutEditor {
            sender: sender.clone(),
            params: params.clone(),
            settings: OscSettings {
                osc_server_address: params.osc_server_address.read().to_string(),
                osc_server_port: *params.osc_server_port.read(),
                osc_address_base: params.osc_address_base.read().to_string()
            }.into()
        }
        .build(cx);

        //Uncomment for debugging styles
        //cx.add_stylesheet(include_style!("src/style.css")).expect("Failed to load stylesheet");

        //ResizeHandle::new(cx);

        VStack::new(cx, |cx| {
            Label::new(cx, "DAW Out")
                .font_size(40.0 * POINT_SCALE)
                .class("title");
            HStack::new(cx, |cx| {
                SettingsView::new(cx, DawOutEditor::settings, DawOutEditor::params);
                ParamView::new(cx, DawOutEditor::params);
                //TODO Errors View
            });
        });
    })
}
