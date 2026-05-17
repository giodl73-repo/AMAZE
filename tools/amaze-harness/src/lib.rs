use std::collections::HashMap;

use muddle_core::{
    MuddleCommand, MuddleCommandHint, MuddleCommandOutcome, MuddleError, MuddleExit, MuddleHost,
    MuddleResource, MuddleRoom,
};

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AmazeSilverstreamMuddleHost {
    rooms: HashMap<String, MuddleRoom>,
    state: AmazeSilverstreamMuddleState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AmazeSilverstreamMuddleState {
    pub clue_found: bool,
    pub signal_aligned: bool,
    pub hatch_unlocked: bool,
    pub hints_used: u8,
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

pub fn silverstream_muddle_host() -> AmazeSilverstreamMuddleHost {
    AmazeSilverstreamMuddleHost::new(silverstream_muddle_surface())
}

impl AmazeSilverstreamMuddleHost {
    pub fn new(surface: AmazeMuddleSurface) -> Self {
        let rooms = surface
            .rooms
            .into_iter()
            .map(|room| {
                (
                    room.id.to_string(),
                    MuddleRoom {
                        id: room.id.to_string(),
                        title: room.title.to_string(),
                        description: room.description.to_string(),
                        exits: room
                            .exits
                            .into_iter()
                            .map(|exit| MuddleExit {
                                command: exit.command.to_string(),
                                target_room: exit.target_room.to_string(),
                                label: exit.label.to_string(),
                            })
                            .collect(),
                    },
                )
            })
            .collect();

        Self {
            rooms,
            state: AmazeSilverstreamMuddleState {
                clue_found: false,
                signal_aligned: false,
                hatch_unlocked: false,
                hints_used: 0,
            },
        }
    }

    pub fn state(&self) -> &AmazeSilverstreamMuddleState {
        &self.state
    }

    fn look(&self, room_id: &str) -> Result<MuddleCommandOutcome, MuddleError> {
        let room = self
            .room(room_id)
            .ok_or_else(|| MuddleError::RoomNotFound {
                room_id: room_id.to_string(),
            })?;
        Ok(MuddleCommandOutcome::stay(format!(
            "{}\n| amaze: clue_found={} signal_aligned={} hatch_unlocked={} hints_used={}",
            room.ascii_card(),
            self.state.clue_found,
            self.state.signal_aligned,
            self.state.hatch_unlocked,
            self.state.hints_used
        )))
    }
}

impl MuddleHost for AmazeSilverstreamMuddleHost {
    fn start_room(&self) -> &str {
        "silverstream-entry"
    }

    fn room(&self, room_id: &str) -> Option<&MuddleRoom> {
        self.rooms.get(room_id)
    }

    fn resource_panel(&self) -> Vec<MuddleResource> {
        vec![
            MuddleResource {
                label: "clue".to_string(),
                value: if self.state.clue_found {
                    "found".to_string()
                } else {
                    "hidden".to_string()
                },
            },
            MuddleResource {
                label: "signal".to_string(),
                value: if self.state.signal_aligned {
                    "aligned".to_string()
                } else {
                    "drifting".to_string()
                },
            },
            MuddleResource {
                label: "hatch".to_string(),
                value: if self.state.hatch_unlocked {
                    "unlocked".to_string()
                } else {
                    "locked".to_string()
                },
            },
        ]
    }

    fn map_panel(&self, current_room: &str) -> Option<String> {
        Some(format!(
            "{} Entry -- {} Receiver Wall -- {} Hatch Exit",
            marker(current_room, "silverstream-entry"),
            marker(current_room, "receiver-wall"),
            marker(current_room, "hatch-exit")
        ))
    }

    fn objective_panel(&self, current_room: &str) -> Vec<String> {
        match current_room {
            "silverstream-entry" => vec!["Move to the receiver wall.".to_string()],
            "receiver-wall" if !self.state.clue_found => {
                vec!["Find the hidden clue before tuning the signal.".to_string()]
            }
            "receiver-wall" if !self.state.signal_aligned => {
                vec!["Tune the signal using the discovered clue.".to_string()]
            }
            "receiver-wall" if !self.state.hatch_unlocked => {
                vec!["Unlock the hatch after aligning the signal.".to_string()]
            }
            "receiver-wall" => vec!["Exit through the hatch.".to_string()],
            "hatch-exit" => {
                vec!["Escape complete; review the transcript and reset path.".to_string()]
            }
            _ => Vec::new(),
        }
    }

    fn command_panel(&self, current_room: &str) -> Vec<MuddleCommandHint> {
        match current_room {
            "silverstream-entry" => hints(&[
                ("look", "Show the entry room."),
                ("go receiver", "Move to the puzzle wall."),
            ]),
            "receiver-wall" => hints(&[
                ("look", "Show the receiver wall."),
                ("inspect clue", "Search for the hidden clue."),
                ("tune signal", "Align the receiver signal."),
                ("unlock hatch", "Open the hatch after solving the wall."),
                ("request hint", "Use an operator hint."),
                ("go entry", "Return to the entry."),
                ("go hatch", "Exit if the hatch is unlocked."),
            ]),
            "hatch-exit" => hints(&[
                ("look", "Show the hatch exit."),
                ("go receiver", "Return to the receiver wall."),
            ]),
            _ => Vec::new(),
        }
    }

    fn handle_command(
        &mut self,
        room_id: &str,
        command: &MuddleCommand,
    ) -> Result<MuddleCommandOutcome, MuddleError> {
        match (room_id, command.normalized().as_str()) {
            (_, "look") | (_, "status") => self.look(room_id),
            ("silverstream-entry", "go receiver") => Ok(MuddleCommandOutcome::move_to(
                "You step up to the receiver wall.",
                "receiver-wall",
            )),
            ("receiver-wall", "inspect clue") => {
                self.state.clue_found = true;
                Ok(MuddleCommandOutcome::stay(
                    "You find a silver frequency mark under the receiver bezel.",
                ))
            }
            ("receiver-wall", "tune signal") if !self.state.clue_found => {
                Ok(MuddleCommandOutcome::stay(
                    "The receiver drifts. You need the hidden frequency clue first.",
                ))
            }
            ("receiver-wall", "tune signal") => {
                self.state.signal_aligned = true;
                Ok(MuddleCommandOutcome::stay(
                    "The signal locks to the silver mark. The hatch relay clicks.",
                ))
            }
            ("receiver-wall", "unlock hatch") if !self.state.signal_aligned => Ok(
                MuddleCommandOutcome::stay("The hatch stays locked until the signal is aligned."),
            ),
            ("receiver-wall", "unlock hatch") => {
                self.state.hatch_unlocked = true;
                Ok(MuddleCommandOutcome::stay(
                    "The hatch unlocks with a clean resettable latch motion.",
                ))
            }
            ("receiver-wall", "request hint") => {
                self.state.hints_used += 1;
                Ok(MuddleCommandOutcome::stay(
                    "Operator hint: inspect the receiver bezel before tuning.",
                ))
            }
            ("receiver-wall", "go hatch") if self.state.hatch_unlocked => {
                Ok(MuddleCommandOutcome::move_to(
                    "You open the hatch and exit the Silverstream room.",
                    "hatch-exit",
                ))
            }
            ("receiver-wall", "go hatch") => Ok(MuddleCommandOutcome::stay(
                "The hatch is still locked. Solve the receiver sequence first.",
            )),
            ("receiver-wall", "go entry") => Ok(MuddleCommandOutcome::move_to(
                "You return to the Silverstream entry.",
                "silverstream-entry",
            )),
            ("hatch-exit", "go receiver") => Ok(MuddleCommandOutcome::move_to(
                "You return to the receiver wall for reset review.",
                "receiver-wall",
            )),
            _ => Err(MuddleError::UnknownCommand {
                room_id: room_id.to_string(),
                command: command.clone(),
            }),
        }
    }
}

fn marker(current_room: &str, room_id: &str) -> &'static str {
    if current_room == room_id {
        "@"
    } else {
        "o"
    }
}

fn hints(items: &[(&str, &str)]) -> Vec<MuddleCommandHint> {
    items
        .iter()
        .map(|(command, description)| MuddleCommandHint {
            command: (*command).to_string(),
            description: (*description).to_string(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use muddle_core::{MuddleCommand, MuddleSession};

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

    #[test]
    fn product_owned_muddle_host_plays_silverstream_path() {
        let mut host = silverstream_muddle_host();
        let mut session = MuddleSession::for_host(&host).expect("host has start room");

        session
            .play_turn(&mut host, MuddleCommand::parse("go receiver"))
            .expect("entry moves to receiver");
        session
            .play_turn(&mut host, MuddleCommand::parse("inspect clue"))
            .expect("clue can be inspected");
        session
            .play_turn(&mut host, MuddleCommand::parse("tune signal"))
            .expect("signal tunes after clue");
        session
            .play_turn(&mut host, MuddleCommand::parse("unlock hatch"))
            .expect("hatch unlocks");
        session
            .play_turn(&mut host, MuddleCommand::parse("go hatch"))
            .expect("unlocked hatch exits");

        assert_eq!(session.current_room, "hatch-exit");
        assert!(host.state().hatch_unlocked);
        assert_eq!(host.resource_panel()[2].value, "unlocked");
    }
}
