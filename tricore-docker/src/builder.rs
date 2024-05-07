use std::collections::HashSet;
use std::process::{Command, Stdio};

use anyhow::Context;

use super::pipe::Pipe;

#[derive(Default)]
pub struct DockerBuilder<'a, 'pipe> {
    display_env_variable: Option<&'a str>,
    name: Option<&'a str>,
    image_name: Option<&'a str>,
    pipes_as_args: Vec<(&'a str, &'pipe Pipe)>,
}

impl<'a, 'pipe> DockerBuilder<'a, 'pipe> {
    pub fn new() -> Self {
        DockerBuilder::default()
    }

    pub fn connect_display(self, display_env_variable: &'a str) -> Self {
        DockerBuilder {
            display_env_variable: Some(display_env_variable),
            ..self
        }
    }

    pub fn named(self, name: &'a str) -> Self {
        DockerBuilder {
            name: Some(name),
            ..self
        }
    }

    pub fn image_name(self, name: &'a str) -> Self {
        DockerBuilder {
            image_name: Some(name),
            ..self
        }
    }

    pub fn add_pipe_as_argument(self, argument_name: &'a str, pipe: &'pipe Pipe) -> Self {
        let mut pipes_as_args = self.pipes_as_args;
        pipes_as_args.push((argument_name, pipe));

        DockerBuilder {
            pipes_as_args,
            ..self
        }
    }

    pub fn build(self) -> DockerInstance<Created<'a>> {
        let mut enumerator = udev::Enumerator::new().unwrap();
        enumerator.match_subsystem("usb").unwrap();
        enumerator.match_property("ID_VENDOR_FROM_DATABASE", "Infineon Technologies").unwrap();
        let mut devices: HashSet<String> = HashSet::new();
        for device in enumerator.scan_devices().unwrap() {
            if let Some(dev_node) = device.devnode(){
                devices.insert(dev_node.to_str().unwrap().to_owned());
            }
        }

        let mut dev_path_param = Vec::new();
        for dev_path in devices{
            println!("--device {}:{}", dev_path, dev_path);
            dev_path_param.push("--device".to_string());
            dev_path_param.push(format!("{}:{}", dev_path, dev_path));
        }

        let mut docker_command = Command::new("docker");
        let command = docker_command
            .stderr(Stdio::inherit())
            .stdout(Stdio::inherit())
            .args(["run", "--init",])
            .args(dev_path_param);

        let mut daemon_command = "RUST_LOG=trace xvfb-run wine64 win-daemon.exe".to_owned();

        if let Some(display_env_variable) = self.display_env_variable {
            command
                .arg("--env")
                .arg(format!("DISPLAY={display_env_variable}"));

            daemon_command = "RUST_LOG=trace wine64 win-daemon.exe".to_owned();
        }

        if let Some(name) = &self.name {
            command.arg("--name").arg(name);
        }

        for (idx, (arg_name, pipe)) in self.pipes_as_args.iter().enumerate() {
            let folder_mount_name = format!("pipe_{}", idx);
            command.arg("--mount").arg(format!(
                "type=bind,source={},target=/root/.wine/drive_c/users/root/{}",
                pipe.directory().display(),
                folder_mount_name
            ));

            daemon_command += &format!(
                r#" --{arg_name} C:/users/root/{}/{}"#,
                folder_mount_name,
                pipe.name()
            );
        }

        let image_name = self.image_name.expect("image name not given");

        command
            .arg(image_name)
            .args(["bash", "-c"])
            .arg(daemon_command);

        DockerInstance {
            state: Created {
                image_name: self.name,
                command: docker_command,
            },
        }
    }
}

pub struct Created<'a> {
    command: Command,
    image_name: Option<&'a str>,
}

pub struct Spawned;

pub struct DockerInstance<S> {
    state: S,
}

impl<'a> DockerInstance<Created<'a>> {
    pub fn spawn(mut self) -> anyhow::Result<DockerInstance<Spawned>> {
        log::debug!("Spawning docker with command {:?}", self.state.command);
        if let Some(image_name) = self.state.image_name {
            kill_docker_with_name(image_name)
                .with_context(|| "Could not remove previous docker instance")?;
        }
        let mut child = self
            .state
            .command
            .spawn()
            .with_context(|| "Could not spawn docker instance")?;

        std::thread::spawn(move || {
            let exit_status = child.wait().expect("Docker did not execute properly");
            assert!(exit_status.success(), "docker command failed!");
        });

        Ok(DockerInstance { state: Spawned })
    }
}

fn kill_docker_with_name(name: &str) -> anyhow::Result<()> {
    log::warn!("Removing/Killing docker container \"{}\"", name);
    // Kill potentially existing container before spawning (ingore output because we don't care)
    Command::new("docker")
        .arg("container")
        .arg("kill")
        .arg(name)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .with_context(|| "")?
        .wait()
        .with_context(|| "")?;

    Command::new("docker")
        .arg("container")
        .arg("rm")
        .arg(name)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .with_context(|| "")?
        .wait()
        .with_context(|| "")?;

    Ok(())
}
