use std::sync::Arc;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::widgets::*;

use crate::{editor::DawOutEditorEvent, editor::OscSettings, DawOutParams};

pub struct ParamView;

impl ParamView {
    pub fn new<P>(cx: &mut Context, params: P) -> Handle<Self>
    where
        P: Lens<Target = Arc<DawOutParams>> + Copy,
    {
        //TODO handle param names
        Self.build(cx, |cx| {
            HStack::new(cx, |cx| {
                Label::new(cx, "param1").class("label");
                ParamSlider::new(cx, params, |params| &params.param1)
                    .class("widget");
            })
            .class("row");
            HStack::new(cx, |cx| {
                Label::new(cx, "param2").class("label");
                ParamSlider::new(cx, params, |params| &params.param2)
                    .class("widget");
            })
            .class("row");
            HStack::new(cx, |cx| {
                Label::new(cx, "param3").class("label");
                ParamSlider::new(cx, params, |params| &params.param3)
                    .class("widget");
            })
            .class("row");
            HStack::new(cx, |cx| {
                Label::new(cx, "param4").class("label");
                ParamSlider::new(cx, params, |params| &params.param4)
                    .class("widget");
            })
            .class("row");
            HStack::new(cx, |cx| {
                Label::new(cx, "param5").class("label");
                ParamSlider::new(cx, params, |params| &params.param5)
                    .class("widget");
            })
            .class("row");
            HStack::new(cx, |cx| {
                Label::new(cx, "param6").class("label");
                ParamSlider::new(cx, params, |params| &params.param6)
                    .class("widget");
            })
            .class("row");
            HStack::new(cx, |cx| {
                Label::new(cx, "param7").class("label");
                ParamSlider::new(cx, params, |params| &params.param7)
                    .class("widget");
            })
            .class("row");
            HStack::new(cx, |cx| {
                Label::new(cx, "param8").class("label");
                ParamSlider::new(cx, params, |params| &params.param8)
                    .class("widget");
            })
            .class("row");
        })
    }
}

impl View for ParamView {
    fn element(&self) -> Option<&'static str> {
        Some("generic-ui")
    }
}


pub struct SettingsView;

impl SettingsView {
    pub fn new<S,P,L>(cx: &mut Context, settings: S, params: P, log: L) -> Handle<Self>
    where
        S: Lens<Target = OscSettings> + Copy,
        P: Lens<Target = Arc<DawOutParams>> + Copy,
        L: Lens<Target = Vec<String>>,
    {
        Self.build(cx, |cx| {
            HStack::new(cx, |cx| {
                Label::new(cx, "OSC Server Address").class("label");
                Textbox::new(cx, settings.map(|settings| settings.osc_server_address.clone()))
                    .on_edit(move |cx, text| {
                        //TODO: validate
                        cx.emit(DawOutEditorEvent::SetOscServerAddress(text));
                    })
                    .on_submit(|cx,  _, _| {
                        cx.emit(DawOutEditorEvent::ConnectionChange);
                    })
                    .width(Pixels(115.0)); //180 - 60 - 5
                Textbox::new(cx, settings.map(|settings| settings.osc_server_port))
                    .on_edit(move |cx, text| {
                        if let Ok(val) = text.parse::<u16>() {
                            cx.emit(DawOutEditorEvent::SetOscServerPort(val));
                            cx.toggle_class("invalid", false);
                        } else {
                            cx.toggle_class("invalid", true);
                        }
                    })
                    .on_submit(|cx,  _, _| {
                        cx.emit(DawOutEditorEvent::ConnectionChange);
                    })
                    .width(Pixels(60.0));
            })
            .class("row");
            // .col_between(Pixels(5.0));
            HStack::new(cx, |cx| {
                Label::new(cx, "OSC Address Base").class("label");
                Textbox::new(cx, settings.map(|settings| settings.osc_address_base.clone()))
                    .on_edit(move |cx, text| {
                        //TODO: validate
                        cx.emit(DawOutEditorEvent::SetOscAddressBase(text));
                    })
                    .on_submit(|cx,  _, _| {
                        cx.emit(DawOutEditorEvent::AddressBaseChange);
                    })
                    .width(Pixels(180.0));
            })
            .class("row");
            HStack::new(cx, |cx| {
                Label::new(cx, "Send MIDI").class("label");
                ParamSlider::new(cx, params, |params| &params.flag_send_midi)
                    .class("widget");
            })
            .class("row");
            HStack::new(cx, |cx| {
                Label::new(cx, "Send Audio").class("label");
                ParamSlider::new(cx, params, |params| &params.flag_send_audio)
                .class("widget");
            })
            .class("row");
            VirtualList::new(cx, log, 20.0, |cx, _index, item| {
                return Label::new(cx, item).left(Pixels(0.0)).class("label");
            })
            .height(Pixels(180.0))
            .class("row");
        })
    }
}

impl View for SettingsView {
    fn element(&self) -> Option<&'static str> {
        Some("generic-ui")
    }
}
