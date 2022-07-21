use std::sync::Arc;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::widgets::*;

use crate::DawOutParams;

pub struct ParamView;

impl ParamView {
    pub fn new<L>(cx: &mut Context, params: L) -> Handle<Self>
    where
        L: Lens<Target = Arc<DawOutParams>> + Copy,
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
