use crate::components::projects::ProjectsView;
use crate::components::settings::SettingsModal;
use crate::components::stats::{StatsFooter, StatsView};
use crate::components::timer::Timer;
use gloo_storage::{LocalStorage, Storage};
use js_sys::Date;
use leptos::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsCast;
use web_sys::{window, Notification};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Tab {
    Focus,
    Projects,
    Stats,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ProjectStatus {
    Planned,
    InProgress,
    Completed,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub status: ProjectStatus,
    pub eta_hours: u32,
    pub progress: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(default)]
pub struct PersistedSettings {
    pub focus_mins: u32,
    pub break_mins: u32,
    pub total_rounds: u32,
    pub notifications_enabled: bool,
    pub sound_enabled: bool,
    pub auto_start_next: bool,
}

impl Default for PersistedSettings {
    fn default() -> Self {
        Self {
            focus_mins: 25,
            break_mins: 5,
            total_rounds: 4,
            notifications_enabled: true,
            sound_enabled: true,
            auto_start_next: false,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PersistedTimerState {
    pub time_remaining: u32,
    pub is_running: bool,
    pub is_focus: bool,
    pub current_round: u32,
    pub target_end_ms: Option<f64>,
}

#[derive(Clone, Copy)]
pub struct AppSettings {
    pub focus_mins: RwSignal<u32>,
    pub break_mins: RwSignal<u32>,
    pub total_rounds: RwSignal<u32>,
    pub notifications_enabled: RwSignal<bool>,
    pub sound_enabled: RwSignal<bool>,
    pub auto_start_next: RwSignal<bool>,
}

#[derive(Clone, Copy)]
pub struct AppProjects {
    pub list: RwSignal<Vec<Project>>,
    pub active_id: RwSignal<Option<String>>,
}

#[derive(Clone, Copy)]
pub struct AppTimer {
    pub time_remaining: RwSignal<u32>,
    pub is_running: RwSignal<bool>,
    pub is_focus: RwSignal<bool>,
    pub current_round: RwSignal<u32>,
    pub target_end_ms: RwSignal<Option<f64>>,
}

fn now_ms() -> f64 {
    Date::now()
}

fn focus_secs(settings: AppSettings) -> u32 {
    settings.focus_mins.get().saturating_mul(60)
}

fn break_secs(settings: AppSettings) -> u32 {
    settings.break_mins.get().saturating_mul(60)
}

fn request_notification_permission_if_needed(enabled: bool) {
    if enabled {
        let _ = Notification::request_permission();
    }
}

fn show_timer_done_notification(is_focus_done: bool) {
    let title = if is_focus_done {
        "Focus session complete"
    } else {
        "Break complete"
    };

    let body = if is_focus_done {
        "Time for a break."
    } else {
        "Time to get back to focus."
    };

    let opts = web_sys::NotificationOptions::new();
    opts.set_body(body);
    let _ = Notification::new_with_options(title, &opts);
}

fn play_beep() {
    let Some(win) = window() else { return };
    let Ok(Some(audio_ctx_ctor)) = js_sys::Reflect::get(&win, &"AudioContext".into())
        .or_else(|_| js_sys::Reflect::get(&win, &"webkitAudioContext".into()))
        .map(|v| if v.is_undefined() { None } else { Some(v) })
    else {
        return;
    };

    let Ok(ctx_val) = js_sys::Reflect::construct(
        audio_ctx_ctor.unchecked_ref::<js_sys::Function>(),
        &js_sys::Array::new(),
    ) else {
        return;
    };

    let Ok(create_osc) = js_sys::Reflect::get(&ctx_val, &"createOscillator".into()) else {
        return;
    };
    let Ok(osc) = js_sys::Reflect::apply(
        create_osc.unchecked_ref::<js_sys::Function>(),
        &ctx_val,
        &js_sys::Array::new(),
    ) else {
        return;
    };

    let Ok(create_gain) = js_sys::Reflect::get(&ctx_val, &"createGain".into()) else {
        return;
    };
    let Ok(gain) = js_sys::Reflect::apply(
        create_gain.unchecked_ref::<js_sys::Function>(),
        &ctx_val,
        &js_sys::Array::new(),
    ) else {
        return;
    };

    let _ = js_sys::Reflect::set(&osc, &"type".into(), &"sine".into());

    if let Ok(freq_obj) = js_sys::Reflect::get(&osc, &"frequency".into()) {
        let _ = js_sys::Reflect::set(&freq_obj, &"value".into(), &880.0.into());
    }

    if let Ok(gain_obj) = js_sys::Reflect::get(&gain, &"gain".into()) {
        let _ = js_sys::Reflect::set(&gain_obj, &"value".into(), &0.0001.into());
    }

    if let Ok(connect_fn) = js_sys::Reflect::get(&osc, &"connect".into()) {
        let _ = js_sys::Reflect::apply(
            connect_fn.unchecked_ref::<js_sys::Function>(),
            &osc,
            &js_sys::Array::of1(&gain),
        );
    }

    if let Ok(dest) = js_sys::Reflect::get(&ctx_val, &"destination".into()) {
        if let Ok(connect_fn) = js_sys::Reflect::get(&gain, &"connect".into()) {
            let _ = js_sys::Reflect::apply(
                connect_fn.unchecked_ref::<js_sys::Function>(),
                &gain,
                &js_sys::Array::of1(&dest),
            );
        }
    }

    let current_time = js_sys::Reflect::get(&ctx_val, &"currentTime".into())
        .ok()
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0);

    if let Ok(gain_param) = js_sys::Reflect::get(&gain, &"gain".into()) {
        if let Ok(set_value_at_time) = js_sys::Reflect::get(&gain_param, &"setValueAtTime".into()) {
            let args = js_sys::Array::new();
            args.push(&0.0001.into());
            args.push(&current_time.into());
            let _ = js_sys::Reflect::apply(
                set_value_at_time.unchecked_ref::<js_sys::Function>(),
                &gain_param,
                &args,
            );
        }

        if let Ok(exp_ramp) =
            js_sys::Reflect::get(&gain_param, &"exponentialRampToValueAtTime".into())
        {
            let args_up = js_sys::Array::new();
            args_up.push(&0.08.into());
            args_up.push(&(current_time + 0.02).into());
            let _ = js_sys::Reflect::apply(
                exp_ramp.unchecked_ref::<js_sys::Function>(),
                &gain_param,
                &args_up,
            );

            let args_down = js_sys::Array::new();
            args_down.push(&0.0001.into());
            args_down.push(&(current_time + 0.35).into());
            let _ = js_sys::Reflect::apply(
                exp_ramp.unchecked_ref::<js_sys::Function>(),
                &gain_param,
                &args_down,
            );
        }
    }

    if let Ok(start_fn) = js_sys::Reflect::get(&osc, &"start".into()) {
        let _ = js_sys::Reflect::apply(
            start_fn.unchecked_ref::<js_sys::Function>(),
            &osc,
            &js_sys::Array::of1(&current_time.into()),
        );
    }

    if let Ok(stop_fn) = js_sys::Reflect::get(&osc, &"stop".into()) {
        let _ = js_sys::Reflect::apply(
            stop_fn.unchecked_ref::<js_sys::Function>(),
            &osc,
            &js_sys::Array::of1(&(current_time + 0.4).into()),
        );
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Load Settings
    let initial_settings: PersistedSettings =
        LocalStorage::get("focusflow_settings").unwrap_or_default();

    let focus_mins = create_rw_signal(initial_settings.focus_mins);
    let break_mins = create_rw_signal(initial_settings.break_mins);
    let total_rounds = create_rw_signal(initial_settings.total_rounds);
    let notifications_enabled = create_rw_signal(initial_settings.notifications_enabled);
    let sound_enabled = create_rw_signal(initial_settings.sound_enabled);
    let auto_start_next = create_rw_signal(initial_settings.auto_start_next);

    let settings = AppSettings {
        focus_mins,
        break_mins,
        total_rounds,
        notifications_enabled,
        sound_enabled,
        auto_start_next,
    };
    provide_context(settings);

    // Save Settings
    create_effect(move |_| {
        let f = focus_mins.get();
        let b = break_mins.get();
        let r = total_rounds.get();
        let n = notifications_enabled.get();
        let s = sound_enabled.get();
        let a = auto_start_next.get();
        let _ = LocalStorage::set(
            "focusflow_settings",
            PersistedSettings {
                focus_mins: f,
                break_mins: b,
                total_rounds: r,
                notifications_enabled: n,
                sound_enabled: s,
                auto_start_next: a,
            },
        );
    });

    // Load Projects
    let initial_projects: Vec<Project> =
        LocalStorage::get("focusflow_projects").unwrap_or_else(|_| Vec::new());
    let initial_active: Option<String> =
        LocalStorage::get("focusflow_active_project").unwrap_or(None);

    let projects_list = create_rw_signal(initial_projects);
    let active_id = create_rw_signal(initial_active);

    let app_projects = AppProjects {
        list: projects_list,
        active_id,
    };
    provide_context(app_projects);

    // Save Projects
    create_effect(move |_| {
        let _ = LocalStorage::set("focusflow_projects", projects_list.get());
    });

    // Save Active Project
    create_effect(move |_| {
        let _ = LocalStorage::set("focusflow_active_project", active_id.get());
    });

    // Load timer state
    let initial_timer: PersistedTimerState =
        LocalStorage::get("focusflow_timer").unwrap_or_else(|_| PersistedTimerState {
            time_remaining: initial_settings.focus_mins.saturating_mul(60),
            is_running: false,
            is_focus: true,
            current_round: 1,
            target_end_ms: None,
        });

    let time_remaining = create_rw_signal(initial_timer.time_remaining.max(1));
    let is_running = create_rw_signal(initial_timer.is_running);
    let is_focus = create_rw_signal(initial_timer.is_focus);
    let current_round = create_rw_signal(initial_timer.current_round.max(1));
    let target_end_ms = create_rw_signal(initial_timer.target_end_ms);

    let timer = AppTimer {
        time_remaining,
        is_running,
        is_focus,
        current_round,
        target_end_ms,
    };
    provide_context(timer);

    create_effect(move |_| {
        request_notification_permission_if_needed(settings.notifications_enabled.get());
    });

    // Ensure timer is consistent when app restores from persisted running state
    create_effect(move |_| {
        if is_running.get() {
            if let Some(target) = target_end_ms.get() {
                let now = now_ms();
                if target <= now {
                    is_running.set(false);
                    target_end_ms.set(None);

                    let was_focus = is_focus.get();
                    if settings.notifications_enabled.get() {
                        show_timer_done_notification(was_focus);
                    }
                    if settings.sound_enabled.get() {
                        play_beep();
                    }

                    let next_secs = if was_focus {
                        is_focus.set(false);
                        break_secs(settings)
                    } else {
                        is_focus.set(true);
                        current_round.update(|r| *r = r.saturating_add(1));
                        focus_secs(settings)
                    };
                    time_remaining.set(next_secs.max(1));

                    if settings.auto_start_next.get() {
                        let target = now_ms() + next_secs as f64 * 1000.0;
                        target_end_ms.set(Some(target));
                        is_running.set(true);
                    }
                }
            } else {
                // Running without target shouldn't happen; recover
                let secs = time_remaining.get().max(1) as f64;
                target_end_ms.set(Some(now_ms() + secs * 1000.0));
            }
        }
    });

    // Main ticking loop (timestamp-based precision)
    create_effect(move |_| {
        let _handle = set_interval_with_handle(
            move || {
                if !is_running.get() {
                    return;
                }

                let Some(target) = target_end_ms.get() else {
                    let secs = time_remaining.get().max(1) as f64;
                    target_end_ms.set(Some(now_ms() + secs * 1000.0));
                    return;
                };

                let now = now_ms();
                if now >= target {
                    is_running.set(false);
                    target_end_ms.set(None);
                    time_remaining.set(0);

                    let was_focus = is_focus.get();
                    if settings.notifications_enabled.get() {
                        show_timer_done_notification(was_focus);
                    }
                    if settings.sound_enabled.get() {
                        play_beep();
                    }

                    let next_secs = if was_focus {
                        is_focus.set(false);
                        break_secs(settings)
                    } else {
                        is_focus.set(true);
                        current_round.update(|r| *r = r.saturating_add(1));
                        focus_secs(settings)
                    };
                    time_remaining.set(next_secs.max(1));

                    if settings.auto_start_next.get() {
                        let target = now_ms() + next_secs as f64 * 1000.0;
                        target_end_ms.set(Some(target));
                        is_running.set(true);
                    }

                    return;
                }

                let remaining_secs = ((target - now) / 1000.0).ceil() as u32;
                time_remaining.set(remaining_secs.max(1));
            },
            std::time::Duration::from_millis(250),
        )
        .expect("Failed to set timer interval");

        on_cleanup(move || {
            _handle.clear();
        });
    });

    // React to settings changes without hard-resetting a running timer
    create_effect(move |_| {
        let _ = settings.focus_mins.get();
        let _ = settings.break_mins.get();

        if is_running.get() {
            // Keep current countdown stable while running
            return;
        }

        let next = if is_focus.get() {
            focus_secs(settings)
        } else {
            break_secs(settings)
        };
        time_remaining.set(next.max(1));
        target_end_ms.set(None);
    });

    // Persist timer state
    create_effect(move |_| {
        let state = PersistedTimerState {
            time_remaining: time_remaining.get(),
            is_running: is_running.get(),
            is_focus: is_focus.get(),
            current_round: current_round.get(),
            target_end_ms: target_end_ms.get(),
        };
        let _ = LocalStorage::set("focusflow_timer", state);
    });

    let (active_tab, set_active_tab) = create_signal(Tab::Focus);
    let (show_settings, set_show_settings) = create_signal(false);

    view! {
        <header>
            <div class="logo">"FOCUSFLOW"</div>
            <div class="header-icons">
                <svg
                    class="icon icon-hover"
                    on:click=move |_| set_show_settings.set(true)
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                >
                    <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"
                    />
                    <path stroke-linecap="round" stroke-linejoin="round" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                </svg>
            </div>
        </header>

        <main class="main-layout">
            <aside class="sidebar">
                <nav class="sidebar-nav">
                    <span
                        class=move || if active_tab.get() == Tab::Focus { "sidebar-item active" } else { "sidebar-item" }
                        on:click=move |_| set_active_tab.set(Tab::Focus)
                    >
                        "01 FOCUS"
                    </span>
                    <span
                        class=move || if active_tab.get() == Tab::Projects { "sidebar-item active" } else { "sidebar-item" }
                        on:click=move |_| set_active_tab.set(Tab::Projects)
                    >
                        "02 PROJECTS"
                    </span>
                    <span
                        class=move || if active_tab.get() == Tab::Stats { "sidebar-item active" } else { "sidebar-item" }
                        on:click=move |_| set_active_tab.set(Tab::Stats)
                    >
                        "03 STATS"
                    </span>
                </nav>
            </aside>

            <section class="content-area">
                <Show when=move || active_tab.get() == Tab::Focus fallback=|| ()>
                    <Timer />
                    <StatsFooter />
                </Show>

                <Show when=move || active_tab.get() == Tab::Projects fallback=|| ()>
                    <ProjectsView />
                </Show>

                <Show when=move || active_tab.get() == Tab::Stats fallback=|| ()>
                    <StatsView />
                </Show>
            </section>
        </main>

        <Show when=move || show_settings.get() fallback=|| ()>
            <SettingsModal on_close=move || set_show_settings.set(false) />
        </Show>
    }
}
