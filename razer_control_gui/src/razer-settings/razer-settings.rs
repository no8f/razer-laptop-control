use std::io::ErrorKind;

use adw::{
    AboutDialog, ActionRow, Application, ApplicationWindow, ButtonRow, ComboRow, HeaderBar,
    PreferencesGroup, PreferencesPage, SwitchRow, ToolbarView, ViewStack, ViewSwitcher,
    WindowTitle,
};
use adw::{PreferencesRow, prelude::*};
use gtk::{
    Box, Button, ColorDialog, ColorDialogButton, Label, License, LinkButton, Scale,
    SingleSelection, StringList,
};
use gtk::{glib, glib::clone, prelude::*};

// sudo apt install libgdk-pixbuf2.0-dev libcairo-dev libatk1.0-dev
// sudo apt install libpango1.0-dev

#[path = "../comms.rs"]
mod comms;
mod error_handling;
mod util;
mod widgets;

use error_handling::*;
use util::*;
use widgets::*;

#[path = "../lib.rs"]
mod lib;

fn send_data(opt: comms::DaemonCommand) -> Option<comms::DaemonResponse> {
    match comms::try_bind() {
        Ok(socket) => comms::send_to_daemon(opt, socket),
        Err(error) if error.kind() == ErrorKind::NotFound => {
            crash_with_msg("Can't connect to the daemon");
        }
        Err(error) => {
            println!("Error opening socket: {error}");
            None
        }
    }
}

fn get_device_name() -> Option<String> {
    let response = send_data(comms::DaemonCommand::GetDeviceName)?;

    use comms::DaemonResponse::*;
    match response {
        GetDeviceName { name } => Some(name),
        response => {
            // This should not happen
            println!("Instead of GetDeviceName got {response:?}");
            None
        }
    }
}

fn get_bho() -> Option<(bool, u8)> {
    let response = send_data(comms::DaemonCommand::GetBatteryHealthOptimizer())?;

    use comms::DaemonResponse::*;
    match response {
        GetBatteryHealthOptimizer { is_on, threshold } => Some((is_on, threshold)),
        response => {
            // This should not happen
            println!("Instead of GetBatteryHealthOptimizer got {response:?}");
            None
        }
    }
}

fn set_bho(is_on: bool, threshold: u8) -> Option<bool> {
    let response = send_data(comms::DaemonCommand::SetBatteryHealthOptimizer { is_on, threshold })?;

    use comms::DaemonResponse::*;
    match response {
        SetBatteryHealthOptimizer { result } => Some(result),
        response => {
            // This should not happen
            println!("Instead of SetBatteryHealthOptimizer got {response:?}");
            None
        }
    }
}

fn get_brightness(ac: bool) -> Option<u8> {
    let ac = if ac { 1 } else { 0 };
    let response = send_data(comms::DaemonCommand::GetBrightness { ac })?;

    use comms::DaemonResponse::*;
    match response {
        GetBrightness { result } => Some(result),
        response => {
            // This should not happen
            println!("Instead of GetBrightness got {response:?}");
            None
        }
    }
}

fn set_brightness(ac: bool, val: u8) -> Option<bool> {
    let ac = if ac { 1 } else { 0 };
    let response = send_data(comms::DaemonCommand::SetBrightness { ac, val })?;

    use comms::DaemonResponse::*;
    match response {
        SetBrightness { result } => Some(result),
        response => {
            // This should not happen
            println!("Instead of SetBrightness got {response:?}");
            None
        }
    }
}

fn get_logo(ac: bool) -> Option<u8> {
    let ac = if ac { 1 } else { 0 };
    let response = send_data(comms::DaemonCommand::GetLogoLedState { ac })?;

    use comms::DaemonResponse::*;
    match response {
        GetLogoLedState { logo_state } => Some(logo_state),
        response => {
            // This should not happen
            println!("Instead of GetLogoLedState got {response:?}");
            None
        }
    }
}

