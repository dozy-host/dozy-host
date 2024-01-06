use std::{str::FromStr, error::Error};

use bollard::service::{EventMessage, EventMessageTypeEnum, EventActor};

pub struct DockerEvent {
    pub event_type: DockerEventType,
    pub actor: EventActor,
}

impl TryFrom<EventMessage> for DockerEvent {
    type Error = DockerEventParseError;

    fn try_from(value: EventMessage) -> Result<Self, Self::Error> {
        let event_type = DockerEventType::try_from(&value)?;
        let actor = value.actor.ok_or(DockerEventParseError::MissingMessageActor)?;

        Ok(Self {
            event_type,
            actor,
        })
    }
}

pub enum DockerEventType {
    Container(ContainerEvent),
    Image(ImageEvent),
    Network(NetworkEvent),
    Volume(VolumeEvent),
    Plugin(PluginEvent),
    Daemon(DaemonEvent),
}

impl TryFrom<&EventMessage> for DockerEventType {
    type Error = DockerEventParseError;

    fn try_from(value: &EventMessage) -> Result<Self, DockerEventParseError> {
        let message_action = value
            .action
            .as_ref()
            .ok_or(DockerEventParseError::MissingMessageAction)?;

        match value.typ.as_ref() {
            Some(EventMessageTypeEnum::CONTAINER) => Ok(Self::Container(message_action.parse()?)),
            Some(EventMessageTypeEnum::IMAGE) => Ok(Self::Image(message_action.parse()?)),
            Some(EventMessageTypeEnum::NETWORK) => Ok(Self::Network(message_action.parse()?)),
            Some(EventMessageTypeEnum::VOLUME) => Ok(Self::Volume(message_action.parse()?)),
            Some(EventMessageTypeEnum::PLUGIN) => Ok(Self::Plugin(message_action.parse()?)),
            Some(EventMessageTypeEnum::DAEMON) => Ok(Self::Daemon(message_action.parse()?)),
            Some(_) => Err(DockerEventParseError::UnknownMessageType),
            None => Err(DockerEventParseError::MissingMessageType),
        }
    }
}

pub enum ContainerEvent {
    Attach,
    Commit,
    Copy,
    Create,
    Destroy,
    Die,
    ExecCreate,
    ExecStart,
    Export,
    HealthStatus,
    Kill,
    Oom,
    Pause,
    Rename,
    Resize,
    Restart,
    Start,
    Stop,
    Top,
    Unpause,
    Update,
}

pub enum ImageEvent {
    Delete,
    Import,
    Load,
    Pull,
    Push,
    Save,
    Tag,
    Untag,
}

pub enum NetworkEvent {
    Connect,
    Create,
    Destroy,
    Disconnect,
    Update,
}

pub enum VolumeEvent {
    Create,
    Destroy,
    Mount,
    Unmount,
}

pub enum PluginEvent {
    Disable,
    Enable,
    Install,
    Remove,
    Upgrade,
}

pub enum DaemonEvent {
    Connect,
    Disconnect,
}

impl FromStr for ContainerEvent {
    type Err = DockerEventParseError;

    fn from_str(event: &str) -> Result<Self, Self::Err> {
        match event {
            "attach" => Ok(Self::Attach),
            "commit" => Ok(Self::Commit),
            "copy" => Ok(Self::Copy),
            "create" => Ok(Self::Create),
            "destroy" => Ok(Self::Destroy),
            "die" => Ok(Self::Die),
            "exec_create" => Ok(Self::ExecCreate),
            "exec_start" => Ok(Self::ExecStart),
            "export" => Ok(Self::Export),
            "health_status" => Ok(Self::HealthStatus),
            "kill" => Ok(Self::Kill),
            "oom" => Ok(Self::Oom),
            "pause" => Ok(Self::Pause),
            "rename" => Ok(Self::Rename),
            "resize" => Ok(Self::Resize),
            "restart" => Ok(Self::Restart),
            "start" => Ok(Self::Start),
            "stop" => Ok(Self::Stop),
            "top" => Ok(Self::Top),
            "unpause" => Ok(Self::Unpause),
            "update" => Ok(Self::Update),
            _ => Err(DockerEventParseError::UnknownMessageAction),
        }
    }
}

impl FromStr for ImageEvent {
    type Err = DockerEventParseError;

    fn from_str(event: &str) -> Result<Self, Self::Err> {
        match event {
            "delete" => Ok(Self::Delete),
            "import" => Ok(Self::Import),
            "load" => Ok(Self::Load),
            "pull" => Ok(Self::Pull),
            "push" => Ok(Self::Push),
            "save" => Ok(Self::Save),
            "tag" => Ok(Self::Tag),
            "untag" => Ok(Self::Untag),
            _ => Err(DockerEventParseError::UnknownMessageAction),
        }
    }
}

impl FromStr for NetworkEvent {
    type Err = DockerEventParseError;

    fn from_str(event: &str) -> Result<Self, Self::Err> {
        match event {
            "connect" => Ok(Self::Connect),
            "create" => Ok(Self::Create),
            "destroy" => Ok(Self::Destroy),
            "disconnect" => Ok(Self::Disconnect),
            "update" => Ok(Self::Update),
            _ => Err(DockerEventParseError::UnknownMessageAction),
        }
    }
}

impl FromStr for VolumeEvent {
    type Err = DockerEventParseError;

    fn from_str(event: &str) -> Result<Self, Self::Err> {
        match event {
            "create" => Ok(Self::Create),
            "destroy" => Ok(Self::Destroy),
            "mount" => Ok(Self::Mount),
            "unmount" => Ok(Self::Unmount),
            _ => Err(DockerEventParseError::UnknownMessageAction),
        }
    }
}

impl FromStr for PluginEvent {
    type Err = DockerEventParseError;

    fn from_str(event: &str) -> Result<Self, Self::Err> {
        match event {
            "disable" => Ok(Self::Disable),
            "enable" => Ok(Self::Enable),
            "install" => Ok(Self::Install),
            "remove" => Ok(Self::Remove),
            "upgrade" => Ok(Self::Upgrade),
            _ => Err(DockerEventParseError::UnknownMessageAction),
        }
    }
}

impl FromStr for DaemonEvent {
    type Err = DockerEventParseError;

    fn from_str(event: &str) -> Result<Self, Self::Err> {
        match event {
            "connect" => Ok(Self::Connect),
            "disconnect" => Ok(Self::Disconnect),
            _ => Err(DockerEventParseError::UnknownMessageAction),
        }
    }
}

#[derive(Debug)]
pub enum DockerEventParseError {
    MissingMessageType,
    UnknownMessageType,
    MissingMessageAction,
    UnknownMessageAction,
    MissingMessageActor,
}

impl Error for DockerEventParseError {}

impl std::fmt::Display for DockerEventParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingMessageType => write!(f, "Missing message type"),
            Self::UnknownMessageType => write!(f, "Unknown message type"),
            Self::MissingMessageAction => write!(f, "Missing message action"),
            Self::UnknownMessageAction => write!(f, "Unknown message action"),
            Self::MissingMessageActor => write!(f, "Missing message actor"),
        }
    }
}