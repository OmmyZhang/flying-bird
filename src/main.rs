use std::iter;

use gloo_utils::format::JsValueSerdeExt;
use rand::{thread_rng, Rng};
use serde::Serialize;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::{
    console, window, CanvasRenderingContext2d, Event, HtmlAudioElement, HtmlCanvasElement,
    HtmlImageElement,
};
use yew::{
    callback::Callback, function_component, html, use_effect_with, use_node_ref, use_state, Html,
    TargetCast,
};
use yew_hooks::use_interval;

const BG_COLOR: u8 = 240;
const OB_COLOR: u8 = 100;
const NEXT_OB_COLOR: u8 = 190;

const BIRD_SIZE: f64 = 128.;
const H_BIRD_RATIO: f64 = 8.0;
const MAX_BIRD_PX: f64 = 96.;
const CHECK_SIZE: f64 = BIRD_SIZE / 2.0 + 5.0;
const OB_WIDTH: f64 = 100.;
const NEXT_OB_WIDTH: f64 = 30.;

const HISTORY_LEN: usize = 250;
const HISTORY_COLOR_CHANGE: usize = 15;

const MIN_SPACE: f64 = 3. * BIRD_SIZE;
const INTERV: u32 = 8;
const V_MIN_2: f64 = 9.0;
const V_MAX_2: f64 = 81.0;
const ROTATE_UP: f64 = -0.022;
const ROTATE_DOWN_D: f64 = 0.12;

const N_LIFES: i32 = 10;
const RESTART_WATING_TIME: u32 = 1500;

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

struct MapConfig {
    w: f64,
    h: f64,
}

impl MapConfig {
    pub fn calc() -> Self {
        // 鸟的大小在canvas里面128以免太糊
        // 鸟的大小是屏幕高度的1/8，但不要超过200px
        // 不去纠结devicePixelRatio了，反正鸟的清晰度有限

        console::log_1(&JsValue::from_str("calc map size"));

        let screen_width = window().unwrap().inner_width().unwrap().as_f64().unwrap();
        let screen_height = window().unwrap().inner_height().unwrap().as_f64().unwrap();

        let h = f64::max(
            BIRD_SIZE * H_BIRD_RATIO,
            screen_height * BIRD_SIZE / MAX_BIRD_PX,
        );

        Self {
            h,
            w: h * screen_width / screen_height,
        }
    }
}

impl Obstacle {
    pub fn random_gen(last: Option<&Obstacle>, w: f64, h: f64, score: u32) -> Self {
        let mut rng = thread_rng();
        let dis = rng.gen_range(0.0..(6.0 - score as f64).max(4.0) * OB_WIDTH)
            + (3.0 - score as f64).max(0.0) * OB_WIDTH;
        let last_y1 = last.map(|ob| ob.y1).unwrap_or(h / 3.0);

        let space = rng.gen_range(MIN_SPACE..1.5 * MIN_SPACE);
        let max_dy = (dis / OB_WIDTH).powi(2) * BIRD_SIZE * 1.5;
        let y1 = rng.gen_range(
            (last_y1 - max_dy).max(0.0).min(h - space - 1.0)
                ..(last_y1 + max_dy).min(h - space),
        );

        Self {
            x: w + dis + 2.0 * OB_WIDTH,
            y1,
            y2: y1 + space,
        }
    }
}

fn get_best_score() -> u32 {
    let storage = window().unwrap().local_storage().unwrap().unwrap();
    storage
        .get_item("best_score")
        .unwrap()
        .and_then(|s| s.parse().ok())
        .unwrap_or(0)
}

fn set_best_score(score: u32) {
    let storage = window().unwrap().local_storage().unwrap().unwrap();
    storage.set_item("best_score", &score.to_string()).unwrap();
}

fn enter_fullscreen() {
    window()
        .unwrap()
        .document()
        .unwrap()
        .document_element()
        .unwrap()
        .request_fullscreen()
        .unwrap();
}

