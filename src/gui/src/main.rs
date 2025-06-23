use gtk4::prelude::*;
use gtk4::{
    glib, Application, ApplicationWindow, Box, Button, HeaderBar, Label, ScrolledWindow, TextView,
    Notebook, Frame, Grid, Separator, ProgressBar, Dialog, Entry, ResponseType
};
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::collections::HashSet;

const APP_ID: &str = "org.hardn.GUI";

// Global process tracker - just track PIDs
type ProcessTracker = Arc<Mutex<HashSet<u32>>>;

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("HARDN - Linux Security Hardening Sentinel")
        .default_width(1200)
        .default_height(800)
        .build();

    // Create process tracker for cleanup
    let process_tracker: ProcessTracker = Arc::new(Mutex::new(HashSet::new()));
    
    // Handle window close event
    let process_tracker_clone = process_tracker.clone();
    window.connect_close_request(move |_| {
        println!("🛡️  GUI closing - terminating all background processes...");
        
        // Kill all tracked processes
        if let Ok(pids) = process_tracker_clone.lock() {
            for &pid in pids.iter() {
                println!("🔪 Terminating process {}", pid);
                let _ = Command::new("kill")
                    .args(["-TERM", &pid.to_string()])
                    .output();
            }
        }
        
        // Also kill any lingering hardn processes
        let _ = Command::new("pkill")
            .args(["-f", "hardn"])
            .output();
            
        println!("✅ All processes terminated. GUI closed safely.");
        glib::Propagation::Proceed
    });

    let header_bar = HeaderBar::new();
    let title_box = Box::new(gtk4::Orientation::Horizontal, 10);
    
    let icon_label = Label::new(Some("🛡️"));
    icon_label.add_css_class("title-1");
    
    let title_label = Label::new(Some("HARDN Security Sentinel v2.0.0"));
    title_label.add_css_class("title-2");
    
    title_box.append(&icon_label);
    title_box.append(&title_label);
    header_bar.set_title_widget(Some(&title_box));
    
    let refresh_btn = Button::with_label("🔄 Refresh Status");
    refresh_btn.add_css_class("suggested-action");
    header_bar.pack_end(&refresh_btn);
    
    window.set_titlebar(Some(&header_bar));

    let notebook = Notebook::new();
    notebook.set_scrollable(true);
    
    create_dashboard_tab(&notebook, &process_tracker, &window);
    create_hardening_tab(&notebook, &process_tracker, &window);
    create_monitoring_tab(&notebook, &process_tracker, &window);
    create_audit_tab(&notebook, &process_tracker, &window);
    create_backup_tab(&notebook, &process_tracker, &window);
    create_tools_tab(&notebook, &process_tracker, &window);

    window.set_child(Some(&notebook));
    window.present();
}

