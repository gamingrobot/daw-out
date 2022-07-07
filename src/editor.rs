use nih_plug::prelude::Editor;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::widgets::*;
use nih_plug_vizia::{assets, create_vizia_editor, ViziaState};
use std::sync::Arc;

use crate::DawOutParams;

const STYLE: &str = r#""#;

#[derive(Lens)]
struct Data {
    params: Arc<DawOutParams>,
}

impl Model for Data {}

// Makes sense to also define this here, makes it a bit easier to keep track of
pub(crate) fn default_state() -> Arc<ViziaState> {
    ViziaState::from_size(1000, 1000)
}

pub(crate) fn create(
    params: Arc<DawOutParams>,
    editor_state: Arc<ViziaState>,
) -> Option<Box<dyn Editor>> {
    create_vizia_editor(editor_state, move |cx, _| {
        cx.add_theme(STYLE);

        Data {
            params: params.clone(),
        }
        .build(cx);

        ResizeHandle::new(cx);

        VStack::new(cx, |cx| {
            Label::new(cx, "Server Address").bottom(Pixels(-1.0));
            Label::new(cx, "Server Port").bottom(Pixels(-1.0));
            Label::new(cx, "OSC Address Base").bottom(Pixels(-1.0));

            Label::new(cx, "param1").bottom(Pixels(-1.0));
            ParamSlider::new(cx, Data::params, |params| &params.param1);
            Label::new(cx, "param2").bottom(Pixels(-1.0));
            ParamSlider::new(cx, Data::params, |params| &params.param2);
            Label::new(cx, "param3").bottom(Pixels(-1.0));
            ParamSlider::new(cx, Data::params, |params| &params.param3);
            Label::new(cx, "param4").bottom(Pixels(-1.0));
            ParamSlider::new(cx, Data::params, |params| &params.param4);
            Label::new(cx, "param5").bottom(Pixels(-1.0));
            ParamSlider::new(cx, Data::params, |params| &params.param5);
            Label::new(cx, "param6").bottom(Pixels(-1.0));
            ParamSlider::new(cx, Data::params, |params| &params.param6);
            Label::new(cx, "param7").bottom(Pixels(-1.0));
            ParamSlider::new(cx, Data::params, |params| &params.param7);
            Label::new(cx, "param8").bottom(Pixels(-1.0));
            ParamSlider::new(cx, Data::params, |params| &params.param8);
        })
        .row_between(Pixels(0.0))
        .child_left(Stretch(1.0))
        .child_right(Stretch(1.0));
    })
}