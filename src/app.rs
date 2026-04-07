use crate::components::projects::ProjectsView;
use crate::components::settings::SettingsModal;
use crate::components::stats::{StatsFooter, StatsView};
use crate::components::timer::Timer;
use gloo_storage::{LocalStorage, Storage};
use js_sys::Date;
use leptos::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::{JsCast, JsValue};
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
    pub sound_volume: f64,
    pub auto_start_next: bool,
    pub theme_dark: bool,
}

impl Default for PersistedSettings {
    fn default() -> Self {
        Self {
            focus_mins: 25,
            break_mins: 5,
            total_rounds: 4,
            notifications_enabled: true,
            sound_enabled: true,
            sound_volume: 1.0,
            auto_start_next: false,
            theme_dark: false,
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
    #[serde(default)]
    pub has_started: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DayCount {
    pub date: String,
    pub count: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(default)]
pub struct PersistedStats {
    pub daily_count: u32,
    pub daily_date: String,
    pub streak_count: u32,
    pub last_completed_date: Option<String>,
    pub weekly: Vec<DayCount>,
    pub total_focus_sessions: u32,
}

impl Default for PersistedStats {
    fn default() -> Self {
        let today = current_date_key();
        Self {
            daily_count: 0,
            daily_date: today,
            streak_count: 0,
            last_completed_date: None,
            weekly: build_weekly(&[]),
            total_focus_sessions: 0,
        }
    }
}

#[derive(Clone, Copy)]
pub struct AppSettings {
    pub focus_mins: RwSignal<u32>,
    pub break_mins: RwSignal<u32>,
    pub total_rounds: RwSignal<u32>,
    pub notifications_enabled: RwSignal<bool>,
    pub sound_enabled: RwSignal<bool>,
    pub sound_volume: RwSignal<f64>,
    pub auto_start_next: RwSignal<bool>,
    pub theme_dark: RwSignal<bool>,
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
    pub has_started: RwSignal<bool>,
}

#[derive(Clone, Copy)]
#[allow(dead_code)]
pub struct AppStats {
    pub daily_count: RwSignal<u32>,
    pub daily_date: RwSignal<String>,
    pub streak_count: RwSignal<u32>,
    pub last_completed_date: RwSignal<Option<String>>,
    pub weekly_counts: RwSignal<Vec<DayCount>>,
    pub total_focus_sessions: RwSignal<u32>,
}

fn now_ms() -> f64 {
    Date::now()
}

fn date_key_from_ms(ms: f64) -> String {
    let d = Date::new(&JsValue::from_f64(ms));
    let year = d.get_full_year() as i32;
    let month = (d.get_month() + 1) as i32;
    let day = d.get_date() as i32;
    format!("{:04}-{:02}-{:02}", year, month, day)
}

fn current_date_key() -> String {
    date_key_from_ms(now_ms())
}

fn date_key_days_ago(days: u32) -> String {
    date_key_from_ms(now_ms() - (days as f64) * 86_400_000.0)
}

fn yesterday_date_key() -> String {
    date_key_days_ago(1)
}

fn build_weekly(weekly: &[DayCount]) -> Vec<DayCount> {
    let mut out = Vec::new();
    for i in (0..7).rev() {
        let date = date_key_days_ago(i as u32);
        let count = weekly
            .iter()
            .find(|d| d.date == date)
            .map(|d| d.count)
            .unwrap_or(0);
        out.push(DayCount { date, count });
    }
    out
}

fn record_focus_completion(
    daily_count: RwSignal<u32>,
    daily_date: RwSignal<String>,
    streak_count: RwSignal<u32>,
    last_completed_date: RwSignal<Option<String>>,
    weekly_counts: RwSignal<Vec<DayCount>>,
    total_focus_sessions: RwSignal<u32>,
) {
    let today = current_date_key();

    if daily_date.get_untracked() != today {
        let yesterday = yesterday_date_key();
        let last = last_completed_date.get_untracked();
        if last.as_deref() != Some(&yesterday) {
            streak_count.set(0);
        }
        daily_date.set(today.clone());
        daily_count.set(0);
        weekly_counts.update(|w| {
            let rebuilt = build_weekly(w);
            *w = rebuilt;
        });
    }

    daily_count.update(|c| *c = c.saturating_add(1));
    total_focus_sessions.update(|t| *t = t.saturating_add(1));

    let last = last_completed_date.get_untracked();
    if last.as_deref() != Some(&today) {
        let yesterday = yesterday_date_key();
        if last.as_deref() == Some(&yesterday) {
            streak_count.update(|s| *s = s.saturating_add(1));
        } else {
            streak_count.set(1);
        }
        last_completed_date.set(Some(today.clone()));
    }

    weekly_counts.update(|w| {
        if let Some(entry) = w.iter_mut().find(|d| d.date == today) {
            entry.count = entry.count.saturating_add(1);
        } else {
            w.push(DayCount {
                date: today.clone(),
                count: 1,
            });
        }
        let rebuilt = build_weekly(w);
        *w = rebuilt;
    });
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

pub fn play_beep(volume: f64) {
    let volume = volume.clamp(0.0, 1.0);
    let peak = 0.12 * volume.max(0.05);
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
            args_up.push(&peak.into());
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
    let sound_volume = create_rw_signal(initial_settings.sound_volume);
    let auto_start_next = create_rw_signal(initial_settings.auto_start_next);
    let theme_dark = create_rw_signal(initial_settings.theme_dark);

    let settings = AppSettings {
        focus_mins,
        break_mins,
        total_rounds,
        notifications_enabled,
        sound_enabled,
        sound_volume,
        auto_start_next,
        theme_dark,
    };
    provide_context(settings);

    create_effect(move |_| {
        let Some(win) = window() else { return };
        let Some(doc) = win.document() else { return };
        let Some(body) = doc.body() else { return };
        let theme = if settings.theme_dark.get() {
            "dark"
        } else {
            "light"
        };
        let _ = body.set_attribute("data-theme", theme);
    });

    // Load Stats
    let initial_stats: PersistedStats = LocalStorage::get("focusflow_stats").unwrap_or_default();
    let today_key = current_date_key();
    let mut normalized_stats = initial_stats;
    if normalized_stats.daily_date != today_key {
        let yesterday = yesterday_date_key();
        if normalized_stats.last_completed_date.as_deref() != Some(&yesterday) {
            normalized_stats.streak_count = 0;
        }
        normalized_stats.daily_date = today_key;
        normalized_stats.daily_count = 0;
    }
    normalized_stats.weekly = build_weekly(&normalized_stats.weekly);

    let daily_count = create_rw_signal(normalized_stats.daily_count);
    let daily_date = create_rw_signal(normalized_stats.daily_date);
    let streak_count = create_rw_signal(normalized_stats.streak_count);
    let last_completed_date = create_rw_signal(normalized_stats.last_completed_date);
    let weekly_counts = create_rw_signal(normalized_stats.weekly);
    let total_focus_sessions = create_rw_signal(normalized_stats.total_focus_sessions);

    let app_stats = AppStats {
        daily_count,
        daily_date,
        streak_count,
        last_completed_date,
        weekly_counts,
        total_focus_sessions,
    };
    provide_context(app_stats);

    // Daily reset watcher
    create_effect(move |_| {
        let _handle = set_interval_with_handle(
            move || {
                let today = current_date_key();
                if daily_date.get_untracked() != today {
                    let yesterday = yesterday_date_key();
                    let last = last_completed_date.get_untracked();
                    if last.as_deref() != Some(&yesterday) {
                        streak_count.set(0);
                    }
                    daily_date.set(today);
                    daily_count.set(0);
                    weekly_counts.update(|w| {
                        let rebuilt = build_weekly(w);
                        *w = rebuilt;
                    });
                }
            },
            std::time::Duration::from_secs(60),
        )
        .expect("Failed to set daily stats interval");

        on_cleanup(move || {
            _handle.clear();
        });
    });

    // Save Stats
    create_effect(move |_| {
        let stats = PersistedStats {
            daily_count: daily_count.get(),
            daily_date: daily_date.get(),
            streak_count: streak_count.get(),
            last_completed_date: last_completed_date.get(),
            weekly: weekly_counts.get(),
            total_focus_sessions: total_focus_sessions.get(),
        };
        let _ = LocalStorage::set("focusflow_stats", stats);
    });

    // Save Settings
    create_effect(move |_| {
        let f = focus_mins.get();
        let b = break_mins.get();
        let r = total_rounds.get();
        let n = notifications_enabled.get();
        let s = sound_enabled.get();
        let v = sound_volume.get();
        let a = auto_start_next.get();
        let t = theme_dark.get();
        let _ = LocalStorage::set(
            "focusflow_settings",
            PersistedSettings {
                focus_mins: f,
                break_mins: b,
                total_rounds: r,
                notifications_enabled: n,
                sound_enabled: s,
                sound_volume: v,
                auto_start_next: a,
                theme_dark: t,
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
            has_started: false,
        });

    let time_remaining = create_rw_signal(initial_timer.time_remaining.max(1));
    let is_running = create_rw_signal(initial_timer.is_running);
    let is_focus = create_rw_signal(initial_timer.is_focus);
    let current_round = create_rw_signal(initial_timer.current_round.max(1));
    let target_end_ms = create_rw_signal(initial_timer.target_end_ms);
    let has_started = create_rw_signal(initial_timer.has_started || initial_timer.is_running);

    let timer = AppTimer {
        time_remaining,
        is_running,
        is_focus,
        current_round,
        target_end_ms,
        has_started,
    };
    provide_context(timer);

    create_effect(move |_| {
        request_notification_permission_if_needed(settings.notifications_enabled.get());
    });

    create_effect(move |_| {
        if is_running.get() {
            has_started.set(true);
        }
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
                        play_beep(settings.sound_volume.get());
                    }

                    if was_focus {
                        record_focus_completion(
                            daily_count,
                            daily_date,
                            streak_count,
                            last_completed_date,
                            weekly_counts,
                            total_focus_sessions,
                        );
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
                        play_beep(settings.sound_volume.get());
                    }

                    if was_focus {
                        record_focus_completion(
                            daily_count,
                            daily_date,
                            streak_count,
                            last_completed_date,
                            weekly_counts,
                            total_focus_sessions,
                        );
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
        let focus = settings.focus_mins.get();
        let break_m = settings.break_mins.get();

        if is_running.get_untracked() {
            // Keep current countdown stable while running or paused
            return;
        }

        let next = if is_focus.get_untracked() {
            focus.saturating_mul(60)
        } else {
            break_m.saturating_mul(60)
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
            has_started: has_started.get(),
        };
        let _ = LocalStorage::set("focusflow_timer", state);
    });

    // Update document title with timer and mode (running or paused)
    create_effect(move |_| {
        let Some(win) = window() else { return };
        let Some(doc) = win.document() else { return };

        if !has_started.get() {
            doc.set_title("FocusFlow • Pomodoro");
            return;
        }

        let secs = time_remaining.get();
        let mode_or_state = if is_running.get() {
            if is_focus.get() {
                "Focus"
            } else {
                "Break"
            }
        } else {
            "Paused"
        };
        let title = format!("{:02}:{:02} • {}", secs / 60, secs % 60, mode_or_state);
        doc.set_title(&title);
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