fn create_dashboard_tab(notebook: &Notebook, process_tracker: &ProcessTracker, window: &ApplicationWindow) {
    let main_box = Box::new(gtk4::Orientation::Vertical, 10);
    main_box.set_margin_start(20);
    main_box.set_margin_end(20);
    main_box.set_margin_top(20);
    main_box.set_margin_bottom(20);
    
    let status_frame = Frame::new(Some("System Status Overview"));
    let status_grid = Grid::new();
    status_grid.set_row_spacing(10);
    status_grid.set_column_spacing(20);
    status_grid.set_margin_start(15);
    status_grid.set_margin_end(15);
    status_grid.set_margin_top(15);
    status_grid.set_margin_bottom(15);
    
    let services = vec![
        ("UFW Firewall", "🔥"),
        ("Fail2Ban IPS", "🚫"),
        ("Audit System", "📊"),
        ("AppArmor MAC", "🛡️"),
        ("ClamAV Antivirus", "🦠"),
        ("SSH Hardening", "🔐"),
    ];
    
    for (i, (service, icon)) in services.iter().enumerate() {
        let icon_label = Label::new(Some(icon));
        let service_label = Label::new(Some(service));
        let status_label = Label::new(Some("Checking..."));
        status_label.add_css_class("dim-label");
        
        status_grid.attach(&icon_label, 0, i as i32, 1, 1);
        status_grid.attach(&service_label, 1, i as i32, 1, 1);
        status_grid.attach(&status_label, 2, i as i32, 1, 1);
    }
    
    status_frame.set_child(Some(&status_grid));
    main_box.append(&status_frame);
    
    let actions_frame = Frame::new(Some("Quick Actions"));
    let actions_box = Box::new(gtk4::Orientation::Horizontal, 10);
    actions_box.set_margin_start(15);
    actions_box.set_margin_end(15);
    actions_box.set_margin_top(15);
    actions_box.set_margin_bottom(15);
    
    let status_btn = Button::with_label("🔍 System Status");
    let monitor_btn = Button::with_label("📊 Start Monitoring");
    let audit_btn = Button::with_label("🔍 Security Audit");
    
    status_btn.add_css_class("pill");
    monitor_btn.add_css_class("pill");
    audit_btn.add_css_class("pill");
    
    actions_box.append(&status_btn);
    actions_box.append(&monitor_btn);
    actions_box.append(&audit_btn);
    
    actions_frame.set_child(Some(&actions_box));
    main_box.append(&actions_frame);
    
    let info_frame = Frame::new(Some("System Information"));
    let info_output = create_output_area();
    info_frame.set_child(Some(&info_output));
    main_box.append(&info_frame);
    
    let info_clone = info_output.clone();
    let tracker_clone = process_tracker.clone();
    let window_weak = window.downgrade();
    status_btn.connect_clicked(move |btn| {
        if let Some(win) = window_weak.upgrade() {
            setup_command_handlers("hardn status", &info_clone, btn, &tracker_clone, &win);
        }
    });
    
    let info_clone2 = info_output.clone();
    let tracker_clone2 = process_tracker.clone();
    let window_weak2 = window.downgrade();
    monitor_btn.connect_clicked(move |btn| {
        if let Some(win) = window_weak2.upgrade() {
            setup_command_handlers("hardn monitor start", &info_clone2, btn, &tracker_clone2, &win);
        }
    });
    
    let info_clone3 = info_output.clone();
    let tracker_clone3 = process_tracker.clone();
    let window_weak3 = window.downgrade();
    audit_btn.connect_clicked(move |btn| {
        if let Some(win) = window_weak3.upgrade() {
            setup_command_handlers("hardn audit", &info_clone3, btn, &tracker_clone3, &win);
        }
    });
    
    notebook.append_page(&main_box, Some(&Label::new(Some("�� Dashboard"))));
}

