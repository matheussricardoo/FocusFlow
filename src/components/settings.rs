use crate::app::AppSettings;
use leptos::*;

#[component]
pub fn SettingsModal<F>(on_close: F) -> impl IntoView
where
    F: Fn() + 'static + Clone,
{
    let settings = use_context::<AppSettings>().expect("AppSettings not found");

    let (temp_focus, set_temp_focus) = create_signal(settings.focus_mins.get());
    let (temp_break, set_temp_break) = create_signal(settings.break_mins.get());
    let (temp_rounds, set_temp_rounds) = create_signal(settings.total_rounds.get());
    let (temp_notifications, set_temp_notifications) =
        create_signal(settings.notifications_enabled.get());
    let (temp_sound, set_temp_sound) = create_signal(settings.sound_enabled.get());
    let (temp_auto_start, set_temp_auto_start) = create_signal(settings.auto_start_next.get());

    let cancel = on_close.clone();

    let save = move |_| {
        settings.focus_mins.set(temp_focus.get());
        settings.break_mins.set(temp_break.get());
        settings.total_rounds.set(temp_rounds.get());
        settings.notifications_enabled.set(temp_notifications.get());
        settings.sound_enabled.set(temp_sound.get());
        settings.auto_start_next.set(temp_auto_start.get());
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

                <div class="input-group toggle-group">
                    <div class="toggle-row">
                        <span class="toggle-label">"NOTIFICATIONS"</span>
                        <label class="switch">
                            <input type="checkbox"
                                   on:change=move |ev| set_temp_notifications.set(event_target_checked(&ev))
                                   prop:checked=temp_notifications />
                            <span class="slider"></span>
                        </label>
                    </div>
                </div>

                <div class="input-group toggle-group">
                    <div class="toggle-row">
                        <span class="toggle-label">"SOUND"</span>
                        <label class="switch">
                            <input type="checkbox"
                                   on:change=move |ev| set_temp_sound.set(event_target_checked(&ev))
                                   prop:checked=temp_sound />
                            <span class="slider"></span>
                        </label>
                    </div>
                </div>

                <div class="input-group toggle-group">
                    <div class="toggle-row">
                        <span class="toggle-label">"AUTO-START NEXT CYCLE"</span>
                        <label class="switch">
                            <input type="checkbox"
                                   on:change=move |ev| set_temp_auto_start.set(event_target_checked(&ev))
                                   prop:checked=temp_auto_start />
                            <span class="slider"></span>
                        </label>
                    </div>
                </div>

                <div class="modal-actions">
                    <button class="btn btn-secondary" on:click=move |_| cancel()>"CANCEL"</button>
                    <button class="btn btn-primary" on:click=save>"SAVE"</button>
                </div>
            </div>
        </div>
    }
}