#[function_component(App)]
fn app() -> Html {
    let canvas_ref = use_node_ref();
    let audio_ref = use_node_ref();
    let audio_wall_ref = use_node_ref();
    let audio_after_ref = use_node_ref();

    let map_config = use_state(MapConfig::calc);
    let canvas_ctx = use_state(|| None);
    let bird_image = use_state(|| None);
    let angle = use_state(|| 0.);
    let pos = use_state(|| 0.);
    let is_flying = use_state(|| false);
    let history = use_state(Vec::<(f64, f64)>::new);
    let obstacles = use_state(Vec::<Obstacle>::new);

    let comming_obstacles_distance = use_state(|| 0_u32);

    let life = use_state(|| N_LIFES);
    let is_playing = use_state(|| false);
    let score = use_state(|| 0_u32);
    let best_score = use_state(get_best_score);
    let restart_waiting = use_state(|| 0_u32);

    // 初始化canvas
    {
        clone_all![canvas_ref, canvas_ctx, map_config];
        use_effect_with(canvas_ref, move |canvas_ref| {
            let canvas = canvas_ref
                .cast::<HtmlCanvasElement>()
                .expect("canvas_ref not attached");

            console::log_1(&JsValue::from_str("Set canvas size"));

            canvas.set_width(map_config.w as u32);
            canvas.set_height(map_config.h as u32);
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

            let update = Closure::wrap(Box::new(move || {
                let new_map_config = MapConfig::calc();
                canvas.set_width(new_map_config.w as u32);
                canvas.set_height(new_map_config.h as u32);
                map_config.set(new_map_config);
            }) as Box<dyn FnMut()>);
            window()
                .unwrap()
                .set_onresize(Some(update.as_ref().unchecked_ref()));
            update.forget();
        });
    }
    // 新的一局各种初始化
    {
        clone_all![
            is_playing,
            pos,
            angle,
            history,
            obstacles,
            score,
            life,
            audio_ref,
            audio_wall_ref,
            audio_after_ref,
            restart_waiting
        ];
        use_effect_with(is_playing, move |is_playing| {
            if **is_playing {
                if window().unwrap().inner_height().unwrap().as_f64().unwrap() < 300.0 {
                    enter_fullscreen();
                }
                pos.set(0.);
                angle.set(0.);
                history.set(vec![]);
                obstacles.set(vec![]);
                score.set(0);

                if let Some(audio) = audio_ref.cast::<HtmlAudioElement>() {
                    audio.set_current_time(0.0);
                    audio.set_volume(0.5);
                    let _ = audio.play().unwrap();
                }
            } else if *life < N_LIFES {
                console::log_1(&JsValue::from_str("Failed"));
                restart_waiting.set(RESTART_WATING_TIME);
                if let Some(audio) = audio_ref.cast::<HtmlAudioElement>() {
                    audio.pause().unwrap();
                }
                if let Some(audio) = audio_wall_ref.cast::<HtmlAudioElement>() {
                    audio.set_volume(1.0);
                    let _ = audio.play().unwrap();
                }
                if let Some(audio) = audio_after_ref.cast::<HtmlAudioElement>() {
                    audio.set_volume(0.15);
                    let _ = audio.play().unwrap();
                }
            }
        });
    }

    // 更新best_score
    {
        clone_all![score, best_score];
        use_effect_with(score, move |score| {
            if **score > *best_score {
                best_score.set(**score);
                set_best_score(**score);
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
        clone_all![is_flying, is_playing, restart_waiting];
        move || {
            if *is_flying {
                return;
            }
            is_flying.set(true);
            if *restart_waiting == 0 && !*is_playing {
                is_playing.set(true);
            }
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
            canvas_ctx,
            angle,
            bird_image,
            is_flying,
            pos,
            history,
            is_playing,
            life,
            score,
            comming_obstacles_distance,
            restart_waiting,
            map_config
        ];
        use_interval(
            move || {
                let MapConfig { w, h } = *map_config;
                if let Some(ctx) = canvas_ctx.as_ref() {
                    if let Some(bird) = bird_image.as_ref() {
                        if !*is_playing && *life < N_LIFES {
                            if *restart_waiting > 0 {
                                restart_waiting
                                    .set(*restart_waiting - INTERV.min(*restart_waiting));
                            }
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

                        // 给预警
                        if let Some(ob) = obstacles.iter().find(|ob| ob.x + OB_WIDTH > w) {
                            if ob.x > w {
                                comming_obstacles_distance.set(((ob.x - w) / 100.0) as u32);
                                ctx.set_fill_style(&JsValue::from_str(&format!(
                                    "rgb({NEXT_OB_COLOR}, {NEXT_OB_COLOR}, {NEXT_OB_COLOR})"
                                )));
                                ctx.fill_rect(w - NEXT_OB_WIDTH, 0.0, NEXT_OB_WIDTH, ob.y1);
                                ctx.fill_rect(w - NEXT_OB_WIDTH, ob.y2, NEXT_OB_WIDTH, h - ob.y2);
                            }
                        } else {
                            comming_obstacles_distance.set(0);
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
                                        .map(|idata| idata.data().0.iter().any(|v| *v < 50))
                                        .unwrap_or(false)
                                            || ctx
                                                .get_image_data(
                                                    ob.x + OB_WIDTH + 1.0,
                                                    start_y,
                                                    1.0,
                                                    (h - start_y).min(2.0 * CHECK_SIZE),
                                                )
                                                .map(|idata| idata.data().0.iter().any(|v| *v < 50))
                                                .unwrap_or(false)
                                            || ctx
                                                .get_image_data(ob.x, ob.y2, OB_WIDTH, -1.)
                                                .map(|idata| idata.data().0.iter().any(|v| *v < 50))
                                                .unwrap_or(false)
                                    })
                                    .unwrap_or(false)
                                    || ctx
                                        .get_image_data(ox - CHECK_SIZE, h, 2.0 * CHECK_SIZE, -1.0)
                                        .map(|idata| idata.data().0.iter().any(|v| *v < 50))
                                        .unwrap_or(false)
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
                                    .map(|idata| idata.data().0.iter().any(|v| *v < 50))
                                    .unwrap_or(false)
                                        || ctx
                                            .get_image_data(
                                                ob.x + OB_WIDTH + 1.0,
                                                start_y,
                                                1.0,
                                                (ob.y1 - start_y).min(2.0 * CHECK_SIZE),
                                            )
                                            .map(|idata| idata.data().0.iter().any(|v| *v < 50))
                                            .unwrap_or(false)
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
                                    .map(|idata| idata.data().0.iter().any(|v| *v < 50))
                                    .unwrap_or(false));

                        if down_collision || up_collision {
                            is_playing.set(false);
                            life.set(*life - 1);
                            /*
                            if let Some(ob) = curr_obstacles {
                                ctx.set_fill_style(&JsValue::from_str("red"));
                                if down_collision {
                                    let start_y = (pos_y - CHECK_SIZE).max(ob.y2);
                                    ctx.fill_rect(
                                        ob.x,
                                        start_y,
                                        -1.0,
                                        (h - start_y).min(2.0 * CHECK_SIZE),
                                    );
                                    ctx.fill_rect(ob.x, ob.y2, OB_WIDTH, -1.0);
                                }

                                if up_collision {
                                    let start_y = (pos_y - CHECK_SIZE).max(0.0);
                                    ctx.fill_rect(
                                        ob.x,
                                        start_y,
                                        -5.0,
                                        (ob.y1 - start_y).min(2.0 * CHECK_SIZE),
                                    );
                                    ctx.fill_rect(
                                        ob.x + OB_WIDTH + 1.0,
                                        start_y,
                                        5.0,
                                        (ob.y1 - start_y).min(2.0 * CHECK_SIZE),
                                    );
                                    ctx.fill_rect(ob.x, ob.y1, OB_WIDTH, 1.0);
                                }
                            }
                            */
                            return;
                        }

                        // 计算运动
                        let v = (V_MIN_2 + (pos_y / h) * (V_MAX_2 - V_MIN_2)).sqrt();

                        //console::log_1(&JsValue::from_str(&format!("{}", v)));
                        let (sin, cos) = angle.sin_cos();
                        let (xl, yl) = (v * cos, v * sin);
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

                        if let Some(ob) = curr_obstacles {
                            if ox < ob.x + OB_WIDTH && ox + xl > ob.x + OB_WIDTH {
                                score.set(*score + 1);
                            }
                        }
                        let mut new_obstacles: Vec<Obstacle> = obstacles
                            .iter()
                            .map(|ob| Obstacle {
                                x: ob.x - xl,
                                y1: ob.y1,
                                y2: ob.y2,
                            })
                            .filter(|ob| ob.x > -w)
                            .collect();

                        if obstacles.last().map(|ob| ob.x).unwrap_or_default() < w {
                            new_obstacles.push(Obstacle::random_gen(
                                obstacles.last(),
                                w,
                                h,
                                *score,
                            ));
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
            <audio loop={true} ref={audio_ref} preload={"auto"}>
                <source src="static/fantasy_world.mp3" type="audio/mpeg" />
            </audio>
            <audio ref={audio_wall_ref} preload={"auto"}>
                <source src="static/hitting_wall.mp3" type="audio/mpeg" />
            </audio>
            <audio ref={audio_after_ref} preload={"auto"}>
                <source src="static/string_end.mp3" type="audio/mpeg" />
            </audio>
            <div class="no-select">
                <img id="birdImage" src="static/bird.webp" onload={img_onload} />
                <span id="lifeCnt"> {*life} </span>
                <span class="score"> {format!("{:0>4}", *score)}</span>
                <span class="score best_score"> {format!("{:0>4}", *best_score)}</span>
            </div>
            if !*is_playing {
                <div id="hint" class="no-select">
                    <p>{ "Tap the screen or press any key to fly" }</p>
                </div>
                if *life == N_LIFES {
                    <div id="badges">
                        <a href="https://notbyai.fyi/">
                            <img
                                src="https://notbyai.fyi/img/written-by-human-not-by-ai-white.svg"
                                alt="written by human, not by AI"
                            />
                        </a>
                        <a href="https://github.com/OmmyZhang/flying-bird">
                            <img
                                src="https://github.githubassets.com/assets/GitHub-Mark-ea2971cee799.png"
                                alt="GitHub"
                            />
                        </a>
                        <a href="https://www.gnu.org/licenses/licenses.html#AGPL">
                            <img
                                src="https://www.gnu.org/graphics/agplv3-155x51.png"
                                alt="AGPL license"
                            />
                        </a>
                    </div>
                }
            }
            if *comming_obstacles_distance > 0 {
                <span id="next" class="no-select">{ *comming_obstacles_distance } { "m" }</span>
            }
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
