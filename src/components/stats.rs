use crate::app::{AppProjects, AppSettings, AppStats};
use leptos::*;

#[component]
pub fn StatsFooter() -> impl IntoView {
    let settings = use_context::<AppSettings>().expect("AppSettings missing in stats");
    let projects_ctx = use_context::<AppProjects>().expect("AppProjects missing in stats");
    let stats_ctx = use_context::<AppStats>().expect("AppStats missing in stats");

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
            <div class=move || { if stats_ctx.daily_count.get() >= settings.total_rounds.get() { "stat-card goal-met" } else { "stat-card" } }>
                <div class="stat-label">
                    {move || if stats_ctx.daily_count.get() >= settings.total_rounds.get() { "DAILY GOAL • MET" } else { "DAILY GOAL" }}
                </div>
                <div class="stat-value">{move || format!("{:02} / {:02} SESSIONS", stats_ctx.daily_count.get(), settings.total_rounds.get())}</div>
            </div>
            <div class="stat-card">
                <div class="stat-label">"STREAK"</div>
                <div class="stat-value">{move || format!("{} DAYS", stats_ctx.streak_count.get())}</div>
            </div>
        </div>
    }
}

#[component]
pub fn StatsView() -> impl IntoView {
    let settings = use_context::<AppSettings>().expect("AppSettings missing in stats");
    let stats_ctx = use_context::<AppStats>().expect("AppStats missing in stats");

    view! {
        <div class="full-stats-section fade-in">
            <h1 class="task-title">"ANALYTICS"</h1>
            <p class="description-text" style="margin-bottom: 4rem;">"Daily focus progress and streak overview."</p>

            <div class="hero-stats">
                <div class="hero-stat-box">
                    <div class="stat-label">"DAILY PROGRESS"</div>
                    <div class="hero-stat-value">
                        {move || format!("{:02} / {:02}", stats_ctx.daily_count.get(), settings.total_rounds.get())}
                    </div>
                    <div class="progress-bar">
                        <div class="progress-fill" style=move || {
                            let total = settings.total_rounds.get();
                            let done = stats_ctx.daily_count.get();
                            let pct = if total == 0 { 0.0 } else { ((done as f64 / total as f64) * 100.0).min(100.0) };
                            format!("width: {:.2}%;", pct)
                        }></div>
                    </div>
                </div>

                <div class="hero-stat-box">
                    <div class="stat-label">"CURRENT STREAK"</div>
                    <div class="hero-stat-value">
                        {move || format!("{} DAYS", stats_ctx.streak_count.get())}
                    </div>
                </div>

                <div class="hero-stat-box">
                    <div class="stat-label">"TOTAL SESSIONS"</div>
                    <div class="hero-stat-value">
                        {move || format!("{:02}", stats_ctx.total_focus_sessions.get())}
                    </div>
                </div>
            </div>

            <div class="hero-stats" style="margin-top: 2rem;">
                <div class="hero-stat-box" style="width: 100%;">
                    <div class="stat-label">"WEEKLY HISTORY"</div>
                    <div class="weekly-chart">
                        <For
                            each=move || stats_ctx.weekly_counts.get()
                            key=|day| day.date.clone()
                            children=move |day| {
                                let label = day.date.split('-').last().unwrap_or("").to_string();
                                view! {
                                    <div class="weekly-bar">
                                        <div class="weekly-bar-fill" style=move || {
                                            let max = stats_ctx.weekly_counts.get().iter().map(|d| d.count).max().unwrap_or(0);
                                            let pct = if max == 0 { 0.0 } else { ((day.count as f64 / max as f64) * 100.0).max(6.0) };
                                            format!("height: {:.2}%;", pct)
                                        }></div>
                                        <div class="weekly-label">{label}</div>
                                    </div>
                                }
                            }
                        />
                    </div>
                </div>
            </div>
        </div>
    }
}
