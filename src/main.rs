use std::iter;

use wasm_bindgen::JsValue;
use web_sys::{
    console, window, CanvasRenderingContext2d, Event, HtmlCanvasElement, HtmlImageElement,
};
use yew::{
    callback::Callback, function_component, html, use_effect_with, use_memo, use_node_ref,
    use_state, Html, TargetCast,
};
use yew_hooks::use_interval;

const BIRD_SIZE: f64 = 120.;
const V: f64 = 10.;
const HISTORY_LEN: usize = 80;

macro_rules! clone_all {
    [$($s:ident), *] => {
        $(
            let $s = $s.clone();
        )*
    };
}

macro_rules! make_cb {
    ($core:expr) => {{
        let core = $core.clone();
        Callback::from(move |_| core())
    }};
}

#[function_component(App)]
fn app() -> Html {
    let canvas_ref = use_node_ref();
    let map_size = use_memo((), |_| {
        let window = window().unwrap();
        let width = window.inner_width().unwrap().as_f64().unwrap();
        let height = window.inner_height().unwrap().as_f64().unwrap();
        (width * 2., height * 2.)
    });
    let canvas_ctx = use_state(|| None);
    let bird_image = use_state(|| None);
    let angle = use_state(|| 0.);
    let pos = use_state(|| 0.);
    let is_flying = use_state(|| false);
    let history = use_state(Vec::<(f64, f64)>::new);

    let life = use_state(|| 10);
    let is_playing = use_state(|| false);

    let (w, h) = *map_size;
    {
        clone_all![canvas_ref, canvas_ctx];
        use_effect_with(canvas_ref, move |canvas_ref| {
            let canvas = canvas_ref
                .cast::<HtmlCanvasElement>()
                .expect("canvas_ref not attached");

            console::log_1(&JsValue::from_str("Set canvas size"));

            canvas.set_width(w as u32);
            canvas.set_height(h as u32);

            let ctx = CanvasRenderingContext2d::from(JsValue::from(
                canvas.get_context("2d").unwrap().unwrap(),
            ));

            canvas_ctx.set(Some(ctx));
        });
    }
    {
        clone_all![is_playing, pos, angle, history];
        use_effect_with(is_playing, move |is_playing| {
            if **is_playing {
                pos.set(0.);
                angle.set(0.);
                history.set(vec![]);
            }
        });
    }

    let img_onload = {
        let bird_image = bird_image.clone();
        Callback::from(move |event: Event| {
            let bird = event.target_dyn_into::<HtmlImageElement>().unwrap();
            bird_image.set(Some(bird));
        })
    };

    let start_fly_core = {
        clone_all![is_flying, is_playing];
        move || {
            is_flying.set(true);
            is_playing.set(true);
        }
    };

    let end_fly_core = {
        clone_all![is_flying];
        move || {
            is_flying.set(false);
        }
    };

    {
        clone_all![canvas_ctx, angle, bird_image, is_flying, pos, history, is_playing, life];
        use_interval(
            move || {
                if let Some(ctx) = canvas_ctx.as_ref() {
                    if let Some(bird) = bird_image.as_ref() {
                        ctx.set_fill_style(&JsValue::from_str("#e0e0e0"));
                        ctx.fill_rect(0., 0., w, h);
                        ctx.save();

                        ctx.translate(w / 2.0, h / 2.0 + *pos).unwrap();

                        ctx.set_stroke_style(&JsValue::from_str("white"));
                        ctx.begin_path();
                        let mut cnt = 0;
                        for (x, y) in history.iter() {
                            ctx.line_to(*x, *y);
                            cnt += 1;
                            if cnt % 3 == 0 {
                                ctx.stroke();
                                let new_color = 255 - cnt / 3;
                                ctx.set_stroke_style(&JsValue::from_str(&format!(
                                    "rgba({new_color}, {new_color}, {new_color})"
                                )));
                                ctx.begin_path();
                                ctx.move_to(*x, *y);
                            }
                        }
                        ctx.stroke();

                        ctx.rotate(*angle).unwrap();
                        ctx.draw_image_with_html_image_element_and_dw_and_dh(
                            bird,
                            -BIRD_SIZE / 2.,
                            -BIRD_SIZE / 2.,
                            BIRD_SIZE,
                            BIRD_SIZE,
                        )
                        .expect("draw bird failed");
                        ctx.restore();

                        if !*is_playing {
                            return;
                        }
                        if *pos > h / 2.0 || *pos < -h / 2.0 {
                            is_playing.set(false);
                            life.set(*life - 1);
                        }

                        pos.set(*pos + V * f64::sin(*angle));
                        angle.set(*angle + if *is_flying { -0.03 } else { 0.02 });
                        history.set(
                            iter::once((0., 0.))
                                .chain(history.iter().map(|(x, y)| {
                                    (x - V * f64::cos(*angle), y - V * f64::sin(*angle))
                                }))
                                .take(HISTORY_LEN)
                                .collect::<Vec<(f64, f64)>>(),
                        );
                    }
                }
            },
            40,
        );
    };

    html! {
        <>
            <canvas
                tabindex="0"
                ref={canvas_ref}
                //onmousedown={make_cb!(start_fly_core)}
                //onmouseup={make_cb!(end_fly_core)}
                onkeydown={make_cb!(start_fly_core)}
                onkeyup={make_cb!(end_fly_core)}
                onpointerdown={make_cb!(start_fly_core)}
                onpointerup={make_cb!(end_fly_core)}
            />
            <img id="birdImage" src="static/bird.webp" onload={img_onload} />
            <span id="lifeCnt"> {*life} </span>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
