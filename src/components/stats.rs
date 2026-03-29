use leptos::*;
use crate::app::{AppSettings, AppProjects};

#[component]
pub fn StatsFooter() -> impl IntoView {
    let settings = use_context::<AppSettings>().expect("AppSettings missing in stats");
    let projects_ctx = use_context::<AppProjects>().expect("AppProjects missing in stats");
    
    view! {
        <div class="stats-section fade-in">
            <div class="stat-card">
                <div class="stat-label">"CURRENT TASK"</div>
                <div class="stat-value" style="white-space: nowrap; overflow: hidden; text-overflow: ellipsis; max-width: 100%;">
                    {move || {
                        if let Some(active_id) = projects_ctx.active_id.get() {
                            if let Some(proj) = projects_ctx.list.with(|l| l.iter().find(|p| p.id == active_id).cloned()) {
                                return proj.name;
                            }
                        }
                        "None".to_string()
                    }}
                </div>
            </div>
            <div class="stat-card">
                <div class="stat-label">"DAILY GOAL"</div>
                <div class="stat-value">{move || format!("0 / {:02} SESSIONS", settings.total_rounds.get())}</div>
            </div>
            <div class="stat-card">
                <div class="stat-label">"STREAK"</div>
                <div class="stat-value">"0 DAYS"</div>
            </div>
        </div>
    }
}

#[component]
pub fn StatsView() -> impl IntoView {
    view! {
        <div class="full-stats-section fade-in">
            <h1 class="task-title">"ANALYTICS"</h1>
            <p class="description-text" style="margin-bottom: 4rem;">"Complete some tasks to generate your analytics."</p>

            <div class="empty-state" style="border: 1px solid var(--border-color); width: 100%; height: 350px; display: flex; align-items: center; justify-content: center; color: var(--text-muted); font-weight: 800; letter-spacing: 2px;">
                "NO DATA AVAILABLE"
            </div>
        </div>
    }
}
