use crate::app::{AppProjects, Project, ProjectStatus};
use leptos::*;
use uuid::Uuid;

#[component]
pub fn NewProjectModal<F>(on_close: F) -> impl IntoView
where
    F: Fn() + 'static + Clone,
{
    let projects_ctx = use_context::<AppProjects>().expect("AppProjects not found");

    let (temp_name, set_temp_name) = create_signal(String::new());
    let (temp_eta, set_temp_eta) = create_signal(0u32);

    let cancel = on_close.clone();

    let save = move |_| {
        let name = temp_name.get();
        if name.trim().is_empty() {
            return;
        }

        let new_proj = Project {
            id: Uuid::new_v4().to_string(),
            name,
            status: ProjectStatus::Planned,
            eta_hours: temp_eta.get(),
            progress: 0,
        };

        projects_ctx.list.update(|list| list.push(new_proj));
        on_close();
    };

    view! {
        <div class="modal-overlay">
            <div class="modal-content">
                <h2 class="modal-title">"NEW PROJECT"</h2>

                <div class="input-group">
                    <label>"PROJECT NAME"</label>
                    <input type="text"
                           on:input=move |ev| set_temp_name.set(event_target_value(&ev))
                           prop:value=temp_name
                           placeholder="Ex: Refactor the UI" />
                </div>

                <div class="input-group">
                    <label>"ESTIMATED EFFORT (HOURS)"</label>
                    <input type="number"
                           on:input=move |ev| if let Ok(v) = event_target_value(&ev).parse() { set_temp_eta.set(v) }
                           prop:value=temp_eta
                           min="0" />
                </div>

                <div class="modal-actions">
                    <button class="btn btn-secondary" on:click=move |_| cancel()>"CANCEL"</button>
                    <button class="btn btn-primary" on:click=save>"CREATE"</button>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn ProjectsView() -> impl IntoView {
    let projects_ctx = use_context::<AppProjects>().expect("AppProjects not found");
    let (show_new_modal, set_show_new_modal) = create_signal(false);

    let render_status = |status: &ProjectStatus| match status {
        ProjectStatus::Planned => view! { <span class="project-tag">"PLANNED"</span> },
        ProjectStatus::InProgress => {
            view! { <span class="project-tag" style="color: #2196F3">"IN PROGRESS"</span> }
        }
        ProjectStatus::Completed => {
            view! { <span class="project-tag" style="color: #4CAF50">"COMPLETED"</span> }
        }
    };

    view! {
        <div class="projects-section fade-in">
            <h1 class="task-title">"PROJECTS"</h1>
            <p class="description-text" style="margin-bottom: 4rem;">"Manage your ongoing objectives and tasks."</p>

            <Show when=move || projects_ctx.list.with(|l| l.is_empty()) fallback=|| ()>
                <div class="empty-state" style="border: 2px dashed var(--border-color); padding: 4rem; text-align: center; color: var(--text-muted); font-weight: 800; letter-spacing: 2px;">
                    "NO PROJECTS YET"
                </div>
            </Show>

            <div class="projects-grid">
                <For
                    each=move || projects_ctx.list.get()
                    key=|proj| proj.id.clone()
                    children=move |proj| {
                        let fill_color = if proj.status == ProjectStatus::Completed { "#4CAF50" } else { "var(--text-main)" };

                        let id_memo = proj.id.clone();
                        let is_active = create_memo(move |_| projects_ctx.active_id.get() == Some(id_memo.clone()));

                        let id_delete = proj.id.clone();
                        let delete_proj = move |_| {
                            projects_ctx.list.update(|l| l.retain(|p| p.id != id_delete));
                            if is_active.get() {
                                projects_ctx.active_id.set(None);
                            }
                        };

                        let id_active = proj.id.clone();
                        let set_active = move |_| {
                            projects_ctx.active_id.set(Some(id_active.clone()));
                        };

                        let unset_active = move |_| {
                            projects_ctx.active_id.set(None);
                        };

                        let border_style = move || if is_active.get() { "border: 2px solid var(--text-main);" } else { "" };

                        view! {
                            <div class="project-card fade-in" style=border_style>
                                <div class="project-card-header">
                                    <div style="display: flex; gap: 1rem; align-items: center;">
                                        {render_status(&proj.status)}
                                        <Show when=move || is_active.get() fallback=move || view! {
                                            <span on:click=set_active.clone() style="cursor: pointer; font-size: 0.65rem; color: var(--text-muted); opacity: 0.8; font-weight: 800;">"○ SET ACTIVE"</span>
                                        }>
                                            <span on:click=unset_active.clone() style="cursor: pointer; font-size: 0.65rem; color: var(--text-main); font-weight: 800;">"◉ ACTIVE"</span>
                                        </Show>
                                    </div>
                                    <div style="display: flex; gap: 1rem;">
                                        <span class="project-eta">{format!("{}h effort", proj.eta_hours)}</span>
                                        <span on:click=delete_proj style="cursor: pointer; opacity: 0.5;" title="Delete">"✖"</span>
                                    </div>
                                </div>
                                <h3 class="project-name">{proj.name.clone()}</h3>
                                <div class="project-progress">
                                    <div class="progress-bar">
                                        <div class="progress-fill" style=format!("width: {}%; background: {};", proj.progress, fill_color)></div>
                                    </div>
                                </div>
                            </div>
                        }
                    }
                />
            </div>

            <button class="btn btn-primary" style="margin-top: 3rem;" on:click=move |_| set_show_new_modal.set(true)>
                "NEW PROJECT"
            </button>

            <Show when=move || show_new_modal.get() fallback=|| ()>
                <NewProjectModal on_close=move || set_show_new_modal.set(false) />
            </Show>
        </div>
    }
}