fn create_hardening_tab(notebook: &Notebook, process_tracker: &ProcessTracker, window: &ApplicationWindow) {
    let main_box = Box::new(gtk4::Orientation::Horizontal, 10);
    main_box.set_margin_start(20);
    main_box.set_margin_end(20);
    main_box.set_margin_top(20);
    main_box.set_margin_bottom(20);
    
    let left_panel = Box::new(gtk4::Orientation::Vertical, 10);
    left_panel.set_width_request(300);
    
    let hardening_frame = Frame::new(Some("System Hardening"));
    let hardening_box = Box::new(gtk4::Orientation::Vertical, 5);
    hardening_box.set_margin_start(15);
    hardening_box.set_margin_end(15);
    hardening_box.set_margin_top(15);
    hardening_box.set_margin_bottom(15);
    
    let setup_btn = Button::with_label("⚙️ Complete Setup");
    setup_btn.set_height_request(50);
    setup_btn.add_css_class("suggested-action");
    
    let setup_ni_btn = Button::with_label("🤖 Non-Interactive Setup");
    setup_ni_btn.set_height_request(40);
    
    let separator = Separator::new(gtk4::Orientation::Horizontal);
    
    hardening_box.append(&setup_btn);
    hardening_box.append(&setup_ni_btn);
    hardening_box.append(&separator);
    
    let tools = vec![
        ("🔥 UFW Firewall", "sudo /usr/share/hardn/tools/ufw.sh"),
        ("🚫 Fail2Ban Setup", "sudo /usr/share/hardn/tools/fail2ban.sh"),
        ("🔐 SSH Hardening", "sudo /usr/share/hardn/tools/openssh.sh"),
        ("🛡️ AppArmor Setup", "sudo /usr/share/hardn/tools/apparmor.sh"),
        ("📦 System Updates", "sudo /usr/share/hardn/tools/update_system_packages.sh"),
        ("🧹 System Cleanup", "sudo /usr/share/hardn/tools/cleanup.sh"),
    ];
    
    let output_area = create_output_area();
    
    for (name, cmd) in tools {
        let btn = Button::with_label(name);
        btn.set_height_request(35);
        
        let output_clone = output_area.clone();
        let cmd_clone = cmd.to_string();
        let tracker_clone = process_tracker.clone();
        let window_weak = window.downgrade();
        btn.connect_clicked(move |btn| {
            if let Some(win) = window_weak.upgrade() {
                setup_command_handlers(&cmd_clone, &output_clone, btn, &tracker_clone, &win);
            }
        });
        
        hardening_box.append(&btn);
    }
    
    hardening_frame.set_child(Some(&hardening_box));
    left_panel.append(&hardening_frame);
    
    let output_clone = output_area.clone();
    let tracker_clone = process_tracker.clone();
    let window_weak = window.downgrade();
    setup_btn.connect_clicked(move |btn| {
        if let Some(win) = window_weak.upgrade() {
            setup_command_handlers("sudo hardn setup", &output_clone, btn, &tracker_clone, &win);
        }
    });
    
    let output_clone2 = output_area.clone();
    let tracker_clone2 = process_tracker.clone();
    let window_weak2 = window.downgrade();
    setup_ni_btn.connect_clicked(move |btn| {
        if let Some(win) = window_weak2.upgrade() {
            setup_command_handlers("sudo hardn setup --non-interactive", &output_clone2, btn, &tracker_clone2, &win);
        }
    });
    
    main_box.append(&left_panel);
    main_box.append(&output_area);
    
    notebook.append_page(&main_box, Some(&Label::new(Some("⚙️ Hardening"))));
}

fn create_monitoring_tab(notebook: &Notebook, process_tracker: &ProcessTracker, window: &ApplicationWindow) {
    let main_box = Box::new(gtk4::Orientation::Vertical, 10);
    main_box.set_margin_start(20);
    main_box.set_margin_end(20);
    main_box.set_margin_top(20);
    main_box.set_margin_bottom(20);
    
    let control_frame = Frame::new(Some("Monitoring Control"));
    let control_box = Box::new(gtk4::Orientation::Horizontal, 10);
    control_box.set_margin_start(15);
    control_box.set_margin_end(15);
    control_box.set_margin_top(15);
    control_box.set_margin_bottom(15);
    
    let start_btn = Button::with_label("▶️ Start Monitoring");
    start_btn.add_css_class("suggested-action");
    
    let stop_btn = Button::with_label("⏹️ Stop Monitoring");
    stop_btn.add_css_class("destructive-action");
    
    let status_indicator = Label::new(Some("Status: Stopped"));
    status_indicator.add_css_class("dim-label");
    
    control_box.append(&start_btn);
    control_box.append(&stop_btn);
    control_box.append(&status_indicator);
    
    control_frame.set_child(Some(&control_box));
    main_box.append(&control_frame);
    
    let output_frame = Frame::new(Some("Real-time Monitoring"));
    let output_area = create_output_area();
    output_frame.set_child(Some(&output_area));
    main_box.append(&output_frame);
    
    let output_clone = output_area.clone();
    let status_clone = status_indicator.clone();
    let tracker_clone = process_tracker.clone();
    let window_weak = window.downgrade();
    start_btn.connect_clicked(move |btn| {
        status_clone.set_text("Status: Starting...");
        if let Some(win) = window_weak.upgrade() {
            setup_command_handlers("sudo hardn monitor start", &output_clone, btn, &tracker_clone, &win);
        }
    });
    
    let output_clone2 = output_area.clone();
    let status_clone2 = status_indicator.clone();
    let tracker_clone2 = process_tracker.clone();
    stop_btn.connect_clicked(move |btn| {
        status_clone2.set_text("Status: Stopping...");
        run_command_in_output("hardn monitor stop", &output_clone2, btn, &tracker_clone2);
    });
    
    notebook.append_page(&main_box, Some(&Label::new(Some("📊 Monitoring"))));
}

