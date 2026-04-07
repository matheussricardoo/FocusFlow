use crate::app::{play_beep, AppSettings};
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
    let (temp_sound_volume, set_temp_sound_volume) = create_signal(settings.sound_volume.get());
    let (temp_auto_start, set_temp_auto_start) = create_signal(settings.auto_start_next.get());
    let (temp_theme_dark, set_temp_theme_dark) = create_signal(settings.theme_dark.get());

    let cancel = on_close.clone();

    let save = move |_| {
        settings.focus_mins.set(temp_focus.get());
        settings.break_mins.set(temp_break.get());
        settings.total_rounds.set(temp_rounds.get());
        settings.notifications_enabled.set(temp_notifications.get());
        settings.sound_enabled.set(temp_sound.get());
        settings.sound_volume.set(temp_sound_volume.get());
        settings.auto_start_next.set(temp_auto_start.get());
        settings.theme_dark.set(temp_theme_dark.get());
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

                <div class="input-group volume-group">
                    <div class="volume-header">
                        <span class="toggle-label">"SOUND VOLUME"</span>
                        <span class="volume-value">
                            {move || format!("{:.0}%", temp_sound_volume.get() * 100.0)}
                        </span>
                    </div>
                    <input class="volume-slider" type="range"
                           min="0" max="1" step="0.05"
                           on:input=move |ev| if let Ok(v) = event_target_value(&ev).parse() { set_temp_sound_volume.set(v) }
                           prop:value=temp_sound_volume
                           prop:disabled=move || !temp_sound.get() />
                    <div class="volume-actions">
                        <button class="btn btn-secondary btn-compact"
                                type="button"
                                prop:disabled=move || !temp_sound.get()
                                on:click=move |_| if temp_sound.get() { play_beep(temp_sound_volume.get()); }>
                            "TEST SOUND"
                        </button>
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

                <div class="input-group toggle-group">
                    <div class="toggle-row">
                        <span class="toggle-label">"DARK THEME"</span>
                        <label class="switch">
                            <input type="checkbox"
                                   on:change=move |ev| set_temp_theme_dark.set(event_target_checked(&ev))
                                   prop:checked=temp_theme_dark />
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
