use std::{process::Stdio, time::Duration};
use tokio::{process::Command, time::timeout};

#[cfg(target_os = "linux")]
fn host_browser_command(
    environment: impl IntoIterator<Item = (std::ffi::OsString, std::ffi::OsString)>,
) -> Command {
    let environment: std::collections::HashMap<_, _> = environment.into_iter().collect();
    let appdir = environment.get(std::ffi::OsStr::new("APPDIR"));
    let mut command = Command::new("/usr/bin/xdg-open");
    command.env_clear().envs(&environment);
    if appdir.is_some() {
        // AppImage libraries belong to Coplayer. Host launchers/Flatpak must use
        // the system libraries, while retaining the desktop session and portal.
        for key in [
            "LD_LIBRARY_PATH",
            "LD_PRELOAD",
            "GTK_PATH",
            "GTK_EXE_PREFIX",
            "GTK_DATA_PREFIX",
            "GIO_MODULE_DIR",
            "GIO_EXTRA_MODULES",
            "GDK_PIXBUF_MODULE_FILE",
            "GDK_PIXBUF_MODULEDIR",
            "GSETTINGS_SCHEMA_DIR",
            "QT_PLUGIN_PATH",
            "QT_QPA_PLATFORM_PLUGIN_PATH",
        ] {
            command.env_remove(key);
        }
        if let Some(appdir) = appdir.filter(|value| !value.is_empty()) {
            for key in ["PATH", "XDG_DATA_DIRS"] {
                if let Some(value) = environment.get(std::ffi::OsStr::new(key)) {
                    let paths = std::env::split_paths(value)
                        .filter(|path| !path.starts_with(std::path::Path::new(appdir)));
                    if let Ok(value) = std::env::join_paths(paths) {
                        command.env(key, value);
                    }
                }
            }
        }
    }
    command
}

pub async fn open_browser(url: &str) -> bool {
    #[cfg(target_os = "linux")]
    let mut command = host_browser_command(std::env::vars_os());
    #[cfg(target_os = "macos")]
    let mut command = Command::new("/usr/bin/open");
    #[cfg(not(any(target_os = "linux", target_os = "macos")))]
    return false;
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    {
        command
            .arg(url)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .kill_on_drop(true);
        timeout(Duration::from_secs(10), command.status())
            .await
            .is_ok_and(|result| result.is_ok_and(|status| status.success()))
    }
}

#[cfg(all(test, target_os = "linux"))]
mod tests {
    use super::*;
    use std::{collections::HashMap, ffi::OsString};

    #[tokio::test]
    async fn appimage_browser_uses_host_libraries_and_preserves_desktop_session() {
        let environment = [
            ("APPDIR", "/tmp/Coplayer Test.AppDir"),
            ("LD_LIBRARY_PATH", "/tmp/Coplayer Test.AppDir/usr/lib"),
            ("LD_PRELOAD", "/tmp/Coplayer Test.AppDir/preload.so"),
            ("GIO_MODULE_DIR", "/tmp/Coplayer Test.AppDir/usr/lib/gio"),
            ("GTK_PATH", "/tmp/Coplayer Test.AppDir/usr/lib/gtk"),
            ("QT_PLUGIN_PATH", "/tmp/Coplayer Test.AppDir/usr/lib/qt"),
            ("PATH", "/tmp/Coplayer Test.AppDir/usr/bin:/usr/bin:/bin"),
            (
                "XDG_DATA_DIRS",
                "/tmp/Coplayer Test.AppDir/usr/share:/usr/share:/var/lib/flatpak/exports/share",
            ),
            ("DBUS_SESSION_BUS_ADDRESS", "unix:path=/run/user/1000/bus"),
            ("WAYLAND_DISPLAY", "wayland-0"),
            ("DISPLAY", ":0"),
            ("XDG_RUNTIME_DIR", "/run/user/1000"),
            ("HOME", "/home/test"),
        ];
        let command = host_browser_command(
            environment.map(|(key, value)| (OsString::from(key), OsString::from(value))),
        );
        // Exercise inheritance in a real child without opening a browser or
        // passing an authentication URL to the test process.
        let mut probe = Command::new("/usr/bin/env");
        probe.env_clear();
        for (key, value) in command.as_std().get_envs() {
            if let Some(value) = value {
                probe.env(key, value);
            }
        }
        let output = probe.output().await.unwrap();
        assert!(output.status.success());
        let text = String::from_utf8(output.stdout).unwrap();
        let actual: HashMap<_, _> = text
            .lines()
            .filter_map(|line| line.split_once('='))
            .collect();
        for key in [
            "LD_LIBRARY_PATH",
            "LD_PRELOAD",
            "GIO_MODULE_DIR",
            "GTK_PATH",
            "QT_PLUGIN_PATH",
        ] {
            assert!(!actual.contains_key(key), "{key} leaked into host launcher");
        }
        assert_eq!(actual["PATH"], "/usr/bin:/bin");
        assert_eq!(
            actual["XDG_DATA_DIRS"],
            "/usr/share:/var/lib/flatpak/exports/share"
        );
        for (key, value) in environment.into_iter().skip(8) {
            assert_eq!(actual[key], value);
        }
    }

    #[test]
    fn unpackaged_browser_preserves_custom_environment() {
        let command = host_browser_command([
            (
                OsString::from("LD_LIBRARY_PATH"),
                OsString::from("/opt/custom/lib"),
            ),
            (
                OsString::from("PATH"),
                OsString::from("/opt/custom/bin:/usr/bin"),
            ),
        ]);
        let actual: HashMap<_, _> = command.as_std().get_envs().collect();
        assert_eq!(
            actual[std::ffi::OsStr::new("LD_LIBRARY_PATH")],
            Some(std::ffi::OsStr::new("/opt/custom/lib"))
        );
        assert_eq!(
            actual[std::ffi::OsStr::new("PATH")],
            Some(std::ffi::OsStr::new("/opt/custom/bin:/usr/bin"))
        );
    }
}