fn set_logo(ac: bool, logo_state: u8) -> Option<bool> {
    let ac = if ac { 1 } else { 0 };
    let response = send_data(comms::DaemonCommand::SetLogoLedState { ac, logo_state })?;

    use comms::DaemonResponse::*;
    match response {
        SetLogoLedState { result } => Some(result),
        response => {
            // This should not happen
            println!("Instead of SetLogoLedState got {response:?}");
            None
        }
    }
}

fn set_effect(name: &str, values: Vec<u8>) -> Option<bool> {
    let response = send_data(comms::DaemonCommand::SetEffect {
        name: name.into(),
        params: values,
    })?;

    use comms::DaemonResponse::*;
    match response {
        SetEffect { result } => Some(result),
        response => {
            // This should not happen
            println!("Instead of SetEffect got {response:?}");
            None
        }
    }
}

fn get_power(ac: bool) -> Option<(u8, u8, u8)> {
    let ac = if ac { 1 } else { 0 };
    let mut result = (0, 0, 0);

    let response = send_data(comms::DaemonCommand::GetPwrLevel { ac })?;
    use comms::DaemonResponse::*;
    match response {
        GetPwrLevel { pwr } => {
            result.0 = pwr;
        }
        response => {
            // This should not happen
            println!("Instead of GetPwrLevel got {response:?}");
            return None;
        }
    }

    let response = send_data(comms::DaemonCommand::GetCPUBoost { ac })?;
    use comms::DaemonResponse::*;
    match response {
        GetCPUBoost { cpu } => {
            result.1 = cpu;
        }
        response => {
            // This should not happen
            println!("Instead of GetCPUBoost got {response:?}");
            return None;
        }
    }

    let response = send_data(comms::DaemonCommand::GetGPUBoost { ac })?;
    use comms::DaemonResponse::*;
    match response {
        GetGPUBoost { gpu } => {
            result.2 = gpu;
        }
        response => {
            // This should not happen
            println!("Instead of GetGPUBoost got {response:?}");
            return None;
        }
    }

    Some(result)
}

fn set_power(ac: bool, power: (u8, u8, u8)) -> Option<bool> {
    let ac = if ac { 1 } else { 0 };
    let response = send_data(comms::DaemonCommand::SetPowerMode {
        ac,
        pwr: power.0,
        cpu: power.1,
        gpu: power.2,
    })?;

    use comms::DaemonResponse::*;
    match response {
        SetPowerMode { result } => Some(result),
        response => {
            // This should not happen
            println!("Instead of SetPowerMode got {response:?}");
            None
        }
    }
}

fn get_fan_speed(ac: bool) -> Option<i32> {
    let ac = if ac { 1 } else { 0 };
    let response = send_data(comms::DaemonCommand::GetFanSpeed { ac })?;

    use comms::DaemonResponse::*;
    match response {
        GetFanSpeed { rpm } => Some(rpm),
        response => {
            // This should not happen
            println!("Instead of GetFanSpeed got {response:?}");
            None
        }
    }
}

fn set_fan_speed(ac: bool, value: i32) -> Option<bool> {
    let ac = if ac { 1 } else { 0 };
    let response = send_data(comms::DaemonCommand::SetFanSpeed { ac, rpm: value })?;

    use comms::DaemonResponse::*;
    match response {
        SetFanSpeed { result } => Some(result),
        response => {
            // This should not happen
            println!("Instead of SetFanSpeed got {response:?}");
            None
        }
    }
}

fn show_about(window: &ApplicationWindow, device: &lib::SupportedDevice) {
    let name = &device.name;
    let features = &device.features.join(",");

    let about = adw::AboutDialog::builder()
        .application_name("Razer Laptop Control")
        .application_icon("com.no8f.razerLaptopControl")
        .developer_name("Noah Felber")
        .issue_url("https://github.com/no8f/razer-laptop-control/issues")
        .website("https://github.com/no8f/razer-laptop-control")
        .comments(format!("<span size='large' weight='bold' >Laptop Information</span>\n\n - Model: {name} \n - Features: {features}"))
        .version("0.2.0")
        .developers(vec!["Noah Felber", "Josu Goñi"])
        .copyright("© 2025 Noah Felber, © 2024 Josu Goñi")
        .license_type(License::Gpl30)
        .build();

    about.present(Some(window));
}

