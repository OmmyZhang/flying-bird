use std::cmp::{max, min};
use std::iter;

use rand::{thread_rng, Rng};
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
const OB_WIDTH: f64 = 100.;
const HISTORY_LEN: usize = 120;
const MIN_SPACE: f64 = 3. * BIRD_SIZE;
const INTERV: u32 = 33;
const CHECK_RANGE: f64 = 2.5;
const V: f64 = 15.;
const ROTATE_UP: f64 = -0.05;
const ROTATE_DOWN_D: f64 = 1.;

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

struct Obstacle {
    x: f64,
    y1: f64,
    y2: f64,
}

impl Obstacle {
    pub fn random_gen(last: Option<&Obstacle>, w: f64, h: f64) -> Self {
        let mut rng = thread_rng();
        let x = w + rng.gen_range(0. ..5. * OB_WIDTH);
        let last_y1 = last.map(|ob| ob.y1).unwrap_or(h / 3.) as i32;
        let last_y2 = last.map(|ob| ob.y2).unwrap_or(h * 2. / 3.) as i32;
        let y1 = rng.gen_range(
            max(0, last_y1 - 5 * BIRD_SIZE as i32) as f64
                ..min(h as i32, last_y1 + 5 * BIRD_SIZE as i32) as f64,
        );
        let y2 = rng.gen_range(
            max(0, last_y2 - 5 * BIRD_SIZE as i32) as f64
                ..min(h as i32, last_y2 + 5 * BIRD_SIZE as i32) as f64,
        );

        if y2 - y1 < MIN_SPACE || y2 - y1 > 2.0 * MIN_SPACE {
            return Obstacle::random_gen(last, w, h);
        }

        Self { x, y1, y2 }
    }
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
    let obstacles = use_state(Vec::<Obstacle>::new);

    let life = use_state(|| 10);
    let is_playing = use_state(|| false);
    let distance = use_state(|| 0_f64);

    let (w, h) = *map_size;
    // 初始化canvas
    {
        clone_all![canvas_ref, canvas_ctx];
        use_effect_with(canvas_ref, move |canvas_ref| {
            let canvas = canvas_ref
                .cast::<HtmlCanvasElement>()
                .expect("canvas_ref not attached");

            console::log_1(&JsValue::from_str("Set canvas size"));

            canvas.set_width(w as u32);
            canvas.set_height(h as u32);
            canvas.focus().unwrap();

            let ctx = CanvasRenderingContext2d::from(JsValue::from(
                canvas.get_context("2d").unwrap().unwrap(),
            ));

            canvas_ctx.set(Some(ctx));
        });
    }
    // 新的一局各种初始化
    {
        clone_all![is_playing, pos, angle, history, obstacles, distance];
        use_effect_with(is_playing, move |is_playing| {
            if **is_playing {
                pos.set(0.);
                angle.set(0.);
                history.set(vec![]);
                obstacles.set(vec![]);
                distance.set(0.);
            }
        });
    }
    // 载入图片
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

    // 核心部分，每过一帧计算运动
    {
        clone_all![
            canvas_ctx, angle, bird_image, is_flying, pos, history, is_playing, life, distance
        ];
        use_interval(
            move || {
                if let Some(ctx) = canvas_ctx.as_ref() {
                    if let Some(bird) = bird_image.as_ref() {
                        ctx.set_fill_style(&JsValue::from_str("#e0e0e0"));
                        ctx.fill_rect(0., 0., w, h);
                        ctx.save();

                        let (ox, oy) = (w / 3., h / 2.);

                        ctx.translate(ox, oy + *pos).unwrap();

                        ctx.set_stroke_style(&JsValue::from_str("white"));
                        ctx.set_line_width(3.);
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

                        ctx.set_fill_style(&JsValue::from_str("#505050"));
                        for Obstacle { x, y1, y2 } in obstacles.iter() {
                            ctx.fill_rect(*x, 0., OB_WIDTH, *y1);
                            ctx.fill_rect(*x, *y2, OB_WIDTH, h - *y2);
                        }

                        if !*is_playing {
                            return;
                        }
                        let curr_obstacles = obstacles.iter().find(|ob| {
                            ob.x - BIRD_SIZE / CHECK_RANGE < ox
                                && ox < ob.x + OB_WIDTH + BIRD_SIZE / CHECK_RANGE
                        });
                        let (min_pos, max_pos) = if let Some(ob) = curr_obstacles {
                            (ob.y1 - oy, ob.y2 - oy)
                        } else {
                            (0. - oy, h - oy)
                        };
                        if *pos + BIRD_SIZE / CHECK_RANGE > max_pos
                            || *pos - BIRD_SIZE / CHECK_RANGE < min_pos
                        {
                            is_playing.set(false);
                            life.set(*life - 1);
                        }

                        let (sin, cos) = angle.sin_cos();
                        let (xl, yl) = (V * cos, V * sin);
                        pos.set(*pos + yl);
                        angle.set(if *is_flying {
                            *angle + ROTATE_UP
                        } else {
                            (yl + ROTATE_DOWN_D).atan2(xl)
                            //*angle - ROTATE_UP
                        });
                        history.set(
                            iter::once((0., 0.))
                                .chain(history.iter().map(|(x, y)| (x - xl, y - yl)))
                                .take(HISTORY_LEN)
                                .collect::<Vec<(f64, f64)>>(),
                        );
                        distance.set(*distance + xl);

                        let mut new_obstacles: Vec<Obstacle> = obstacles
                            .iter()
                            .map(|Obstacle { x, y1, y2 }| Obstacle {
                                x: *x - xl,
                                y1: *y1,
                                y2: *y2,
                            })
                            .filter(|Obstacle { x, y1: _, y2: _ }| *x > -OB_WIDTH)
                            .collect();

                        if obstacles
                            .last()
                            .map(|Obstacle { x, y1: _, y2: _ }| *x)
                            .unwrap_or_default()
                            < w - 2. * OB_WIDTH
                        {
                            new_obstacles.push(Obstacle::random_gen(obstacles.last(), w, h));
                        }
                        obstacles.set(new_obstacles);
                    }
                }
            },
            INTERV,
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
            <span id="score"> {format!("{:0>9}", (*distance * 2. / w) as u32)}</span>
            if !*is_playing {
                <div id="hint">
                    <p>{ "Tap the screen or press any key to fly" }</p>
                </div>
            }
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
