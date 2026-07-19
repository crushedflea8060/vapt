//VAPT (visual apt).
use iced::widget::{button, column, text,  text_input, center_x};
use std::process::Command;
use iced::{Element, Alignment, Fill, Size};
pub fn main() -> iced::Result {
    iced::application(Installer::default, Installer::update, Installer::view)
        .title(Installer::title)
        // We set the window config here.
        .window(iced::window::Settings {
            size: Size::new(250.0, 350.0),
            ..Default::default()
        })
        
        .run()
}

#[derive(Default)]
struct Installer {
    package: String,
    log: String,
}

#[derive(Debug, Clone)]
enum CMD {
    Remove,
    Install,
    PackageChange(String),
    PurgeUnused,
}

impl Installer {

    fn title(_state: &Self) -> String {
        String::from("VAPT")
    }


    fn update(&mut self, message: CMD) {
        match message {
            CMD::Install => self.log = install(self.package.clone()),
            CMD::Remove => self.log = remove(self.package.clone()),
            CMD::PackageChange(new_package) => self.package = new_package,
            CMD::PurgeUnused => self.log = purge(),
        }
    }

    fn view(&self) -> Element<'_, CMD> {
        center_x(column![
            text("Search for Debian packages at https://packages.debian.org/nl/"),
            text(format!("Package Name: {}", &self.package)),
            text_input("", &self.package).on_input(CMD::PackageChange),
            button("Install").on_press(CMD::Install),
            button("Remove").on_press(CMD::Remove),
            button("Purge Unused Dependencies").on_press(CMD::PurgeUnused),
            text(format!("Install Log: {}", &self.log)),
        ].spacing(20)
            .width(Fill)                       // Forces the column to span full width
            .align_x(Alignment::Center))
            .into()
    }
}

fn install(package: String) -> String
{
    println!("installing {}", package.clone());
    let output = Command::new("sudo")
        .arg("apt")
        .arg("install")
        .arg("-y")
        .arg(package)
        .output()
        .expect("Failed to execute apt command");
    if output.status.success() {
        return format!("Package installed successfully!")
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        
        // Check the trace to confirm it's a missing package error
        if stderr.contains("Unable to locate package") {
            return format!("Error: The package does not exist.")
        }
        return format!("APT failed with exit code: {:?}", output.status.code())
    }
}
fn remove(package: String) -> String
{
    println!("removing {}", package.clone());
    let output = Command::new("sudo")
        .arg("apt")
        .arg("remove")
        .arg("-y")
        .arg(package)
        .output()
        .expect("Failed to execute apt command");
    if output.status.success() {
        return format!("Package removed successfully!")
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        
        // Check the trace to confirm it's a missing package error
        if stderr.contains("Unable to locate package") {
            return format!("Error: The package does not exist.")
        }
        return format!("APT failed with exit code: {:?}", output.status.code())
    }
}
fn purge() -> String
{
    println!("purging unused packages");
    let output = Command::new("sudo")
        .arg("apt")
        .arg("autoremove")
        .arg("--purge")
        .arg("-y")
        .output()
        .expect("Failed to execute apt command");
    if output.status.success() {
        return format!("Packages purged successfully!")
    } else {
        return format!("APT failed with exit code: {:?}", output.status.code())
    }
}
