//VAPT (visual apt).
use iced::widget::{button, column, row, text, rule, text_input, scrollable};
use std::process::{Command};
use iced::{Element, Alignment, Fill, Size, Theme, Center, Color};
pub fn main() -> iced::Result {
    iced::application(Installer::default, Installer::update, Installer::view)
        .title(Installer::title)
        // We set the window config here.
        .theme(Theme::CatppuccinMacchiato)
        .window(iced::window::Settings {
            size: Size::new(500.0, 500.0),
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
enum Cmd { // was named command, but had to be changed because of a conflict with the command module in std::process.
    Remove,
    Install,
    PackageChange(String),
    PurgeUnused,
    Search,
    Update,
    Upgrade,
    List,
    Desc,
}

impl Installer {

    fn title(_state: &Self) -> String {
        String::from("VAPT")
    }


    fn update(&mut self, message: Cmd) { // we are using our enum as a sort of control center and outputting the results of the functions into the log.
        match message {
            Cmd::Install => self.log = install(self.package.clone()),
            Cmd::Remove => self.log = remove(self.package.clone()),
            Cmd::PackageChange(new_package) => self.package = new_package,
            Cmd::PurgeUnused => self.log = purge(),
            Cmd::Search => self.log = search(self.package.clone()),
            Cmd::Update => self.log = update(),
            Cmd::Upgrade => self.log = upgrade(),
            Cmd::List => self.log = list(),
            Cmd::Desc => self.log = describe_package(self.package.clone()),
        }
    }

    fn view(&self) -> Element<'_, Cmd> { //the view function is essentially the layout.
        let installer_buttons = row![
            button("+").on_press(Cmd::Install),
            button("-").on_press(Cmd::Remove),
            button("?").on_press(Cmd::Search),
            button("Desc").on_press(Cmd::Desc)
        ].spacing(10)
        .align_y(Center);
        let divider = rule::horizontal(2.0).style(|_theme: &iced::Theme| rule::Style {
            color: Color::WHITE,
            fill_mode: rule::FillMode::Full, // Fills the line completely across layout boundaries
            radius: 0.0.into(),             // Sharp corners for the rule line
            snap: true,                     // Snaps line position cleanly onto pixel grids
        });
        let system_buttons = row![
        button("Purge").on_press(Cmd::PurgeUnused),
        button("Upgrade").on_press(Cmd::Upgrade),
        button("Update").on_press(Cmd::Update),
        button("PKG Log").on_press(Cmd::List),
        ].spacing(10).align_y(Center);

        scrollable(column![
        text_input("", &self.package).on_input(Cmd::PackageChange),
        installer_buttons,
        divider,
        text("System"),
        system_buttons,
        text(format!("Output Log: {}", self.log)),
        ].spacing(20)
        .width(Fill)                       // Forces the column to span full width
        .align_x(Alignment::Center))
        .height(Fill)
        .into()
    }
}

fn install(package: String) -> String //we're running apt install and telling the user if the install was successful.
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
        "Package installed successfully!".to_string()
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        
        // Check the trace to confirm it's a missing package error
        if stderr.contains("Unable to locate package") {
            return "Error: The package does not exist.".to_string()
        }
        format!("APT failed with exit code: {:?}", output.status.code())
    }
}
fn remove(package: String) -> String // we're essentially just running apt remove. I didn't include purge as there is already a separate button. 
// we are not purging automatically because it could break dependencies for the user.
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
        "Package removed successfully!".to_string()
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        
        // Check the trace to confirm it's a missing package error
        if stderr.contains("Unable to locate package") {
            return "Error: The package does not exist.".to_string()
        }
        format!("APT failed with exit code: {:?}", output.status.code())
    }
}
fn purge() -> String // we're running apt autoremove --purge -y to remove unused packages
{
    println!("purging unused packages");
    Command::new("sudo")
        .arg("apt")
        .arg("autoremove")
        .arg("--purge")
        .arg("-y")
        .spawn()
        .expect("Failed to execute apt command");
    "Purging unused packages in the background.".to_string()
}

fn search(package: String) -> String {
    println!("searching for {}", package.clone());
    let output = Command::new("apt-cache")
        .arg("search")
        .arg("--names-only")
        .arg(package)
        .output()
        .expect("Failed to execute apt command");
    println!("searching for packages has completed");
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let string_stdout = stdout.to_string();
        let mut package_arr = string_stdout.split("\n").map(|pack| pack.split(" - ").next().unwrap_or("").trim()).collect::<Vec<&str>>();
        package_arr.sort_by_key(|s| s.len());
        let package_list = package_arr.join("\n");
        format!("Search results:\n{}", package_list)
    } else {
        format!("APT failed with exit code: {:?}", output.status.code())
    }
}

fn upgrade() -> String {
    println!("upgrading system");
    let output = Command::new("sudo")
        .arg("apt")
        .arg("upgrade")
        .arg("-y")
        .output()
        .expect("Failed to execute apt command");
    if output.status.success() {
        "System upgraded successfully!".to_string()
    } else {
        format!("APT failed with exit code: {:?}", output.status.code())
    }
}

fn update() -> String {
    println!("updating system");
    let output = Command::new("sudo")
        .arg("apt")
        .arg("update")
        .output()
        .expect("Failed to execute apt command");
    if output.status.success() {
        "System updated successfully!".to_string()
    } else {
        format!("APT failed with exit code: {:?}", output.status.code())
    }
}

fn list() -> String {
    println!("generating package list");
    let output = Command::new("apt")
        .arg("list")
        .arg("--installed")
        .output()
        .expect("Failed to execute apt command");
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let string_stdout = stdout.to_string();
        let package_count = string_stdout.as_bytes().iter().filter(|&&c| c == b'\n').count();
        std::fs::write("installed_packages.txt", string_stdout).expect("Unable to write file");
        format!("Package list generated successfully! Check installed_packages.txt in the current directory. Total packages: {}", package_count)
    } else {
        format!("apt failed with exit code: {:?}", output.status.code())
    }
}

fn describe_package(package: String) -> String {
    println!("describing {}", package.clone());
    let output = Command::new("apt-cache")
        .arg("search")
        .arg("--names-only")
        .arg(format!("^{}$", package.clone()))
        .output()
        .expect("Failed to execute apt command");
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let string_stdout = stdout.to_string();
        return format!("{string_stdout}")
    }
    else {
        return format!("apt failed with exit code: {:?}", output.status.code())
    }

}
