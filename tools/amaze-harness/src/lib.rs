#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AmazeMuddleSurface {
    pub host_name: &'static str,
    pub title: &'static str,
    pub start_room: &'static str,
    pub rooms: Vec<AmazeMuddleRoom>,
    pub resources: Vec<AmazeMuddlePanelValue>,
    pub objectives: Vec<AmazeMuddleObjective>,
    pub commands: Vec<AmazeMuddleCommand>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AmazeMuddleRoom {
    pub id: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    pub exits: Vec<AmazeMuddleExit>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AmazeMuddleExit {
    pub command: &'static str,
    pub target_room: &'static str,
    pub label: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AmazeMuddlePanelValue {
    pub label: &'static str,
    pub value: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AmazeMuddleObjective {
    pub room_id: &'static str,
    pub text: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AmazeMuddleCommand {
    pub room_id: &'static str,
    pub command: &'static str,
    pub description: &'static str,
}

pub fn silverstream_muddle_surface() -> AmazeMuddleSurface {
    AmazeMuddleSurface {
        host_name: "amaze-silverstream",
        title: "AMAZE Silverstream",
        start_room: "silverstream-entry",
        rooms: vec![
            AmazeMuddleRoom {
                id: "silverstream-entry",
                title: "Silverstream Entry",
                description: "Enter the trailer-scale Silverstream escape room.",
                exits: vec![AmazeMuddleExit {
                    command: "go receiver",
                    target_room: "receiver-wall",
                    label: "Receiver Wall",
                }],
            },
            AmazeMuddleRoom {
                id: "receiver-wall",
                title: "Receiver Wall",
                description: "Solve a hidden clue, signal dial, and resettable hatch.",
                exits: vec![
                    AmazeMuddleExit {
                        command: "go entry",
                        target_room: "silverstream-entry",
                        label: "Silverstream Entry",
                    },
                    AmazeMuddleExit {
                        command: "go hatch",
                        target_room: "hatch-exit",
                        label: "Hatch Exit",
                    },
                ],
            },
            AmazeMuddleRoom {
                id: "hatch-exit",
                title: "Hatch Exit",
                description: "Complete the escape and prepare reset review.",
                exits: vec![AmazeMuddleExit {
                    command: "go receiver",
                    target_room: "receiver-wall",
                    label: "Receiver Wall",
                }],
            },
        ],
        resources: vec![
            AmazeMuddlePanelValue {
                label: "clue",
                value: "hidden",
            },
            AmazeMuddlePanelValue {
                label: "signal",
                value: "drifting",
            },
            AmazeMuddlePanelValue {
                label: "hatch",
                value: "locked",
            },
        ],
        objectives: vec![
            AmazeMuddleObjective {
                room_id: "silverstream-entry",
                text: "Move to the receiver wall.",
            },
            AmazeMuddleObjective {
                room_id: "receiver-wall",
                text: "Find the hidden clue, tune the signal, and unlock the hatch.",
            },
            AmazeMuddleObjective {
                room_id: "hatch-exit",
                text: "Escape complete; review the transcript and reset path.",
            },
        ],
        commands: vec![
            AmazeMuddleCommand {
                room_id: "receiver-wall",
                command: "inspect clue",
                description: "Search for the hidden clue.",
            },
            AmazeMuddleCommand {
                room_id: "receiver-wall",
                command: "tune signal",
                description: "Align the receiver signal.",
            },
            AmazeMuddleCommand {
                room_id: "receiver-wall",
                command: "unlock hatch",
                description: "Open the hatch after solving the wall.",
            },
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exposes_silverstream_muddle_surface() {
        let surface = silverstream_muddle_surface();

        assert_eq!(surface.host_name, "amaze-silverstream");
        assert_eq!(surface.start_room, "silverstream-entry");
        assert!(surface.rooms.iter().any(|room| room.id == "receiver-wall"));
        assert!(surface
            .commands
            .iter()
            .any(|command| command.command == "unlock hatch"));
    }
}