fn create_audit_tab(notebook: &Notebook, process_tracker: &ProcessTracker, window: &ApplicationWindow) {
    let main_box = Box::new(gtk4::Orientation::Vertical, 10);
    main_box.set_margin_start(20);
    main_box.set_margin_end(20);
    main_box.set_margin_top(20);
    main_box.set_margin_bottom(20);
    
    let control_frame = Frame::new(Some("Security Audit"));
    let control_box = Box::new(gtk4::Orientation::Horizontal, 10);
    control_box.set_margin_start(15);
    control_box.set_margin_end(15);
    control_box.set_margin_top(15);
    control_box.set_margin_bottom(15);
    
    let audit_btn = Button::with_label("🔍 Run Security Audit");
    audit_btn.add_css_class("suggested-action");
    audit_btn.set_height_request(50);
    
    let progress = ProgressBar::new();
    progress.set_show_text(true);
    progress.set_text(Some("Ready to run audit"));
    
    control_box.append(&audit_btn);
    control_box.append(&progress);
    
    control_frame.set_child(Some(&control_box));
    main_box.append(&control_frame);
    
    let output_frame = Frame::new(Some("Audit Results"));
    let output_area = create_output_area();
    output_frame.set_child(Some(&output_area));
    main_box.append(&output_frame);
    
    let output_clone = output_area.clone();
    let tracker_clone = process_tracker.clone();
    audit_btn.connect_clicked(move |btn| {
        run_command_in_output("hardn audit", &output_clone, btn, &tracker_clone);
    });
    
    notebook.append_page(&main_box, Some(&Label::new(Some("🔍 Audit"))));
}

fn create_backup_tab(notebook: &Notebook, process_tracker: &ProcessTracker, window: &ApplicationWindow) {
    let main_box = Box::new(gtk4::Orientation::Vertical, 10);
    main_box.set_margin_start(20);
    main_box.set_margin_end(20);
    main_box.set_margin_top(20);
    main_box.set_margin_bottom(20);
    
    let control_frame = Frame::new(Some("Backup & Restore"));
    let control_grid = Grid::new();
    control_grid.set_row_spacing(10);
    control_grid.set_column_spacing(10);
    control_grid.set_margin_start(15);
    control_grid.set_margin_end(15);
    control_grid.set_margin_top(15);
    control_grid.set_margin_bottom(15);
    
    let backup_btn = Button::with_label("💾 Create Backup");
    backup_btn.add_css_class("suggested-action");
    backup_btn.set_height_request(50);
    
    let restore_btn = Button::with_label("♻️ Restore from Backup");
    restore_btn.set_height_request(50);
    
    control_grid.attach(&backup_btn, 0, 0, 1, 1);
    control_grid.attach(&restore_btn, 1, 0, 1, 1);
    
    control_frame.set_child(Some(&control_grid));
    main_box.append(&control_frame);
    
    let output_frame = Frame::new(Some("Backup Operations"));
    let output_area = create_output_area();
    output_frame.set_child(Some(&output_area));
    main_box.append(&output_frame);
    
    let output_clone = output_area.clone();
    let tracker_clone = process_tracker.clone();
    backup_btn.connect_clicked(move |btn| {
        run_command_in_output("hardn backup", &output_clone, btn, &tracker_clone);
    });
    
    let output_clone2 = output_area.clone();
    let tracker_clone2 = process_tracker.clone();
    restore_btn.connect_clicked(move |btn| {
        run_command_in_output("hardn restore", &output_clone2, btn, &tracker_clone2);
    });
    
    notebook.append_page(&main_box, Some(&Label::new(Some("💾 Backup"))));
}

