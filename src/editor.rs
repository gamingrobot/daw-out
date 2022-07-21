
use nih_plug::prelude::Editor;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::widgets::*;
use nih_plug_vizia::{assets, create_vizia_editor, ViziaState};
use std::sync::Arc;

use crate::param_view::ParamView;
use crate::DawOutParams;

/// VIZIA uses points instead of pixels for text
const POINT_SCALE: f32 = 0.75;

#[derive(Lens)]
struct DawOutEditor {
    params: Arc<DawOutParams>,
}

impl Model for DawOutEditor {}

// Makes sense to also define this here, makes it a bit easier to keep track of
pub(crate) fn default_state() -> Arc<ViziaState> {
    ViziaState::from_size(700, 400)
}

pub(crate) fn create(
    params: Arc<DawOutParams>,
    editor_state: Arc<ViziaState>,
) -> Option<Box<dyn Editor>> {
    create_vizia_editor(editor_state, move |cx, _| {
        //cx.add_theme(STYLE);

        DawOutEditor {
            params: params.clone(),
        }
        .build(cx);

        //ResizeHandle::new(cx);

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
                        Label::new(cx, "Send MIDI").class("label");
                        ParamSlider::new(cx, DawOutEditor::params, |params| &params.flag_send_midi)
                            .class("widget");
                    })
                    .class("row")
                    .col_between(Pixels(5.0));
                    HStack::new(cx, |cx| {
                        Label::new(cx, "Send Audio").class("label");
                        ParamSlider::new(cx, DawOutEditor::params, |params| &params.flag_send_audio)
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
