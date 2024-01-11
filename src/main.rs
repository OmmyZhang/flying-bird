use std::iter;

use gloo_utils::format::JsValueSerdeExt;
use rand::{thread_rng, Rng};
use serde::Serialize;
use wasm_bindgen::JsValue;
use web_sys::{
    console, window, CanvasRenderingContext2d, Event, HtmlCanvasElement, HtmlImageElement,
};
use yew::{
    callback::Callback, function_component, html, use_effect_with, use_memo, use_node_ref,
    use_state, Html, TargetCast,
};
use yew_hooks::use_interval;

const BG_COLOR: u8 = 240;
const OB_COLOR: u8 = 100;
const BIRD_SIZE: f64 = 120.;
const CHECK_SIZE: f64 = BIRD_SIZE / 2.;
const OB_WIDTH: f64 = 100.;
const HISTORY_LEN: usize = 120;
const HISTORY_COLOR_CHANGE: usize = 7;
const MIN_SPACE: f64 = 3. * BIRD_SIZE;
const INTERV: u32 = 17;
const V: f64 = 10.;
const ROTATE_UP: f64 = -0.05;
const ROTATE_DOWN_D: f64 = 0.35;

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

#[derive(Serialize)]
#[serde(rename_all(serialize = "camelCase"))]
struct ContextOtions {
    pub will_read_frequently: bool,
}

struct Obstacle {
    x: f64,
    y1: f64,
    y2: f64,
}