fn create_tools_tab(notebook: &Notebook, process_tracker: &ProcessTracker, window: &ApplicationWindow) {
    let main_box = Box::new(gtk4::Orientation::Horizontal, 10);
    main_box.set_margin_start(20);
    main_box.set_margin_end(20);
    main_box.set_margin_top(20);
    main_box.set_margin_bottom(20);
    
    let tools_frame = Frame::new(Some("Additional Tools"));
    let tools_box = Box::new(gtk4::Orientation::Vertical, 5);
    tools_box.set_margin_start(15);
    tools_box.set_margin_end(15);
    tools_box.set_margin_top(15);
    tools_box.set_margin_bottom(15);
    tools_box.set_width_request(250);
    
    let output_area = create_output_area();
    
    let tools = vec![
        ("🔄 System Update", "hardn update"),
        ("🔧 Lynis Audit", "sudo /usr/share/hardn/tools/lynis.sh"),
        ("🔍 RKHunter Setup", "sudo /usr/share/hardn/tools/rkhunter.sh"),
        ("🌐 API Server", "hardn api --port 8080"),
        ("❌ Uninstall HARDN", "hardn uninstall"),
    ];
    
    for (name, cmd) in tools {
        let btn = Button::with_label(name);
        btn.set_height_request(40);
        
        let output_clone = output_area.clone();
        let cmd_clone = cmd.to_string();
        let tracker_clone = process_tracker.clone();
        btn.connect_clicked(move |btn| {
            if cmd_clone.starts_with("sudo") {
                show_sudo_message(&output_clone, &cmd_clone, &tracker_clone);
            } else {
                run_command_in_output(&cmd_clone, &output_clone, btn, &tracker_clone);
            }
        });
        
        tools_box.append(&btn);
    }
    
    tools_frame.set_child(Some(&tools_box));
    main_box.append(&tools_frame);
    main_box.append(&output_area);
    
    notebook.append_page(&main_box, Some(&Label::new(Some("🔧 Tools"))));
}

fn create_output_area() -> ScrolledWindow {
    let scrolled = ScrolledWindow::builder()
        .hscrollbar_policy(gtk4::PolicyType::Automatic)
        .vscrollbar_policy(gtk4::PolicyType::Automatic)
        .build();

    let text_view = TextView::builder()
        .editable(false)
        .monospace(true)
        .wrap_mode(gtk4::WrapMode::Word)
        .build();
    
    text_view.set_margin_start(10);
    text_view.set_margin_end(10);
    text_view.set_margin_top(10);
    text_view.set_margin_bottom(10);
    
    text_view.buffer().set_text("Ready to execute commands...\n\n");
    
    scrolled.set_child(Some(&text_view));
    scrolled
}

