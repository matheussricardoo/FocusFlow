use leptos::*;
use crate::app::AppSettings;

#[component]
pub fn SettingsModal<F>(on_close: F) -> impl IntoView 
where
    F: Fn() + 'static + Clone,
{
    let settings = use_context::<AppSettings>().expect("AppSettings not found");
    
    let (temp_focus, set_temp_focus) = create_signal(settings.focus_mins.get());
    let (temp_break, set_temp_break) = create_signal(settings.break_mins.get());
    let (temp_rounds, set_temp_rounds) = create_signal(settings.total_rounds.get());

    let cancel = on_close.clone();

    let save = move |_| {
        settings.focus_mins.set(temp_focus.get());
        settings.break_mins.set(temp_break.get());
        settings.total_rounds.set(temp_rounds.get());
        on_close();
    };

    view! {
        <div class="modal-overlay">
            <div class="modal-content">
                <h2 class="modal-title">"SETTINGS"</h2>
                
                <div class="input-group">
                    <label>"FOCUS DURATION (MIN)"</label>
                    <input type="number" 
                           on:input=move |ev| if let Ok(v) = event_target_value(&ev).parse() { set_temp_focus.set(v) }
                           prop:value=temp_focus 
                           min="1" max="120" />
                </div>
                
                <div class="input-group">
                    <label>"BREAK DURATION (MIN)"</label>
                    <input type="number" 
                           on:input=move |ev| if let Ok(v) = event_target_value(&ev).parse() { set_temp_break.set(v) }
                           prop:value=temp_break 
                           min="1" max="60" />
                </div>
                
                <div class="input-group">
                    <label>"TOTAL ROUNDS"</label>
                    <input type="number" 
                           on:input=move |ev| if let Ok(v) = event_target_value(&ev).parse() { set_temp_rounds.set(v) }
                           prop:value=temp_rounds 
                           min="1" max="20" />
                </div>
                
                <div class="modal-actions">
                    <button class="btn btn-secondary" on:click=move |_| cancel()>"CANCEL"</button>
                    <button class="btn btn-primary" on:click=save>"SAVE"</button>
                </div>
            </div>
        </div>
    }
}