fn main() {
    setup_panic_hook();
    gtk::init().or_crash("Failed to initialize GTK.");

    let device_file =
        std::fs::read_to_string(lib::DEVICE_FILE).or_crash("Failed to read the device file");
    let devices: Vec<lib::SupportedDevice> =
        serde_json::from_str(&device_file).or_crash("Failed to parse the device file");

    let device_name = get_device_name().or_crash("Failed to get device name");

    let app = Application::builder()
        .application_id("com.no8f.razerLaptopControl") // TODO: Change this name
        .build();

    app.connect_activate(move |app| {
        // For now we get the device from the device name. One is duplicated but
        // its settings are the same.
        // TODO: Document this or make it more robust
        let device = devices
            .iter()
            .find(|d| d.name == device_name)
            .or_crash("Failed to get device info");

        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(640)
            .default_height(740)
            .title("Razer Settings")
            .build();

        let ac_settings_page = make_page(true, device.clone());
        let battery_settings_page = make_page(false, device.clone());
        let general_page = make_general_page();

        let stack = ViewStack::new();

        stack.add_titled_with_icon(&ac_settings_page, Some("AC"), "AC", "ac-adapter-symbolic");
        stack.add_titled_with_icon(
            &battery_settings_page,
            Some("Battery"),
            "Battery",
            "battery",
        );
        stack.add_titled_with_icon(
            &general_page,
            Some("General"),
            "General",
            "preferences-system-symbolic",
        );
        stack.set_property("enable-transitions", true);

        let stack_switcher = ViewSwitcher::builder().build();

        stack_switcher.set_stack(Some(&stack));
        stack_switcher.set_halign(gtk::Align::Center);
        stack_switcher.set_policy(adw::ViewSwitcherPolicy::Wide);

        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
        vbox.append(&stack_switcher);
        vbox.append(&stack);

        let header_bar = HeaderBar::new();

        header_bar.set_title_widget(Some(&WindowTitle::new("Razer Laptop Control", "")));

        // Add a button to the header
        let header_button = Button::from_icon_name("help-about");
        header_bar.pack_start(&header_button);

        header_button.connect_clicked(clone!(
            #[strong]
            window,
            #[strong]
            device,
            move |button| {
                show_about(&window, &device);
            }
        ));

        let toolbar = ToolbarView::new();
        toolbar.add_top_bar(&header_bar);

        vbox.set_margin_top(12);
        toolbar.set_content(Some(&vbox));

        window.set_content(Some(&toolbar));

        window.present();

        // If we know we are not running on AC, we show the battery tab by
        // default
        match check_if_running_on_ac_power() {
            Some(false) => stack.set_visible_child_name("Battery"),
            _ => {}
        }
    });

    app.run();
}