fn setup_command_handlers(command: &str, output_area: &ScrolledWindow, button: &Button, process_tracker: &ProcessTracker, window: &ApplicationWindow) {
    // Check if command requires sudo
    if command.starts_with("sudo") || command.contains("sudo ") {
        // Show password dialog for sudo commands
        let dialog = Dialog::builder()
            .title("Administrator Password Required")
            .modal(true)
            .transient_for(window)
            .build();
        
        dialog.add_button("Cancel", ResponseType::Cancel);
        dialog.add_button("Execute", ResponseType::Accept);
        
        let content_area = dialog.content_area();
        let vbox = Box::new(gtk4::Orientation::Vertical, 10);
        vbox.set_margin_start(20);
        vbox.set_margin_end(20);
        vbox.set_margin_top(20);
        vbox.set_margin_bottom(20);
        
        let label = Label::new(Some(&format!("This command requires administrator privileges:\n{}", command)));
        label.set_wrap(true);
        
        let password_entry = Entry::builder()
            .placeholder_text("Enter your password")
            .visibility(false)
            .activates_default(true)
            .build();
        
        vbox.append(&label);
        vbox.append(&password_entry);
        content_area.append(&vbox);
        
        dialog.set_default_response(ResponseType::Accept);
        password_entry.grab_focus();
        
        let command_clone = command.to_string();
        let output_area_clone = output_area.clone();
        let button_clone = button.clone();
        let process_tracker_clone = process_tracker.clone();
        
        dialog.connect_response(move |dialog, response| {
            if response == ResponseType::Accept {
                let password = password_entry.text();
                if !password.is_empty() {
                    execute_command_with_password(&command_clone, &password, &output_area_clone, &button_clone, &process_tracker_clone);
                }
            }
            dialog.close();
        });
        
        dialog.present();
    } else {
        // Regular command without sudo
        run_command_in_output(command, output_area, button, process_tracker);
    }
}

fn execute_command_with_password(command: &str, password: &str, output_area: &ScrolledWindow, button: &Button, process_tracker: &ProcessTracker) {
    let text_view = output_area.child().unwrap().downcast::<TextView>().unwrap();
    let buffer = text_view.buffer();
    
    buffer.set_text(&format!("Running: {}\n", command));
    buffer.insert_at_cursor("=".repeat(60).as_str());
    buffer.insert_at_cursor("\n");
    
    button.set_sensitive(false);
    let original_label = button.label().unwrap_or_default();
    button.set_label("Running...");
    
    let (tx, rx) = mpsc::channel();
    let cmd_string = command.to_string();
    let password_string = password.to_string();
    let button_clone = button.clone();
    let original_label_clone = original_label.clone();
    let process_tracker_clone = process_tracker.clone();
    
    thread::spawn(move || {
        execute_sudo_command_streaming(&cmd_string, &password_string, tx, &process_tracker_clone);
    });
    
    let output_area_clone = output_area.clone();
    glib::timeout_add_local(std::time::Duration::from_millis(100), move || {
        while let Ok(message) = rx.try_recv() {
            if message == "DONE" {
                button_clone.set_sensitive(true);
                button_clone.set_label(&original_label_clone);
                return glib::ControlFlow::Break;
            } else {
                let text_view = output_area_clone.child().unwrap().downcast::<TextView>().unwrap();
                let buffer = text_view.buffer();
                buffer.insert_at_cursor(&message);
                
                let mut end_iter = buffer.end_iter();
                text_view.scroll_to_iter(&mut end_iter, 0.0, false, 0.0, 0.0);
            }
        }
        glib::ControlFlow::Continue
    });
}

