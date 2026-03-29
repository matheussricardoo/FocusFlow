use leptos::*;
use std::time::Duration;
use crate::app::{AppSettings, AppProjects};

#[component]
pub fn Timer() -> impl IntoView {
    let settings = use_context::<AppSettings>().expect("AppSettings missed");
    let projects_ctx = use_context::<AppProjects>().expect("AppProjects missed");
    
    // In seconds
    let (time_remaining, set_time_remaining) = create_signal(settings.focus_mins.get() * 60);
    let (is_running, set_is_running) = create_signal(false);
    let (is_focus, set_is_focus) = create_signal(true);
    let (current_round, set_current_round) = create_signal(1);

    // Watch for settings changes to auto-reset
    create_effect(move |_| {
        let focus = settings.focus_mins.get();
        let break_m = settings.break_mins.get();
        
        // Auto-restart with the new settings
        set_is_running.set(false);
        set_time_remaining.set(if is_focus.get() { focus * 60 } else { break_m * 60 });
    });

    create_effect(move |_| {
        if is_running.get() {
            let handle = set_interval_with_handle(
                move || {
                    set_time_remaining.update(|t| {
                        if *t > 0 {
                            *t -= 1;
                        } else {
                            set_is_running.set(false);
                            let currently_focus = is_focus.get();
                            
                            if currently_focus {
                                set_is_focus.set(false);
                                set_time_remaining.set(settings.break_mins.get() * 60);
                            } else {
                                set_is_focus.set(true);
                                set_time_remaining.set(settings.focus_mins.get() * 60);
                                set_current_round.update(|r| *r += 1);
                            }
                        }
                    });
                },
                Duration::from_secs(1)
            ).expect("Failed to set interval");
            
            on_cleanup(move || {
                handle.clear();
            });
        }
    });

    let toggle_timer = move |_| {
        set_is_running.update(|running| *running = !*running);
    };

    let reset_timer = move |_| {
        set_is_running.set(false);
        set_time_remaining.set(if is_focus.get() { settings.focus_mins.get() * 60 } else { settings.break_mins.get() * 60 });
    };

    view! {
        <div class="hero-section fade-in">
            <div class="info-column">
                <div class="session-info">
                    {move || format!("SESSION {}/{}", current_round.get(), settings.total_rounds.get())}
                </div>
                <div class="task-title">
                    {move || {
                        if !is_focus.get() {
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
                    <button class=move || if is_focus.get() { "toggle-btn active" } else { "toggle-btn" } 
                            on:click=move |_| { set_is_focus.set(true); set_time_remaining.set(settings.focus_mins.get() * 60); set_is_running.set(false); }>
                        "FOCUS"
                    </button>
                    <div class="toggle-divider"></div>
                    <button class=move || if !is_focus.get() { "toggle-btn active" } else { "toggle-btn" } 
                            on:click=move |_| { set_is_focus.set(false); set_time_remaining.set(settings.break_mins.get() * 60); set_is_running.set(false); }>
                        "BREAK"
                    </button>
                </div>
                
                <p class="description-text">
                    {move || if is_focus.get() {
                        "Eliminate all digital noise. Focus strictly on the primary objective."
                    } else {
                        "Step away from the screen. Take a breath and let your mind wander."
                    }}
                </p>
            </div>
            
            <div class="timer-column">
                <div class="timer-display">
                    {move || format!("{:02}:{:02}", time_remaining.get() / 60, time_remaining.get() % 60)}
                </div>
                
                <div class="timer-controls">
                    <button class="btn btn-primary" on:click=toggle_timer>
                        {move || if is_running.get() { "PAUSE" } else { "START" }}
                    </button>
                    <button class="btn btn-secondary" on:click=reset_timer>
                        "RESET"
                    </button>
                </div>
            </div>
        </div>
    }
}
