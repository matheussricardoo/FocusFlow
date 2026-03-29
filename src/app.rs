use leptos::*;
use gloo_storage::{LocalStorage, Storage};
use serde::{Serialize, Deserialize};
use crate::components::timer::Timer;
use crate::components::stats::{StatsFooter, StatsView};
use crate::components::projects::ProjectsView;
use crate::components::settings::SettingsModal;

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
pub struct PersistedSettings {
    pub focus_mins: u32,
    pub break_mins: u32,
    pub total_rounds: u32,
}

impl Default for PersistedSettings {
    fn default() -> Self {
        Self {
            focus_mins: 25,
            break_mins: 5,
            total_rounds: 4,
        }
    }
}

#[derive(Clone, Copy)]
pub struct AppSettings {
    pub focus_mins: RwSignal<u32>,
    pub break_mins: RwSignal<u32>,
    pub total_rounds: RwSignal<u32>,
}

#[derive(Clone, Copy)]
pub struct AppProjects {
    pub list: RwSignal<Vec<Project>>,
    pub active_id: RwSignal<Option<String>>,
}

#[component]
pub fn App() -> impl IntoView {
    // Load Settings
    let initial_settings: PersistedSettings = LocalStorage::get("focusflow_settings")
        .unwrap_or_default();

    let focus_mins = create_rw_signal(initial_settings.focus_mins);
    let break_mins = create_rw_signal(initial_settings.break_mins);
    let total_rounds = create_rw_signal(initial_settings.total_rounds);

    let settings = AppSettings {
        focus_mins, break_mins, total_rounds,
    };
    provide_context(settings);

    // Save Settings
    create_effect(move |_| {
        let f = focus_mins.get(); let b = break_mins.get(); let r = total_rounds.get();
        let _ = LocalStorage::set("focusflow_settings", PersistedSettings { focus_mins: f, break_mins: b, total_rounds: r });
    });

    // Load Projects
    let initial_projects: Vec<Project> = LocalStorage::get("focusflow_projects")
        .unwrap_or_else(|_| Vec::new());
        
    let initial_active: Option<String> = LocalStorage::get("focusflow_active_project")
        .unwrap_or(None);
        
    let projects_list = create_rw_signal(initial_projects);
    let active_id = create_rw_signal(initial_active);
    
    let app_projects = AppProjects { list: projects_list, active_id };
    provide_context(app_projects);
    
    // Save Projects
    create_effect(move |_| {
        let _ = LocalStorage::set("focusflow_projects", projects_list.get());
    });
    
    // Save Active Project
    create_effect(move |_| {
        let _ = LocalStorage::set("focusflow_active_project", active_id.get());
    });

    let (active_tab, set_active_tab) = create_signal(Tab::Focus);
    let (show_settings, set_show_settings) = create_signal(false);

    view! {
        <header>
            <div class="logo">"FOCUSFLOW"</div>
            <div class="header-icons">
                <svg class="icon icon-hover" on:click=move |_| set_show_settings.set(true) viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
                    <path stroke-linecap="round" stroke-linejoin="round" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                </svg>
            </div>
        </header>

        <main class="main-layout">
            <aside class="sidebar">
                <nav class="sidebar-nav">
                    <span class=move || if active_tab.get() == Tab::Focus { "sidebar-item active" } else { "sidebar-item" }
                          on:click=move |_| set_active_tab.set(Tab::Focus)>"01 FOCUS"</span>
                    <span class=move || if active_tab.get() == Tab::Projects { "sidebar-item active" } else { "sidebar-item" }
                          on:click=move |_| set_active_tab.set(Tab::Projects)>"02 PROJECTS"</span>
                    <span class=move || if active_tab.get() == Tab::Stats { "sidebar-item active" } else { "sidebar-item" }
                          on:click=move |_| set_active_tab.set(Tab::Stats)>"03 STATS"</span>
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