fn run_command_in_output(command: &str, output_area: &ScrolledWindow, button: &Button, process_tracker: &ProcessTracker) {
    let text_view = output_area.child().unwrap().downcast::<TextView>().unwrap();
    let buffer = text_view.buffer();
    
    buffer.set_text(&format!("Running: {}\n", command));
    buffer.insert_at_cursor("=".repeat(60).as_str());
    buffer.insert_at_cursor("\n");
    
    if command.starts_with("sudo") {
        buffer.insert_at_cursor("⚠️ This command requires sudo privileges. Use the password dialog instead.\n");
        return;
    }
    
    if command.contains("monitor start") {
        buffer.insert_at_cursor("🔍 Starting HARDN monitoring services...\n");
        buffer.insert_at_cursor("This will start real-time monitoring of your system security.\n\n");
    } else if command.contains("monitor stop") {
        buffer.insert_at_cursor("⏹️  Stopping HARDN monitoring services...\n\n");
    } else if command.contains("audit") {
        buffer.insert_at_cursor("🔍 Running comprehensive security audit...\n");
        buffer.insert_at_cursor("This may take several minutes to complete.\n\n");
    } else if command.contains("backup") {
        buffer.insert_at_cursor("💾 Creating system configuration backup...\n\n");
    }
    
    button.set_sensitive(false);
    let original_label = button.label().unwrap_or_default();
    button.set_label("Running...");
    
    let (tx, rx) = mpsc::channel();
    let cmd_string = command.to_string();
    let button_clone = button.clone();
    let original_label_clone = original_label.clone();
    let process_tracker_clone = process_tracker.clone();
    
    thread::spawn(move || {
        execute_command_streaming(&cmd_string, tx, &process_tracker_clone);
    });
    
    let output_area_clone = output_area.clone();
    glib::timeout_add_local(std::time::Duration::from_millis(100), move || {
        while let Ok(message) = rx.try_recv() {
            if message == "DONE" {
                button_clone.set_sensitive(true);
                button_clone.set_label(&original_label_clone);
                return glib::ControlFlow::Break;
            } else {
                let text_view = output_area_clone.child().unwrap().downcast::<TextView>().unwrap();
                let buffer = text_view.buffer();
                buffer.insert_at_cursor(&message);
                
                let mut end_iter = buffer.end_iter();
                text_view.scroll_to_iter(&mut end_iter, 0.0, false, 0.0, 0.0);
            }
        }
        glib::ControlFlow::Continue
    });
}

fn execute_command_streaming(command: &str, tx: mpsc::Sender<String>, process_tracker: &ProcessTracker) {
    let parts: Vec<&str> = command.split_whitespace().collect();
        if parts.is_empty() {
        let _ = tx.send("Error: Empty command\n".to_string());
        let _ = tx.send("DONE".to_string());
            return;
        }
        
        let mut cmd = Command::new(parts[0]);
        if parts.len() > 1 {
            cmd.args(&parts[1..]);
        }
        
    cmd.env("DEBIAN_FRONTEND", "noninteractive");
    cmd.env("NEEDRESTART_MODE", "a");
    
    let child = cmd
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
        .stdin(Stdio::null())
        .spawn();
        
    let mut child = match child {
            Ok(child) => child,
            Err(e) => {
            let _ = tx.send(format!("❌ Error starting command: {}\n", e));
            let _ = tx.send("DONE".to_string());
                return;
            }
        };
        
    // Track the process for cleanup
    let pid = child.id();
    if let Ok(mut pids) = process_tracker.lock() {
        pids.insert(pid);
    }
    
        if let Some(stdout) = child.stdout.take() {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            let reader = BufReader::new(stdout);
                for line in reader.lines() {
                if let Ok(line) = line {
                    if line.trim().is_empty() {
                        continue;
                    }
                    if line.contains("[INFO]") {
                        let _ = tx_clone.send(format!("ℹ️  {}\n", line.replace("[INFO]", "").trim()));
                    } else if line.contains("[WARN]") {
                        let _ = tx_clone.send(format!("⚠️  {}\n", line.replace("[WARN]", "").trim()));
                    } else if line.contains("[ERROR]") {
                        let _ = tx_clone.send(format!("❌ {}\n", line.replace("[ERROR]", "").trim()));
                    } else {
                        let _ = tx_clone.send(format!("{}\n", line));
                    }
                    }
                }
            });
        }
        
        if let Some(stderr) = child.stderr.take() {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            let reader = BufReader::new(stderr);
                for line in reader.lines() {
                if let Ok(line) = line {
                    if line.trim().is_empty() {
                        continue;
                    }
                    if line.contains("error") || line.contains("Error") || line.contains("ERROR") {
                        let _ = tx_clone.send(format!("❌ {}\n", line));
                    } else if line.contains("[INFO]") {
                        let _ = tx_clone.send(format!("ℹ️  {}\n", line.replace("[INFO]", "").trim()));
                            } else {
                        let _ = tx_clone.send(format!("⚠️  {}\n", line));
                    }
                    }
                }
            });
        }
        
        match child.wait() {
            Ok(status) => {
            println!("Process {} finished", pid);
                if status.success() {
                let _ = tx.send("\n✅ Command completed successfully\n".to_string());
                } else {
                let _ = tx.send(format!("\n❌ Command failed with exit code: {:?}\n", status.code()));
            }
        }
        Err(e) => {
            let _ = tx.send(format!("\n❌ Error waiting for command: {}\n", e));
        }
    }
    
    let _ = tx.send("DONE".to_string());
}

