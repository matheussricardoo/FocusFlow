use crate::app::{AppProjects, AppSettings, AppStats, AppTimer};
use leptos::*;

#[component]
pub fn Timer() -> impl IntoView {
    let settings = use_context::<AppSettings>().expect("AppSettings missed");
    let projects_ctx = use_context::<AppProjects>().expect("AppProjects missed");
    let stats_ctx = use_context::<AppStats>().expect("AppStats missed");
    let timer = use_context::<AppTimer>().expect("AppTimer missed");

    let toggle_timer = move |_| {
        if timer.is_running.get() {
            timer.is_running.set(false);
            timer.target_end_ms.set(None);
        } else {
            let now = js_sys::Date::now();
            let secs = timer.time_remaining.get().max(1) as f64;
            timer.target_end_ms.set(Some(now + secs * 1000.0));
            timer.is_running.set(true);
            timer.has_started.set(true);
        }
    };

    let reset_timer = move |_| {
        timer.is_running.set(false);
        timer.target_end_ms.set(None);
        let next = if timer.is_focus.get() {
            settings.focus_mins.get().saturating_mul(60)
        } else {
            settings.break_mins.get().saturating_mul(60)
        };
        timer.time_remaining.set(next.max(1));
    };

    let set_focus_mode = move |_| {
        timer.is_running.set(false);
        timer.target_end_ms.set(None);
        timer.is_focus.set(true);
        timer
            .time_remaining
            .set(settings.focus_mins.get().saturating_mul(60).max(1));
    };

    let set_break_mode = move |_| {
        timer.is_running.set(false);
        timer.target_end_ms.set(None);
        timer.is_focus.set(false);
        timer
            .time_remaining
            .set(settings.break_mins.get().saturating_mul(60).max(1));
    };

    view! {
        <div class="hero-section fade-in">
            <div class="info-column">
                <div class="session-info">
                    {move || format!("SESSION {}/{}", timer.current_round.get(), settings.total_rounds.get())}
                </div>

                <div class="timer-status">
                    <span class=move || if settings.notifications_enabled.get() { "status-pill on" } else { "status-pill off" }>
                        {move || if settings.notifications_enabled.get() { "NOTIF ON" } else { "NOTIF OFF" }}
                    </span>
                    <span class=move || if settings.sound_enabled.get() { "status-pill on" } else { "status-pill off" }>
                        {move || if settings.sound_enabled.get() { "SOUND ON" } else { "SOUND OFF" }}
                    </span>
                    <span class=move || if settings.auto_start_next.get() { "status-pill on" } else { "status-pill off" }>
                        {move || if settings.auto_start_next.get() { "AUTO ON" } else { "AUTO OFF" }}
                    </span>
                </div>
                <Show when=move || { stats_ctx.daily_count.get() >= settings.total_rounds.get() } fallback=|| ()>
                    <div class="goal-met-indicator">"DAILY GOAL MET"</div>
                </Show>

                <div class="task-title">
                    {move || {
                        if !timer.is_focus.get() {
                            return "Rest & Recover".to_string();
                        }

                        if let Some(active_id) = projects_ctx.active_id.get() {
                            if let Some(proj) = projects_ctx.list.with(|l| l.iter().find(|p| p.id == active_id).cloned()) {
                                return proj.name;
                            }
                        }

                        "Deep Work".to_string()
                    }}
                </div>

                <div class="toggle-container">
                    <button
                        class=move || if timer.is_focus.get() { "toggle-btn active" } else { "toggle-btn" }
                        on:click=set_focus_mode
                    >
                        "FOCUS"
                    </button>
                    <div class="toggle-divider"></div>
                    <button
                        class=move || if !timer.is_focus.get() { "toggle-btn active" } else { "toggle-btn" }
                        on:click=set_break_mode
                    >
                        "BREAK"
                    </button>
                </div>

                <p class="description-text">
                    {move || if timer.is_focus.get() {
                        "Eliminate all digital noise. Focus strictly on the primary objective."
                    } else {
                        "Step away from the screen. Take a breath and let your mind wander."
                    }}
                </p>
            </div>

            <div class="timer-column">
                <div class="timer-display">
                    {move || {
                        let secs = timer.time_remaining.get();
                        format!("{:02}:{:02}", secs / 60, secs % 60)
                    }}
                </div>

                <div class="timer-controls">
                    <button class="btn btn-primary" on:click=toggle_timer>
                        {move || if timer.is_running.get() { "PAUSE" } else { "START" }}
                    </button>
                    <button class="btn btn-secondary" on:click=reset_timer>
                        "RESET"
                    </button>
                </div>
            </div>
        </div>
    }
}