impl Obstacle {
    pub fn random_gen(last: Option<&Obstacle>, w: f64, h: f64) -> Self {
        let mut rng = thread_rng();
        let x = w + rng.gen_range(0.0..5.0 * OB_WIDTH);
        let last_y1 = last.map(|ob| ob.y1).unwrap_or(h / 3.0);

        let space = rng.gen_range(MIN_SPACE..2.0 * MIN_SPACE);
        let y1 = rng.gen_range(
            (last_y1 - 5.0 * BIRD_SIZE).max(0.0)..(last_y1 + 5.0 * BIRD_SIZE).min(h - space),
        );

        Self {
            x,
            y1,
            y2: y1 + space,
        }
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
                canvas
                    .get_context_with_context_options(
                        "2d",
                        &JsValue::from_serde(&ContextOtions {
                            will_read_frequently: true,
                        })
                        .unwrap(),
                    )
                    .unwrap()
                    .unwrap(),
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
                        if !*is_playing && distance.abs() > 1e-10 {
                            return;
                        }

                        ctx.set_fill_style(&JsValue::from_str(&format!(
                            "rgb({BG_COLOR}, {BG_COLOR}, {BG_COLOR})"
                        )));
                        ctx.fill_rect(0., 0., w, h);
                        ctx.save();

                        // 画轨迹 && 画鸟
                        let (ox, oy) = (w / 3., h / 2.);
                        let pos_y = oy + *pos;
                        ctx.translate(ox, pos_y).unwrap();

                        ctx.set_stroke_style(&JsValue::from_str("white"));
                        ctx.set_line_width(3.);
                        ctx.begin_path();
                        let mut cnt = 0;
                        for (x, y) in history.iter() {
                            ctx.line_to(*x, *y);
                            cnt += 1;
                            if cnt % HISTORY_COLOR_CHANGE == 0 {
                                ctx.stroke();
                                let new_color = 255 - cnt / HISTORY_COLOR_CHANGE;
                                ctx.set_stroke_style(&JsValue::from_str(&format!(
                                    "rgb({new_color}, {new_color}, {new_color})"
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

                        // 画障碍物
                        ctx.set_fill_style(&JsValue::from_str(&format!(
                            "rgb({OB_COLOR}, {OB_COLOR}, {OB_COLOR})"
                        )));
                        for Obstacle { x, y1, y2 } in obstacles.iter() {
                            ctx.fill_rect(*x, 0., OB_WIDTH, *y1);
                            ctx.fill_rect(*x, *y2, OB_WIDTH, h - *y2);
                        }

                        if !*is_playing {
                            return;
                        }

                        // 检查是否撞击
                        // 初筛
                        let curr_obstacles = obstacles
                            .iter()
                            .find(|ob| ob.x - CHECK_SIZE < ox && ox < ob.x + OB_WIDTH + CHECK_SIZE);
                        let (min_pos, max_pos) = if let Some(ob) = curr_obstacles {
                            (ob.y1, ob.y2)
                        } else {
                            (0.0, h)
                        };

                        let down_collision = pos_y + CHECK_SIZE > max_pos
                            && (
                                // 细筛
                                curr_obstacles
                                    .map(|ob| {
                                        // 读出来的rgb会有变化，先不用处理，
                                        // colorSpace: "display-p3" 浏览器还没都支持
                                        // https://stackoverflow.com/questions/70480792/getimagedata-does-not-return-correct-rgb-values-display-p3-image
                                        let start_y = (pos_y - CHECK_SIZE).max(ob.y2);
                                        ctx.get_image_data(
                                            ob.x,
                                            start_y,
                                            -1.0,
                                            (h - start_y).min(2.0 * CHECK_SIZE),
                                        )
                                        .unwrap()
                                        .data()
                                        .0
                                        .iter()
                                        .any(|v| *v < 50)
                                            || ctx
                                                .get_image_data(
                                                    ob.x + OB_WIDTH,
                                                    start_y,
                                                    1.0,
                                                    (h - start_y).min(2.0 * CHECK_SIZE),
                                                )
                                                .unwrap()
                                                .data()
                                                .0
                                                .iter()
                                                .any(|v| *v < 50)
                                            || ctx
                                                .get_image_data(ob.x, ob.y2, OB_WIDTH, -1.)
                                                .unwrap()
                                                .data()
                                                .0
                                                .iter()
                                                .any(|v| *v < 50)
                                    })
                                    .unwrap_or(false)
                                    || ctx
                                        .get_image_data(ox - CHECK_SIZE, h, 2.0 * CHECK_SIZE, -1.0)
                                        .unwrap()
                                        .data()
                                        .0
                                        .iter()
                                        .any(|v| *v < 50)
                            );
                        let up_collision = pos_y - CHECK_SIZE < min_pos
                            && (curr_obstacles
                                .map(|ob| {
                                    let start_y = (pos_y - CHECK_SIZE).max(0.0);
                                    ctx.get_image_data(
                                        ob.x,
                                        start_y,
                                        -1.0,
                                        (ob.y1 - start_y).min(2.0 * CHECK_SIZE),
                                    )
                                    .unwrap()
                                    .data()
                                    .0
                                    .iter()
                                    .any(|v| *v < 50)
                                        || ctx
                                            .get_image_data(
                                                ob.x + OB_WIDTH,
                                                start_y,
                                                1.0,
                                                (ob.y1 - start_y).min(2.0 * CHECK_SIZE),
                                            )
                                            .unwrap()
                                            .data()
                                            .0
                                            .iter()
                                            .any(|v| *v < 50)
                                        || ctx
                                            .get_image_data(ob.x, ob.y1, OB_WIDTH, 1.0)
                                            .unwrap()
                                            .data()
                                            .0
                                            .iter()
                                            .any(|v| *v < 50)
                                })
                                .unwrap_or(false)
                                || ctx
                                    .get_image_data(ox - CHECK_SIZE, 0.0, 2.0 * CHECK_SIZE, 1.0)
                                    .unwrap()
                                    .data()
                                    .0
                                    .iter()
                                    .any(|v| *v < 50));

                        if down_collision || up_collision {
                            is_playing.set(false);
                            life.set(*life - 1);
                            /*
                            if let Some(ob) = curr_obstacles {
                                ctx.set_fill_style(&JsValue::from_str("red"));
                                if pos_y + CHECK_SIZE > max_pos {
                                    let start_y = (pos_y - CHECK_SIZE).max(ob.y2);
                                    ctx.fill_rect(
                                        ob.x,
                                        start_y,
                                        -1.0,
                                        (h - start_y).min(2.0 * CHECK_SIZE),
                                    );
                                    ctx.fill_rect(ob.x, ob.y2, OB_WIDTH, -1.0);
                                }

                                if pos_y - CHECK_SIZE < min_pos {
                                    let start_y = (pos_y - CHECK_SIZE).max(0.0);
                                    ctx.fill_rect(
                                        ob.x,
                                        start_y,
                                        -1.0,
                                        (ob.y1 - start_y).min(2.0 * CHECK_SIZE),
                                    );
                                    ctx.fill_rect(ob.x, ob.y1, OB_WIDTH, 1.0);
                                }
                            }
                            */
                            return;
                        }

                        // 计算运动
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
                            .map(|ob| Obstacle {
                                x: ob.x - xl,
                                y1: ob.y1,
                                y2: ob.y2,
                            })
                            .filter(|ob| ob.x > -w)
                            .collect();

                        if obstacles.last().map(|ob| ob.x).unwrap_or_default() < w - 2. * OB_WIDTH {
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
