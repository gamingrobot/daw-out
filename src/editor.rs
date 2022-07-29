use crossbeam_channel::Sender;
use nih_plug::debug::*;
use nih_plug::prelude::Editor;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::widgets::*;
use nih_plug_vizia::{assets, create_vizia_editor, ViziaState};
use std::sync::Arc;

use crate::param_view::ParamView;
use crate::{DawOutParams, OscChannelMessageType, OscConnectionType, OscAddressBaseType};

/// VIZIA uses points instead of pixels for text
const POINT_SCALE: f32 = 0.75;

#[derive(Lens)]
struct DawOutEditor {
    osc_server_address: String,
    osc_server_port: u16,
    osc_address_base: String,
    sender: Arc<Sender<OscChannelMessageType>>,
    params: Arc<DawOutParams>,
}

pub enum DawOutEditorEvent {
    SetOscServerAddress(String),
    SetOscServerPort(u16),
    SetOscAddressBase(String),
    ConnectionChange,
    AddressBaseChange,
}

impl Model for DawOutEditor {
    fn event(&mut self, _cx: &mut Context, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            DawOutEditorEvent::SetOscServerAddress(text) => {
                nih_trace!("Edit Event {}", text);
                self.osc_server_address = text.clone();
                *self.params.osc_server_address.write() = self.osc_server_address.clone();
            }
            DawOutEditorEvent::SetOscServerPort(port) => {
                nih_trace!("Edit Event {}", port);
                self.osc_server_port = port.clone();
                *self.params.osc_server_port.write() = self.osc_server_port.clone();
            }
            DawOutEditorEvent::SetOscAddressBase(text) => {
                nih_trace!("Edit Event {}", text);
                self.osc_address_base = text.clone();
                *self.params.osc_address_base.write() = self.osc_address_base.clone();
            }
            DawOutEditorEvent::ConnectionChange => {
                nih_trace!("Connection Changed {}:{}", self.osc_server_address, self.osc_server_port);
                self.sender.send(OscChannelMessageType::ConnectionChange(OscConnectionType {
                        ip: self.osc_server_address.clone(),
                        port: self.osc_server_port,
                    }))
                    .unwrap();
            }
            DawOutEditorEvent::AddressBaseChange => {
                nih_trace!("AddressBase Changed: {}", self.osc_address_base);
                self.sender.send(OscChannelMessageType::AddressBaseChange(OscAddressBaseType {
                    address: self.osc_address_base.clone()
                }))
                .unwrap();
            }
        });
    }
}

// Makes sense to also define this here, makes it a bit easier to keep track of
pub(crate) fn default_state() -> Arc<ViziaState> {
    ViziaState::from_size(700, 400)
}

pub(crate) fn create(
    params: Arc<DawOutParams>,
    sender: Arc<Sender<OscChannelMessageType>>,
    editor_state: Arc<ViziaState>,
) -> Option<Box<dyn Editor>> {
    create_vizia_editor(editor_state, move |cx, _| {
        //cx.add_theme(STYLE);

        DawOutEditor {
            osc_server_address: params.osc_server_address.read().to_string(),
            osc_server_port: *params.osc_server_port.read(),
            osc_address_base: params.osc_address_base.read().to_string(),
            sender: sender.clone(),
            params: params.clone(),
        }
        .build(cx);

        //ResizeHandle::new(cx);

        //TODO: cleanup styling, split settings into another view?
        VStack::new(cx, |cx| {
            Label::new(cx, "DAW Out")
                .font(assets::NOTO_SANS_THIN)
                .font_size(40.0 * POINT_SCALE)
                .height(Pixels(50.0))
                .child_top(Stretch(1.0))
                .child_bottom(Pixels(10.0));
            HStack::new(cx, |cx| {
                VStack::new(cx, |cx| {
                    HStack::new(cx, |cx| {
                        Label::new(cx, "OSC Server Address").class("label");
                        Textbox::new(cx, DawOutEditor::osc_server_address)
                            .on_edit(move |cx, text| {
                                //TODO: validate
                                cx.emit(DawOutEditorEvent::SetOscServerAddress(text));
                            })
                            .on_leave(|cx| {
                                cx.emit(DawOutEditorEvent::ConnectionChange);
                            })
                            .width(Pixels(115.0)); //180 - 60 - 5
                        Textbox::new(cx, DawOutEditor::osc_server_port)
                            .on_edit(move |cx, text| {
                                if let Ok(val) = text.parse::<u16>() {
                                    cx.emit(DawOutEditorEvent::SetOscServerPort(val));
                                    cx.toggle_class("invalid", false);
                                } else {
                                    cx.toggle_class("invalid", true);
                                }
                            })
                            .on_leave(|cx| {
                                cx.emit(DawOutEditorEvent::ConnectionChange);
                            })
                            .width(Pixels(60.0));
                    })
                    .class("row")
                    .col_between(Pixels(5.0));
                    HStack::new(cx, |cx| {
                        Label::new(cx, "OSC Address Base").class("label");
                        Textbox::new(cx, DawOutEditor::osc_address_base)
                            .on_edit(move |cx, text| {
                                //TODO: validate
                                cx.emit(DawOutEditorEvent::SetOscAddressBase(text));
                            })
                            .on_leave(|cx| {
                                cx.emit(DawOutEditorEvent::AddressBaseChange);
                            })
                            .width(Pixels(180.0));
                    })
                    .class("row")
                    .col_between(Pixels(5.0));
                    HStack::new(cx, |cx| {
                        Label::new(cx, "Send MIDI").class("label");
                        ParamSlider::new(cx, DawOutEditor::params, |params| &params.flag_send_midi)
                            .class("widget");
                    })
                    .class("row")
                    .col_between(Pixels(5.0));
                    HStack::new(cx, |cx| {
                        Label::new(cx, "Send Audio").class("label");
                        ParamSlider::new(cx, DawOutEditor::params, |params| {
                            &params.flag_send_audio
                        })
                        .class("widget");
                    })
                    .class("row")
                    .col_between(Pixels(5.0));
                })
                .top(Pixels(10.0)) //make the colums align TODO move these to their own view?
                .width(Auto)
                .height(Auto)
                .row_between(Pixels(5.0))
                .child_left(Stretch(1.0));
                ParamView::new(cx, DawOutEditor::params)
                    .width(Auto)
                    .height(Auto);
            });
        })
        .width(Percentage(100.0))
        .row_between(Pixels(0.0))
        .child_left(Stretch(1.0))
        .child_right(Stretch(1.0));
    })
}