fn execute_sudo_command_streaming(command: &str, password: &str, tx: mpsc::Sender<String>, process_tracker: &ProcessTracker) {
    // Use echo to pipe password to sudo command
    let full_command = format!("echo '{}' | {}", password, command);
    
    let mut cmd = Command::new("bash");
    cmd.args(["-c", &full_command]);
    cmd.env("DEBIAN_FRONTEND", "noninteractive");
    cmd.env("NEEDRESTART_MODE", "a");
    
    let child = cmd
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .stdin(Stdio::null())
        .spawn();
        
    let mut child = match child {
        Ok(child) => child,
        Err(e) => {
            let _ = tx.send(format!("❌ Error starting sudo command: {}\n", e));
            let _ = tx.send("DONE".to_string());
            return;
        }
    };
    
    // Track the process for cleanup
    let pid = child.id();
    if let Ok(mut pids) = process_tracker.lock() {
        pids.insert(pid);
    }
    
    if let Some(stdout) = child.stdout.take() {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            let reader = BufReader::new(stdout);
            for line in reader.lines() {
                if let Ok(line) = line {
                    if line.trim().is_empty() {
                        continue;
                    }
                    if line.contains("[INFO]") {
                        let _ = tx_clone.send(format!("ℹ️  {}\n", line.replace("[INFO]", "").trim()));
                    } else if line.contains("[WARN]") {
                        let _ = tx_clone.send(format!("⚠️  {}\n", line.replace("[WARN]", "").trim()));
                    } else if line.contains("[ERROR]") {
                        let _ = tx_clone.send(format!("❌ {}\n", line.replace("[ERROR]", "").trim()));
                    } else {
                        let _ = tx_clone.send(format!("{}\n", line));
                    }
                }
            }
        });
    }
    
    if let Some(stderr) = child.stderr.take() {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            let reader = BufReader::new(stderr);
            for line in reader.lines() {
                if let Ok(line) = line {
                    if line.trim().is_empty() {
                        continue;
                    }
                    if line.contains("error") || line.contains("Error") || line.contains("ERROR") {
                        let _ = tx_clone.send(format!("❌ {}\n", line));
                    } else if line.contains("[INFO]") {
                        let _ = tx_clone.send(format!("ℹ️  {}\n", line.replace("[INFO]", "").trim()));
                    } else {
                        let _ = tx_clone.send(format!("⚠️  {}\n", line));
                    }
                }
            }
        });
    }
    
    match child.wait() {
        Ok(status) => {
            println!("Sudo process {} finished", pid);
            if status.success() {
                let _ = tx.send("\n✅ Sudo command completed successfully\n".to_string());
            } else {
                let _ = tx.send(format!("\n❌ Sudo command failed with exit code: {:?}\n", status.code()));
            }
        }
        Err(e) => {
            let _ = tx.send(format!("\n❌ Error waiting for sudo command: {}\n", e));
        }
    }
    
    let _ = tx.send("DONE".to_string());
}