fn make_page(ac: bool, device: lib::SupportedDevice) -> PreferencesPage {
    let fan_speed = get_fan_speed(ac).or_crash("Error reading fan speed");
    let brightness = get_brightness(ac).or_crash("Error reading brightness");
    let power = get_power(ac);

    let min_fan_speed = *device.fan.get(0).or_crash("Invalid fan values") as f64;
    let max_fan_speed = *device.fan.get(1).or_crash("Invalid fan values") as f64;

    let settings_page = PreferencesPage::new();

    // Logo section
    if device.has_logo() {
        let logo = get_logo(ac).or_crash("Error reading logo");

        let settings_section = PreferencesGroup::new();
        settings_section.set_title("Logo");
        settings_page.add(&settings_section);

        let logo_options = StringList::new(&["Off", "On", "Breathing"]);
        let logo_options_dropdown = ComboRow::new();
        logo_options_dropdown.set_model(Some(&logo_options));
        logo_options_dropdown.set_title("Turn on logo");
        logo_options_dropdown.set_selected(logo as u32);
        logo_options_dropdown.connect_selected_notify(move |options| {
            let logo = options.selected() as u8;
            set_logo(ac, logo);
            let logo = get_logo(ac).or_crash("Error reading logo").clamp(0, 2);
            options.set_selected(logo as u32);
        });
        settings_section.add(&logo_options_dropdown);
    }

    // Power section
    if let Some(power) = power {
        let settings_section = PreferencesGroup::new();
        settings_section.set_title("Power");
        settings_page.add(&settings_section);

        let power_profile = StringList::new(&["Balanced", "Gaming", "Creator", "Silent", "Custom"]);
        let power_profile_dropdown = ComboRow::new();
        power_profile_dropdown.set_model(Some(&power_profile));
        power_profile_dropdown.set_selected(power.0 as u32);
        power_profile_dropdown.set_title("Power Profile");

        settings_section.add(&power_profile_dropdown);

        let cpu_boost = StringList::new(&["Low", "Medium", "High"]);

        if device.can_boost() {
            cpu_boost.append("Boost")
        };

        let cpu_boost_dropdown = ComboRow::new();
        cpu_boost_dropdown.set_model(Some(&cpu_boost));
        cpu_boost_dropdown.set_selected(power.1 as u32);
        cpu_boost_dropdown.set_title("CPU Boost");
        settings_section.add(&cpu_boost_dropdown);

        let gpu_boost = StringList::new(&["Low", "Medium", "High"]);
        let gpu_boost_dropdown = ComboRow::new();
        gpu_boost_dropdown.set_model(Some(&gpu_boost));
        gpu_boost_dropdown.set_selected(power.2 as u32);
        gpu_boost_dropdown.set_title("GPU Boost");
        settings_section.add(&gpu_boost_dropdown);

        if power.0 == 4 {
            gpu_boost_dropdown.set_visible(true);
            gpu_boost_dropdown.set_visible(true);
        } else {
            gpu_boost_dropdown.set_visible(false);
            gpu_boost_dropdown.set_visible(false);
        }

        power_profile_dropdown.connect_selectable_notify(clone!(
            #[weak]
            gpu_boost_dropdown,
            #[weak]
            cpu_boost_dropdown,
            move |power_profile_dropdown| {
                let profile = power_profile_dropdown.selected() as u8;
                let cpu = cpu_boost_dropdown.selected() as u8;
                let gpu = gpu_boost_dropdown.selected() as u8;
                set_power(ac, (profile, cpu, gpu)).or_crash("Error setting power");

                let power = get_power(ac).or_crash("Error reading power");
                power_profile_dropdown.set_selected(power.0 as u32);
                cpu_boost_dropdown.set_selected(power.1 as u32);
                gpu_boost_dropdown.set_selected(power.2 as u32);

                if power.0 == 4 {
                    gpu_boost_dropdown.set_visible(true);
                    gpu_boost_dropdown.set_visible(true);
                } else {
                    gpu_boost_dropdown.set_visible(false);
                    gpu_boost_dropdown.set_visible(false);
                }
            }
        ));

        cpu_boost_dropdown.connect_activated(clone!(
            #[weak]
            power_profile_dropdown,
            #[weak]
            gpu_boost_dropdown,
            move |cpu_boost_dropdown| {
                let profile = power_profile_dropdown.selected() as u8;
                let cpu = cpu_boost_dropdown.selected() as u8;
                let gpu = gpu_boost_dropdown.selected() as u8;
                set_power(ac, (profile, cpu, gpu)).or_crash("Error setting power");

                let power = get_power(ac).or_crash("Error reading power");
                power_profile_dropdown.set_selected(power.0 as u32);
                cpu_boost_dropdown.set_selected(power.1 as u32);
                gpu_boost_dropdown.set_selected(power.2 as u32);
            }
        ));

        gpu_boost_dropdown.connect_activated(clone!(
            #[weak]
            power_profile_dropdown,
            #[weak]
            cpu_boost_dropdown,
            move |gpu_boost_dropdown| {
                let profile = power_profile_dropdown.selected() as u8;
                let cpu = cpu_boost_dropdown.selected() as u8;
                let gpu = gpu_boost_dropdown.selected() as u8;
                set_power(ac, (profile, cpu, gpu)).or_crash("Error setting power");

                let power = get_power(ac).or_crash("Error reading power");
                power_profile_dropdown.set_selected(power.0 as u32);
                cpu_boost_dropdown.set_selected(power.1 as u32);
                gpu_boost_dropdown.set_selected(power.2 as u32);
            }
        ));
    }

    // Fan Speed Section
    let settings_section = PreferencesGroup::new(); //settings_page.add_section(Some("Fan Speed"));
    settings_section.set_title("Fan Control");
    settings_page.add(&settings_section);

    let switch = SwitchRow::new();
    let auto = fan_speed == 0;
    switch.set_active(auto);
    switch.set_title("Auto");

    settings_section.add(&switch);
    let scale = Scale::with_range(
        gtk::Orientation::Horizontal,
        min_fan_speed,
        max_fan_speed,
        1f64,
    );
    scale.set_value(fan_speed as f64);
    scale.set_sensitive(fan_speed != 0);
    scale.set_width_request(150);
    scale.set_draw_value(true);

    let update_label = gtk::Label::default();
    scale.connect_change_value(clone!(
        #[weak]
        switch,
        #[upgrade_or_panic]
        move |scale, stype, value| {
            let value = value.clamp(min_fan_speed, max_fan_speed);
            set_fan_speed(ac, value as i32).or_crash("Error setting fan speed");
            let fan_speed = get_fan_speed(ac).or_crash("Error reading fan speed");
            let auto = fan_speed == 0;
            scale.set_value(fan_speed as f64);
            scale.set_sensitive(!auto);
            switch.set_active(auto);
            update_label.set_text(&format!("Horizontal scale value: {:.2}", value));
            return glib::Propagation::Stop;
        },
    ));

    switch.connect_active_notify(clone!(
        #[weak]
        scale,
        #[upgrade_or_panic]
        move |switch| {
            set_fan_speed(
                ac,
                if switch.is_active() {
                    0
                } else {
                    min_fan_speed as i32
                },
            )
            .or_crash("Error setting fan speed");

            let fan_speed = get_fan_speed(ac).or_crash("Error reading fan speed");
            let auto = fan_speed == 0;

            scale.set_value(fan_speed as f64);
            scale.set_sensitive(!auto);
            switch.set_active(auto);
        }
    ));

    let row = ActionRow::new();
    row.set_title("Fan Speed");
    row.add_suffix(&scale);
    settings_section.add(&row);

    // Keyboard Section
    let settings_section = PreferencesGroup::new(); //settings_page.add_section(Some("Keyboard"));
    settings_section.set_title("Keyboard");
    settings_page.add(&settings_section);

    let scale = Scale::with_range(gtk::Orientation::Horizontal, 0f64, 100f64, 1f64);
    scale.set_value(brightness as f64);
    scale.set_width_request(150);
    scale.set_draw_value(true);
    scale.connect_change_value(move |scale, stype, value| {
        let value = value.clamp(0f64, 100f64);
        set_brightness(ac, value as u8).or_crash("Error setting brigthness");
        let brightness = get_brightness(ac).or_crash("Error reading brightness");
        scale.set_value(brightness as f64);
        return gtk::glib::Propagation::Stop;
    });
    let row = ActionRow::new();
    row.set_title("Brightness");
    row.add_suffix(&scale);
    settings_section.add(&row);

    settings_page
}

fn make_general_page() -> PreferencesPage {
    let bho = get_bho();

    let page = PreferencesPage::new();

    // Keyboard Section
    let settings_section = PreferencesGroup::new(); //page.add_section(Some("Keyboard"));
    settings_section.set_title("Keyboard");
    page.add(&settings_section);

    let effect_options =
        StringList::new(&["Static", "Static Gradient", "Wave Gradient", "Breathing"]);
    let effect_options_dropdown = ComboRow::new();
    effect_options_dropdown.set_model(Some(&effect_options));
    effect_options_dropdown.set_selected(0);
    effect_options_dropdown.set_title("Effect");

    settings_section.add(&effect_options_dropdown);

    let color_picker = ColorDialogButton::new(Some(ColorDialog::new()));
    let row = ActionRow::new();
    row.set_title("Color 1");
    row.add_suffix(&color_picker);
    settings_section.add(&row);

    let color_picker_2 = ColorDialogButton::new(Some(ColorDialog::new()));
    let row = ActionRow::new();
    row.set_title("Color 2");
    row.add_suffix(&color_picker_2);
    settings_section.add(&row);

    let button = ButtonRow::new();
    button.set_title("Write effect");
    button.set_action_name(Some("Write"));

    settings_section.add(&button);

    button.connect_activated(clone!(
        #[weak]
        effect_options_dropdown,
        #[weak]
        color_picker,
        #[weak]
        color_picker_2,
        #[upgrade_or_panic]
        move |_| {
            let color = color_picker.rgba();
            let red = (color.red() * 255.0).round() as u8;
            let green = (color.green() * 255.0).round() as u8;
            let blue = (color.blue() * 255.0).round() as u8;

            let color = color_picker_2.rgba();
            let red2 = (color.red() * 255.0).round() as u8;
            let green2 = (color.green() * 255.0).round() as u8;
            let blue2 = (color.blue() * 255.0).round() as u8;

            let effect = effect_options_dropdown.selected();
            match effect {
                0 => {
                    set_effect("static", vec![red, green, blue]).or_crash("Failed to set effect");
                }
                1 => {
                    set_effect(
                        "static_gradient",
                        vec![red, green, blue, red2, green2, blue2],
                    )
                    .or_crash("Failed to set effect");
                }
                2 => {
                    set_effect("wave_gradient", vec![red, green, blue, red2, green2, blue2])
                        .or_crash("Failed to set effect");
                }
                3 => {
                    set_effect("breathing_single", vec![red, green, blue, 10])
                        .or_crash("Failed to set effect");
                }
                _ => {}
            }
        }
    ));

    effect_options_dropdown.connect_activated(clone!(
        #[weak]
        color_picker,
        #[weak]
        color_picker_2,
        #[upgrade_or_panic]
        move |options| {
            let logo = options.selected(); // Unwrap: There is always one active

            match logo {
                0 => {
                    // TODO: Color 1 visible
                }
                1 => {
                    // TODO: Color 1 and 2 visible
                }
                2 => {
                    // TODO: Color 1 and 2 visible
                }
                3 => {
                    // TODO: Color 1, 2, and duration visible
                }
                _ => {}
            }
        }
    ));

    // Battery Health Optimizer section
    if let Some(bho) = bho {
        let settings_section = PreferencesGroup::new(); //page.add_section(Some("Battery Health Optimizer"));
        settings_section.set_title("Battery Health Optimizer");
        page.add(&settings_section);

        let switch = SwitchRow::new();
        switch.set_active(bho.0);
        switch.set_title("Enable Battery Health Optimizer");
        settings_section.add(&switch);
        let scale = Scale::with_range(gtk::Orientation::Horizontal, 65f64, 80f64, 1f64);
        scale.set_value(bho.1 as f64);
        scale.set_width_request(150);
        scale.set_draw_value(true);
        scale.connect_change_value(clone!(
            #[weak]
            switch,
            #[upgrade_or_panic]
            move |scale, stype, value| {
                let is_on = switch.is_active();
                let threshold = value.clamp(50f64, 80f64) as u8;

                set_bho(is_on, threshold).or_crash("Error setting bho");

                let (is_on, threshold) = get_bho().or_crash("Error reading bho");

                scale.set_value(threshold as f64);
                scale.set_visible(is_on);
                scale.set_sensitive(is_on);

                return gtk::glib::Propagation::Stop;
            }
        ));
        scale.set_sensitive(bho.0);
        switch.connect_active_notify(clone!(
            #[weak]
            scale,
            #[upgrade_or_panic]
            move |switch| {
                let threshold = scale.value().clamp(50f64, 80f64) as u8;

                set_bho(switch.is_active(), threshold);

                let (is_on, threshold) = get_bho().or_crash("Error reading bho");

                scale.set_value(threshold as f64);
                scale.set_visible(is_on);
                scale.set_sensitive(is_on);
            }
        ));
        let row = ActionRow::new();
        row.set_title("Theshold");
        row.add_suffix(&scale);

        settings_section.add(&row);
    }

    page
}
