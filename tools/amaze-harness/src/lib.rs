use std::collections::HashMap;

use muddle_core::{
    MuddleCommand, MuddleCommandHint, MuddleCommandOutcome, MuddleError, MuddleExit, MuddleHost,
    MuddleInventoryItem, MuddleResource, MuddleRoom, MuddleVisualNode,
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
    pub route_planned: bool,
    pub breakers_set: bool,
    pub galley_sorted: bool,
    pub table_aligned: bool,
    pub broadcast_tuned: bool,
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
                description:
                    "Enter the trailer-scale Silverstream escape room and face the route rail.",
                exits: vec![AmazeMuddleExit {
                    command: "go route",
                    target_room: "route-rail",
                    label: "Route Rail",
                }],
            },
            AmazeMuddleRoom {
                id: "route-rail",
                title: "Route Rail",
                description: "Order postcards against the public map before powering the trailer.",
                exits: vec![
                    AmazeMuddleExit {
                        command: "go entry",
                        target_room: "silverstream-entry",
                        label: "Silverstream Entry",
                    },
                    AmazeMuddleExit {
                        command: "go breaker",
                        target_room: "breaker-panel",
                        label: "Breaker Panel",
                    },
                ],
            },
            AmazeMuddleRoom {
                id: "breaker-panel",
                title: "Breaker Panel",
                description: "Set the low-voltage prop breakers to wake the UV-safe cabinet path.",
                exits: vec![
                    AmazeMuddleExit {
                        command: "go route",
                        target_room: "route-rail",
                        label: "Route Rail",
                    },
                    AmazeMuddleExit {
                        command: "go galley",
                        target_room: "galley-cabinet",
                        label: "Galley Cabinet",
                    },
                ],
            },
            AmazeMuddleRoom {
                id: "galley-cabinet",
                title: "Galley Cabinet",
                description: "Sort the counted galley objects and reveal the frequency mark.",
                exits: vec![
                    AmazeMuddleExit {
                        command: "go breaker",
                        target_room: "breaker-panel",
                        label: "Breaker Panel",
                    },
                    AmazeMuddleExit {
                        command: "go table",
                        target_room: "fold-table",
                        label: "Fold Table",
                    },
                ],
            },
            AmazeMuddleRoom {
                id: "fold-table",
                title: "Fold Table",
                description: "Align the transform table to expose the bearing for the final radio.",
                exits: vec![
                    AmazeMuddleExit {
                        command: "go galley",
                        target_room: "galley-cabinet",
                        label: "Galley Cabinet",
                    },
                    AmazeMuddleExit {
                        command: "go radio",
                        target_room: "radio-nook",
                        label: "Radio Nook",
                    },
                ],
            },
            AmazeMuddleRoom {
                id: "radio-nook",
                title: "Radio Nook",
                description: "Tune the tactile radio and trigger the final Silverstream broadcast.",
                exits: vec![
                    AmazeMuddleExit {
                        command: "go table",
                        target_room: "fold-table",
                        label: "Fold Table",
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
                label: "route",
                value: "unordered",
            },
            AmazeMuddlePanelValue {
                label: "breakers",
                value: "off",
            },
            AmazeMuddlePanelValue {
                label: "galley",
                value: "unsorted",
            },
            AmazeMuddlePanelValue {
                label: "table",
                value: "folded",
            },
            AmazeMuddlePanelValue {
                label: "radio",
                value: "static",
            },
            AmazeMuddlePanelValue {
                label: "hatch",
                value: "locked",
            },
        ],
        objectives: vec![
            AmazeMuddleObjective {
                room_id: "silverstream-entry",
                text: "Start at the route rail and make the trailer route public.",
            },
            AmazeMuddleObjective {
                room_id: "route-rail",
                text: "Sort the postcards against the mounted map.",
            },
            AmazeMuddleObjective {
                room_id: "breaker-panel",
                text: "Set breakers in route order to power the reveal path.",
            },
            AmazeMuddleObjective {
                room_id: "galley-cabinet",
                text: "Sort counted galley objects and reveal the frequency.",
            },
            AmazeMuddleObjective {
                room_id: "fold-table",
                text: "Align the transform table to expose the bearing.",
            },
            AmazeMuddleObjective {
                room_id: "radio-nook",
                text: "Tune the radio, unlock the hatch, and exit.",
            },
            AmazeMuddleObjective {
                room_id: "hatch-exit",
                text: "Escape complete; review transcript, bypasses, and reset path.",
            },
        ],
        commands: vec![
            AmazeMuddleCommand {
                room_id: "route-rail",
                command: "sort postcards",
                description: "Order the route postcards against the wall map.",
            },
            AmazeMuddleCommand {
                room_id: "breaker-panel",
                command: "set breakers",
                description: "Set the prop breakers in route order.",
            },
            AmazeMuddleCommand {
                room_id: "galley-cabinet",
                command: "sort galley",
                description: "Place the counted galley objects into their homes.",
            },
            AmazeMuddleCommand {
                room_id: "fold-table",
                command: "align table",
                description: "Rotate the transform table to the visible bearing.",
            },
            AmazeMuddleCommand {
                room_id: "radio-nook",
                command: "tune radio",
                description: "Tune the tactile radio to the revealed frequency.",
            },
            AmazeMuddleCommand {
                room_id: "radio-nook",
                command: "unlock hatch",
                description: "Open the hatch after the final broadcast.",
            },
        ],
    }
}

pub fn silverstream_muddle_host() -> AmazeSilverstreamMuddleHost {
    AmazeSilverstreamMuddleHost::new(silverstream_muddle_surface())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AmazePrismVaultMuddleHost {
    rooms: HashMap<String, MuddleRoom>,
    state: AmazePrismVaultMuddleState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AmazePrismVaultMuddleState {
    pub lens_aligned: bool,
    pub color_mixed: bool,
    pub mirrors_set: bool,
    pub vault_unlocked: bool,
    pub exit_open: bool,
    pub hints_used: u8,
}

pub fn prism_vault_muddle_surface() -> AmazeMuddleSurface {
    AmazeMuddleSurface {
        host_name: "amaze-prism-vault",
        title: "AMAZE Prism Vault",
        start_room: "prism-entry",
        rooms: vec![
            AmazeMuddleRoom {
                id: "prism-entry",
                title: "Prism Entry",
                description:
                    "A daylight-safe foyer frames the lens bench and a locked garden exit.",
                exits: vec![AmazeMuddleExit {
                    command: "go lens",
                    target_room: "lens-bench",
                    label: "Lens Bench",
                }],
            },
            AmazeMuddleRoom {
                id: "lens-bench",
                title: "Lens Bench",
                description: "A chunky lens rail needs alignment before color can be trusted.",
                exits: vec![
                    AmazeMuddleExit {
                        command: "go entry",
                        target_room: "prism-entry",
                        label: "Prism Entry",
                    },
                    AmazeMuddleExit {
                        command: "go color",
                        target_room: "color-sink",
                        label: "Color Sink",
                    },
                ],
            },
            AmazeMuddleRoom {
                id: "color-sink",
                title: "Color Sink",
                description: "Three tactile color sliders mix a code visible to the mirror wall.",
                exits: vec![
                    AmazeMuddleExit {
                        command: "go lens",
                        target_room: "lens-bench",
                        label: "Lens Bench",
                    },
                    AmazeMuddleExit {
                        command: "go mirrors",
                        target_room: "mirror-wall",
                        label: "Mirror Wall",
                    },
                ],
            },
            AmazeMuddleRoom {
                id: "mirror-wall",
                title: "Mirror Wall",
                description: "Resettable mirrors bounce the mixed code toward the vault door.",
                exits: vec![
                    AmazeMuddleExit {
                        command: "go color",
                        target_room: "color-sink",
                        label: "Color Sink",
                    },
                    AmazeMuddleExit {
                        command: "go vault",
                        target_room: "vault-door",
                        label: "Vault Door",
                    },
                ],
            },
            AmazeMuddleRoom {
                id: "vault-door",
                title: "Vault Door",
                description:
                    "The prism vault accepts the reflected code and releases the reset key.",
                exits: vec![
                    AmazeMuddleExit {
                        command: "go mirrors",
                        target_room: "mirror-wall",
                        label: "Mirror Wall",
                    },
                    AmazeMuddleExit {
                        command: "go exit",
                        target_room: "garden-exit",
                        label: "Garden Exit",
                    },
                ],
            },
            AmazeMuddleRoom {
                id: "garden-exit",
                title: "Garden Exit",
                description: "A reset-ready garden door completes the Prism Vault escape.",
                exits: vec![AmazeMuddleExit {
                    command: "go vault",
                    target_room: "vault-door",
                    label: "Vault Door",
                }],
            },
        ],
        resources: vec![
            AmazeMuddlePanelValue {
                label: "lens",
                value: "skewed",
            },
            AmazeMuddlePanelValue {
                label: "color",
                value: "unmixed",
            },
            AmazeMuddlePanelValue {
                label: "mirrors",
                value: "scattered",
            },
            AmazeMuddlePanelValue {
                label: "vault",
                value: "locked",
            },
            AmazeMuddlePanelValue {
                label: "exit",
                value: "closed",
            },
        ],
        objectives: vec![
            AmazeMuddleObjective {
                room_id: "prism-entry",
                text: "Enter the lens bench and establish a safe light path.",
            },
            AmazeMuddleObjective {
                room_id: "lens-bench",
                text: "Align the lens rail before mixing the color code.",
            },
            AmazeMuddleObjective {
                room_id: "color-sink",
                text: "Mix the color sliders into the prism code.",
            },
            AmazeMuddleObjective {
                room_id: "mirror-wall",
                text: "Set mirrors to carry the code to the vault.",
            },
            AmazeMuddleObjective {
                room_id: "vault-door",
                text: "Unlock the vault and open the garden exit.",
            },
            AmazeMuddleObjective {
                room_id: "garden-exit",
                text: "Escape complete; verify the reset key and transcript.",
            },
        ],
        commands: vec![
            AmazeMuddleCommand {
                room_id: "lens-bench",
                command: "align lens",
                description: "Seat the chunky lens rail into the safe detent.",
            },
            AmazeMuddleCommand {
                room_id: "color-sink",
                command: "mix color",
                description: "Mix the tactile color sliders into a visible code.",
            },
            AmazeMuddleCommand {
                room_id: "mirror-wall",
                command: "set mirrors",
                description: "Set the resettable mirror wall to carry the code.",
            },
            AmazeMuddleCommand {
                room_id: "vault-door",
                command: "unlock vault",
                description: "Open the prism vault after the mirrors are set.",
            },
        ],
    }
}

pub fn prism_vault_muddle_host() -> AmazePrismVaultMuddleHost {
    AmazePrismVaultMuddleHost::new(prism_vault_muddle_surface())
}

impl AmazePrismVaultMuddleHost {
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
            state: AmazePrismVaultMuddleState {
                lens_aligned: false,
                color_mixed: false,
                mirrors_set: false,
                vault_unlocked: false,
                exit_open: false,
                hints_used: 0,
            },
        }
    }

    pub fn state(&self) -> &AmazePrismVaultMuddleState {
        &self.state
    }

    fn look(&self, room_id: &str) -> Result<MuddleCommandOutcome, MuddleError> {
        let room = self
            .room(room_id)
            .ok_or_else(|| MuddleError::RoomNotFound {
                room_id: room_id.to_string(),
            })?;
        Ok(MuddleCommandOutcome::stay(format!(
            "{}\n| amaze: lens_aligned={} color_mixed={} mirrors_set={} vault_unlocked={} exit_open={} hints_used={}",
            room.ascii_card(),
            self.state.lens_aligned,
            self.state.color_mixed,
            self.state.mirrors_set,
            self.state.vault_unlocked,
            self.state.exit_open,
            self.state.hints_used
        )))
    }
}

impl MuddleHost for AmazePrismVaultMuddleHost {
    fn start_room(&self) -> &str {
        "prism-entry"
    }

    fn room(&self, room_id: &str) -> Option<&MuddleRoom> {
        self.rooms.get(room_id)
    }

    fn resource_panel(&self) -> Vec<MuddleResource> {
        vec![
            MuddleResource {
                label: "lens".to_string(),
                value: if self.state.lens_aligned {
                    "aligned"
                } else {
                    "skewed"
                }
                .to_string(),
            },
            MuddleResource {
                label: "color".to_string(),
                value: if self.state.color_mixed {
                    "mixed"
                } else {
                    "unmixed"
                }
                .to_string(),
            },
            MuddleResource {
                label: "mirrors".to_string(),
                value: if self.state.mirrors_set {
                    "set"
                } else {
                    "scattered"
                }
                .to_string(),
            },
            MuddleResource {
                label: "vault".to_string(),
                value: if self.state.vault_unlocked {
                    "open"
                } else {
                    "locked"
                }
                .to_string(),
            },
            MuddleResource {
                label: "exit".to_string(),
                value: if self.state.exit_open {
                    "escaped"
                } else {
                    "closed"
                }
                .to_string(),
            },
        ]
    }

    fn inventory_panel(&self) -> Vec<MuddleInventoryItem> {
        let mut items = vec![MuddleInventoryItem {
            label: "operator card".to_string(),
            detail: format!("{} prism hints used", self.state.hints_used),
        }];
        if self.state.lens_aligned {
            items.push(MuddleInventoryItem {
                label: "aligned lens".to_string(),
                detail: "safe light path established".to_string(),
            });
        }
        if self.state.color_mixed {
            items.push(MuddleInventoryItem {
                label: "prism code".to_string(),
                detail: "color sliders expose the vault code".to_string(),
            });
        }
        if self.state.mirrors_set {
            items.push(MuddleInventoryItem {
                label: "mirror path".to_string(),
                detail: "reflected code reaches the vault door".to_string(),
            });
        }
        if self.state.exit_open {
            items.push(MuddleInventoryItem {
                label: "prism reset key".to_string(),
                detail: "reset safe exit complete".to_string(),
            });
        }
        items
    }

    fn map_panel(&self, current_room: &str) -> Option<String> {
        Some(format!(
            "{} Entry -- {} Lens -- {} Color -- {} Mirrors -- {} Vault -- {} Garden",
            marker(current_room, "prism-entry"),
            marker(current_room, "lens-bench"),
            marker(current_room, "color-sink"),
            marker(current_room, "mirror-wall"),
            marker(current_room, "vault-door"),
            marker(current_room, "garden-exit")
        ))
    }

    fn objective_panel(&self, current_room: &str) -> Vec<String> {
        match current_room {
            "prism-entry" => vec!["Move to the lens bench.".to_string()],
            "lens-bench" if !self.state.lens_aligned => {
                vec!["Align the lens rail before mixing color.".to_string()]
            }
            "lens-bench" => vec!["Move to the color sink.".to_string()],
            "color-sink" if !self.state.lens_aligned => {
                vec!["Return to the lens bench before mixing color.".to_string()]
            }
            "color-sink" if !self.state.color_mixed => {
                vec!["Mix the prism color code.".to_string()]
            }
            "color-sink" => vec!["Move to the mirror wall.".to_string()],
            "mirror-wall" if !self.state.color_mixed => {
                vec!["Mix the color code before setting mirrors.".to_string()]
            }
            "mirror-wall" if !self.state.mirrors_set => {
                vec!["Set mirrors to carry the code to the vault.".to_string()]
            }
            "mirror-wall" => vec!["Move to the vault door.".to_string()],
            "vault-door" if !self.state.mirrors_set => {
                vec!["Set the mirror wall before unlocking the vault.".to_string()]
            }
            "vault-door" if !self.state.vault_unlocked => {
                vec!["Unlock the vault to release the reset key.".to_string()]
            }
            "vault-door" => vec!["Go exit through the open garden door.".to_string()],
            "garden-exit" => {
                vec!["Escape complete; review the reset key and transcript.".to_string()]
            }
            _ => Vec::new(),
        }
    }

    fn command_panel(&self, current_room: &str) -> Vec<MuddleCommandHint> {
        match current_room {
            "prism-entry" => hints(&[
                ("look", "Show the prism entry."),
                ("go lens", "Move to the lens bench."),
            ]),
            "lens-bench" => hints(&[
                ("look", "Show the lens bench."),
                ("align lens", "Align the chunky lens rail."),
                ("request hint", "Ask for an operator-safe hint."),
                ("go entry", "Return to the prism entry."),
                ("go color", "Move to the color sink."),
            ]),
            "color-sink" => hints(&[
                ("look", "Show the color sink."),
                ("mix color", "Mix the prism color code."),
                ("request hint", "Ask for an operator-safe hint."),
                ("go lens", "Return to the lens bench."),
                ("go mirrors", "Move to the mirror wall."),
            ]),
            "mirror-wall" => hints(&[
                ("look", "Show the mirror wall."),
                ("set mirrors", "Set the mirror path."),
                ("request hint", "Ask for an operator-safe hint."),
                ("go color", "Return to the color sink."),
                ("go vault", "Move to the vault door."),
            ]),
            "vault-door" => hints(&[
                ("look", "Show the vault door."),
                ("unlock vault", "Unlock the prism vault."),
                ("request hint", "Ask for an operator-safe hint."),
                ("go mirrors", "Return to the mirror wall."),
                ("go exit", "Exit if the vault is unlocked."),
            ]),
            "garden-exit" => hints(&[
                ("look", "Show the garden exit."),
                ("go vault", "Return to the vault door."),
            ]),
            _ => Vec::new(),
        }
    }

    fn visual_nodes(&self, current_room: &str) -> Vec<MuddleVisualNode> {
        let mut children = vec![
            MuddleVisualNode::sprite(
                "prism-vault-map",
                "Prism Vault map",
                "sprites/amaze/prism-vault-map.png",
                "A prism-entry-to-garden escape-room strip.",
            )
            .with_layer(0)
            .with_rect(0, 0, 12, 6),
            MuddleVisualNode::text("current-room-label", "Current room", current_room)
                .with_layer(30)
                .with_rect(1, 0, 4, 1),
            amaze_room_token("prism-entry-token", "Entry", "prism-entry", current_room, 1),
            amaze_room_token("lens-token", "Lens bench", "lens-bench", current_room, 2),
            amaze_room_token("color-token", "Color sink", "color-sink", current_room, 3),
            amaze_room_token(
                "mirror-token",
                "Mirror wall",
                "mirror-wall",
                current_room,
                4,
            ),
            amaze_room_token("vault-token", "Vault door", "vault-door", current_room, 5),
            amaze_room_token(
                "garden-token",
                "Garden exit",
                "garden-exit",
                current_room,
                6,
            ),
        ];

        if self.state.lens_aligned {
            children.push(
                MuddleVisualNode::sprite(
                    "lens-aligned-badge",
                    "Lens aligned",
                    "sprites/amaze/prism-lens.png",
                    "Aligned lens badge.",
                )
                .with_layer(20)
                .with_rect(1, 6, 2, 1)
                .with_frame("aligned"),
            );
        }
        if self.state.color_mixed {
            children.push(
                MuddleVisualNode::sprite(
                    "color-mixed-badge",
                    "Color mixed",
                    "sprites/amaze/prism-color.png",
                    "Mixed color code badge.",
                )
                .with_layer(20)
                .with_rect(3, 6, 2, 1)
                .with_frame("mixed"),
            );
        }
        if self.state.mirrors_set {
            children.push(
                MuddleVisualNode::sprite(
                    "mirrors-set-badge",
                    "Mirrors set",
                    "sprites/amaze/prism-mirrors.png",
                    "Mirror path set badge.",
                )
                .with_layer(20)
                .with_rect(5, 6, 2, 1)
                .with_frame("set"),
            );
        }
        if self.state.vault_unlocked {
            children.push(
                MuddleVisualNode::sprite(
                    "vault-open-badge",
                    "Vault open",
                    "sprites/amaze/prism-vault.png",
                    "Open prism vault badge.",
                )
                .with_layer(20)
                .with_rect(7, 6, 2, 1)
                .with_frame("open")
                .with_animation("pulse"),
            );
        }

        vec![MuddleVisualNode::group(
            "prism-vault-scene",
            "Prism Vault scene",
            children,
        )]
    }

    fn export_checkpoint(&self) -> Option<String> {
        Some(format!(
            "lens_aligned={};color_mixed={};mirrors_set={};vault_unlocked={};exit_open={};hints_used={}",
            self.state.lens_aligned,
            self.state.color_mixed,
            self.state.mirrors_set,
            self.state.vault_unlocked,
            self.state.exit_open,
            self.state.hints_used
        ))
    }

    fn import_checkpoint(&mut self, checkpoint: &str) -> Result<(), MuddleError> {
        let mut lens_aligned = None;
        let mut color_mixed = None;
        let mut mirrors_set = None;
        let mut vault_unlocked = None;
        let mut exit_open = None;
        let mut hints_used = None;

        for part in checkpoint.split(';') {
            let (key, value) =
                part.split_once('=')
                    .ok_or_else(|| MuddleError::InvalidHostCheckpoint {
                        message: format!("malformed checkpoint field `{part}`"),
                    })?;
            match key {
                "lens_aligned" => lens_aligned = Some(parse_checkpoint_bool(key, value)?),
                "color_mixed" => color_mixed = Some(parse_checkpoint_bool(key, value)?),
                "mirrors_set" => mirrors_set = Some(parse_checkpoint_bool(key, value)?),
                "vault_unlocked" => vault_unlocked = Some(parse_checkpoint_bool(key, value)?),
                "exit_open" => exit_open = Some(parse_checkpoint_bool(key, value)?),
                "hints_used" => {
                    hints_used = Some(value.parse::<u8>().map_err(|_| {
                        MuddleError::InvalidHostCheckpoint {
                            message: format!("invalid hints_used checkpoint field `{value}`"),
                        }
                    })?);
                }
                _ => {
                    return Err(MuddleError::InvalidHostCheckpoint {
                        message: format!("unknown checkpoint field `{key}`"),
                    });
                }
            }
        }

        self.state = AmazePrismVaultMuddleState {
            lens_aligned: lens_aligned.ok_or_else(|| MuddleError::InvalidHostCheckpoint {
                message: "missing lens_aligned checkpoint field".to_string(),
            })?,
            color_mixed: color_mixed.ok_or_else(|| MuddleError::InvalidHostCheckpoint {
                message: "missing color_mixed checkpoint field".to_string(),
            })?,
            mirrors_set: mirrors_set.ok_or_else(|| MuddleError::InvalidHostCheckpoint {
                message: "missing mirrors_set checkpoint field".to_string(),
            })?,
            vault_unlocked: vault_unlocked.ok_or_else(|| MuddleError::InvalidHostCheckpoint {
                message: "missing vault_unlocked checkpoint field".to_string(),
            })?,
            exit_open: exit_open.ok_or_else(|| MuddleError::InvalidHostCheckpoint {
                message: "missing exit_open checkpoint field".to_string(),
            })?,
            hints_used: hints_used.ok_or_else(|| MuddleError::InvalidHostCheckpoint {
                message: "missing hints_used checkpoint field".to_string(),
            })?,
        };
        Ok(())
    }

    fn handle_command(
        &mut self,
        room_id: &str,
        command: &MuddleCommand,
    ) -> Result<MuddleCommandOutcome, MuddleError> {
        let normalized = command.normalized();
        match (room_id, normalized.as_str()) {
            (_, "look") => self.look(room_id),
            ("prism-entry", "go lens") => Ok(MuddleCommandOutcome::move_to(
                "You move to the chunky lens bench.",
                "lens-bench",
            )),
            ("lens-bench", "align lens") => {
                self.state.lens_aligned = true;
                Ok(MuddleCommandOutcome::stay(
                    "The lens rail seats into its safe detent. The color sink wakes.",
                ))
            }
            ("lens-bench", "request hint") => {
                self.state.hints_used += 1;
                Ok(MuddleCommandOutcome::stay(
                    "Operator hint: align the large arrow mark with the daylight stripe.",
                ))
            }
            ("lens-bench", "go entry") => Ok(MuddleCommandOutcome::move_to(
                "You return to the Prism entry.",
                "prism-entry",
            )),
            ("lens-bench", "go color") => Ok(MuddleCommandOutcome::move_to(
                "You move to the tactile color sink.",
                "color-sink",
            )),
            ("color-sink", "mix color") if !self.state.lens_aligned => {
                Ok(MuddleCommandOutcome::stay(
                    "The color sliders need a safe light path. Align the lens first.",
                ))
            }
            ("color-sink", "mix color") => {
                self.state.color_mixed = true;
                Ok(MuddleCommandOutcome::stay(
                    "The color sliders settle into cyan, amber, and violet. A prism code appears.",
                ))
            }
            ("color-sink", "request hint") => {
                self.state.hints_used += 1;
                Ok(MuddleCommandOutcome::stay(
                    "Operator hint: match the tactile slider notches to the three lens marks.",
                ))
            }
            ("color-sink", "go lens") => Ok(MuddleCommandOutcome::move_to(
                "You return to the lens bench.",
                "lens-bench",
            )),
            ("color-sink", "go mirrors") => Ok(MuddleCommandOutcome::move_to(
                "You move to the mirror wall.",
                "mirror-wall",
            )),
            ("mirror-wall", "set mirrors") if !self.state.color_mixed => {
                Ok(MuddleCommandOutcome::stay(
                    "The mirrors need the color code. Mix color before setting mirrors.",
                ))
            }
            ("mirror-wall", "set mirrors") => {
                self.state.mirrors_set = true;
                Ok(MuddleCommandOutcome::stay(
                    "The mirrors click into a resettable path and throw the prism code at the vault.",
                ))
            }
            ("mirror-wall", "request hint") => {
                self.state.hints_used += 1;
                Ok(MuddleCommandOutcome::stay(
                    "Operator hint: start with the largest mirror, then follow the colored edge labels.",
                ))
            }
            ("mirror-wall", "go color") => Ok(MuddleCommandOutcome::move_to(
                "You return to the color sink.",
                "color-sink",
            )),
            ("mirror-wall", "go vault") => Ok(MuddleCommandOutcome::move_to(
                "You move to the prism vault door.",
                "vault-door",
            )),
            ("vault-door", "unlock vault") if !self.state.mirrors_set => {
                Ok(MuddleCommandOutcome::stay(
                    "The vault lens is dark. Set mirrors before unlocking the vault.",
                ))
            }
            ("vault-door", "unlock vault") => {
                self.state.vault_unlocked = true;
                self.state.exit_open = true;
                Ok(MuddleCommandOutcome::stay(
                    "The prism vault opens. A reset key drops and the garden exit unlocks.",
                ))
            }
            ("vault-door", "request hint") => {
                self.state.hints_used += 1;
                Ok(MuddleCommandOutcome::stay(
                    "Operator hint: the vault accepts the reflected code, not a manual force input.",
                ))
            }
            ("vault-door", "go mirrors") => Ok(MuddleCommandOutcome::move_to(
                "You return to the mirror wall.",
                "mirror-wall",
            )),
            ("vault-door", "go exit") if self.state.exit_open => Ok(MuddleCommandOutcome::move_to(
                "You step through the garden exit with the prism key reset.",
                "garden-exit",
            )),
            ("vault-door", "go exit") => Ok(MuddleCommandOutcome::stay(
                "The garden exit is still closed. Unlock the prism vault first.",
            )),
            ("garden-exit", "go vault") => Ok(MuddleCommandOutcome::move_to(
                "You return to the vault door for reset review.",
                "vault-door",
            )),
            (_, "go exit") if self.state.exit_open => Ok(MuddleCommandOutcome::move_to(
                "You follow the solved light path and step through the garden exit.",
                "garden-exit",
            )),
            (_, "go exit") => Ok(MuddleCommandOutcome::stay(
                "The exit route needs the lens, color, mirrors, and vault solved first.",
            )),
            _ => Err(MuddleError::UnknownCommand {
                room_id: room_id.to_string(),
                command: command.clone(),
            }),
        }
    }
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
                route_planned: false,
                breakers_set: false,
                galley_sorted: false,
                table_aligned: false,
                broadcast_tuned: false,
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
            "{}\n| amaze: route_planned={} breakers_set={} galley_sorted={} table_aligned={} broadcast_tuned={} hatch_unlocked={} hints_used={}",
            room.ascii_card(),
            self.state.route_planned,
            self.state.breakers_set,
            self.state.galley_sorted,
            self.state.table_aligned,
            self.state.broadcast_tuned,
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
                label: "route".to_string(),
                value: if self.state.route_planned {
                    "ordered".to_string()
                } else {
                    "unordered".to_string()
                },
            },
            MuddleResource {
                label: "breakers".to_string(),
                value: if self.state.breakers_set {
                    "set".to_string()
                } else {
                    "off".to_string()
                },
            },
            MuddleResource {
                label: "galley".to_string(),
                value: if self.state.galley_sorted {
                    "sorted".to_string()
                } else {
                    "unsorted".to_string()
                },
            },
            MuddleResource {
                label: "table".to_string(),
                value: if self.state.table_aligned {
                    "bearing shown".to_string()
                } else {
                    "folded".to_string()
                },
            },
            MuddleResource {
                label: "radio".to_string(),
                value: if self.state.broadcast_tuned {
                    "broadcast".to_string()
                } else {
                    "static".to_string()
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

    fn inventory_panel(&self) -> Vec<MuddleInventoryItem> {
        let mut items = vec![MuddleInventoryItem {
            label: "operator radio".to_string(),
            detail: format!("{} hints used", self.state.hints_used),
        }];

        if self.state.route_planned {
            items.push(MuddleInventoryItem {
                label: "ordered postcard route".to_string(),
                detail: "breaker sequence is public".to_string(),
            });
        }

        if self.state.galley_sorted {
            items.push(MuddleInventoryItem {
                label: "silver frequency mark".to_string(),
                detail: "galley reveal for the radio dial".to_string(),
            });
        }

        if self.state.table_aligned {
            items.push(MuddleInventoryItem {
                label: "compass bearing".to_string(),
                detail: "fold table bearing is exposed".to_string(),
            });
        }

        if self.state.hatch_unlocked {
            items.push(MuddleInventoryItem {
                label: "hatch reset key".to_string(),
                detail: "exit is open".to_string(),
            });
        }

        items
    }

    fn map_panel(&self, current_room: &str) -> Option<String> {
        Some(format!(
            "{} Entry -- {} Route -- {} Breakers -- {} Galley -- {} Table -- {} Radio -- {} Hatch",
            marker(current_room, "silverstream-entry"),
            marker(current_room, "route-rail"),
            marker(current_room, "breaker-panel"),
            marker(current_room, "galley-cabinet"),
            marker(current_room, "fold-table"),
            marker(current_room, "radio-nook"),
            marker(current_room, "hatch-exit")
        ))
    }

    fn objective_panel(&self, current_room: &str) -> Vec<String> {
        match current_room {
            "silverstream-entry" => vec!["Move to the route rail.".to_string()],
            "route-rail" if !self.state.route_planned => {
                vec!["Sort postcards against the mounted map.".to_string()]
            }
            "route-rail" => vec!["Move to the breaker panel.".to_string()],
            "breaker-panel" if !self.state.route_planned => {
                vec!["Return to the route rail before setting breakers.".to_string()]
            }
            "breaker-panel" if !self.state.breakers_set => {
                vec!["Set breakers in the route order.".to_string()]
            }
            "breaker-panel" => vec!["Move to the galley cabinet.".to_string()],
            "galley-cabinet" if !self.state.breakers_set => {
                vec!["Set the breakers before sorting the galley.".to_string()]
            }
            "galley-cabinet" if !self.state.galley_sorted => {
                vec!["Sort counted galley objects to reveal the frequency.".to_string()]
            }
            "galley-cabinet" => vec!["Move to the fold table.".to_string()],
            "fold-table" if !self.state.galley_sorted => {
                vec!["Reveal the frequency in the galley first.".to_string()]
            }
            "fold-table" if !self.state.table_aligned => {
                vec!["Align the fold table to expose the compass bearing.".to_string()]
            }
            "fold-table" => vec!["Move to the radio nook.".to_string()],
            "radio-nook" if !self.state.table_aligned => {
                vec!["Expose the table bearing before tuning the radio.".to_string()]
            }
            "radio-nook" if !self.state.broadcast_tuned => {
                vec!["Tune the radio to trigger the final broadcast.".to_string()]
            }
            "radio-nook" if !self.state.hatch_unlocked => {
                vec!["Unlock the hatch after the broadcast.".to_string()]
            }
            "radio-nook" => vec!["Exit through the hatch.".to_string()],
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
                ("go route", "Move to the route rail."),
            ]),
            "route-rail" => hints(&[
                ("look", "Show the route rail."),
                ("sort postcards", "Order the postcards against the map."),
                ("request hint", "Use an operator hint."),
                ("go entry", "Return to the entry."),
                ("go breaker", "Move to the breaker panel."),
            ]),
            "breaker-panel" => hints(&[
                ("look", "Show the breaker panel."),
                ("set breakers", "Set the prop breakers."),
                ("request hint", "Use an operator hint."),
                ("go route", "Return to the route rail."),
                ("go galley", "Move to the galley cabinet."),
            ]),
            "galley-cabinet" => hints(&[
                ("look", "Show the galley cabinet."),
                ("sort galley", "Sort the counted galley objects."),
                ("request hint", "Use an operator hint."),
                ("go breaker", "Return to the breaker panel."),
                ("go table", "Move to the fold table."),
            ]),
            "fold-table" => hints(&[
                ("look", "Show the fold table."),
                ("align table", "Align the transform table."),
                ("request hint", "Use an operator hint."),
                ("go galley", "Return to the galley cabinet."),
                ("go radio", "Move to the radio nook."),
            ]),
            "radio-nook" => hints(&[
                ("look", "Show the radio nook."),
                ("tune radio", "Tune the tactile radio."),
                ("unlock hatch", "Open the hatch after the broadcast."),
                ("request hint", "Use an operator hint."),
                ("go table", "Return to the fold table."),
                ("go hatch", "Exit if the hatch is unlocked."),
            ]),
            "hatch-exit" => hints(&[
                ("look", "Show the hatch exit."),
                ("go radio", "Return to the radio nook."),
            ]),
            _ => Vec::new(),
        }
    }

    fn visual_nodes(&self, current_room: &str) -> Vec<MuddleVisualNode> {
        let mut children = vec![
            MuddleVisualNode::sprite(
                "silverstream-trailer",
                "Silverstream trailer",
                "sprites/amaze/silverstream-trailer.png",
                "A compact Silverstream trailer escape-room cutaway.",
            )
            .with_layer(0)
            .with_rect(0, 0, 12, 6),
            MuddleVisualNode::text("current-room-label", "Current room", current_room)
                .with_layer(30)
                .with_rect(1, 0, 4, 1),
            amaze_room_token("route-token", "Route rail", "route-rail", current_room, 1),
            amaze_room_token(
                "breaker-token",
                "Breaker panel",
                "breaker-panel",
                current_room,
                2,
            ),
            amaze_room_token(
                "galley-token",
                "Galley cabinet",
                "galley-cabinet",
                current_room,
                3,
            ),
            amaze_room_token("table-token", "Fold table", "fold-table", current_room, 4),
            amaze_room_token("radio-token", "Radio nook", "radio-nook", current_room, 5),
            amaze_room_token("hatch-token", "Hatch exit", "hatch-exit", current_room, 6),
        ];

        if self.state.route_planned {
            children.push(
                MuddleVisualNode::sprite(
                    "route-solved-badge",
                    "Route ordered",
                    "sprites/amaze/route-postcards.png",
                    "Ordered postcard route badge.",
                )
                .with_layer(20)
                .with_rect(1, 6, 2, 1)
                .with_frame("ordered"),
            );
        }
        if self.state.breakers_set {
            children.push(
                MuddleVisualNode::sprite(
                    "breaker-solved-badge",
                    "Breakers set",
                    "sprites/amaze/breaker-panel.png",
                    "Low-voltage breaker panel solved badge.",
                )
                .with_layer(20)
                .with_rect(3, 6, 2, 1)
                .with_frame("set"),
            );
        }
        if self.state.galley_sorted {
            children.push(
                MuddleVisualNode::sprite(
                    "galley-solved-badge",
                    "Galley sorted",
                    "sprites/amaze/galley-cabinet.png",
                    "Sorted galley object reveal badge.",
                )
                .with_layer(20)
                .with_rect(5, 6, 2, 1)
                .with_frame("frequency"),
            );
        }
        if self.state.table_aligned {
            children.push(
                MuddleVisualNode::sprite(
                    "table-solved-badge",
                    "Table aligned",
                    "sprites/amaze/fold-table.png",
                    "Fold table bearing badge.",
                )
                .with_layer(20)
                .with_rect(7, 6, 2, 1)
                .with_frame("bearing"),
            );
        }
        if self.state.broadcast_tuned {
            children.push(
                MuddleVisualNode::sprite(
                    "radio-solved-badge",
                    "Radio broadcast",
                    "sprites/amaze/radio-dial.png",
                    "Tuned radio broadcast badge.",
                )
                .with_layer(20)
                .with_rect(9, 6, 2, 1)
                .with_frame("broadcast")
                .with_animation("pulse"),
            );
        }

        vec![MuddleVisualNode::group(
            "silverstream-scene",
            "Silverstream scene",
            children,
        )]
    }

    fn export_checkpoint(&self) -> Option<String> {
        Some(format!(
            "route_planned={};breakers_set={};galley_sorted={};table_aligned={};broadcast_tuned={};hatch_unlocked={};hints_used={}",
            self.state.route_planned,
            self.state.breakers_set,
            self.state.galley_sorted,
            self.state.table_aligned,
            self.state.broadcast_tuned,
            self.state.hatch_unlocked,
            self.state.hints_used
        ))
    }

    fn import_checkpoint(&mut self, checkpoint: &str) -> Result<(), MuddleError> {
        let mut route_planned = None;
        let mut breakers_set = None;
        let mut galley_sorted = None;
        let mut table_aligned = None;
        let mut broadcast_tuned = None;
        let mut hatch_unlocked = None;
        let mut hints_used = None;

        for part in checkpoint.split(';') {
            let (key, value) =
                part.split_once('=')
                    .ok_or_else(|| MuddleError::InvalidHostCheckpoint {
                        message: format!("malformed checkpoint field `{part}`"),
                    })?;
            match key {
                "route_planned" => route_planned = Some(parse_checkpoint_bool(key, value)?),
                "breakers_set" => breakers_set = Some(parse_checkpoint_bool(key, value)?),
                "galley_sorted" => galley_sorted = Some(parse_checkpoint_bool(key, value)?),
                "table_aligned" => table_aligned = Some(parse_checkpoint_bool(key, value)?),
                "broadcast_tuned" => broadcast_tuned = Some(parse_checkpoint_bool(key, value)?),
                "hatch_unlocked" => hatch_unlocked = Some(parse_checkpoint_bool(key, value)?),
                "clue_found" => {
                    route_planned = Some(parse_checkpoint_bool(key, value)?);
                }
                "signal_aligned" => {
                    let value = parse_checkpoint_bool(key, value)?;
                    galley_sorted = Some(value);
                    table_aligned = Some(value);
                    broadcast_tuned = Some(value);
                }
                "hints_used" => {
                    hints_used = Some(value.parse::<u8>().map_err(|_| {
                        MuddleError::InvalidHostCheckpoint {
                            message: format!("invalid hints_used checkpoint field `{value}`"),
                        }
                    })?);
                }
                _ => {
                    return Err(MuddleError::InvalidHostCheckpoint {
                        message: format!("unknown checkpoint field `{key}`"),
                    });
                }
            }
        }

        self.state = AmazeSilverstreamMuddleState {
            route_planned: route_planned.ok_or_else(|| MuddleError::InvalidHostCheckpoint {
                message: "missing route_planned checkpoint field".to_string(),
            })?,
            breakers_set: breakers_set.unwrap_or_else(|| route_planned.unwrap_or(false)),
            galley_sorted: galley_sorted.ok_or_else(|| MuddleError::InvalidHostCheckpoint {
                message: "missing galley_sorted checkpoint field".to_string(),
            })?,
            table_aligned: table_aligned.ok_or_else(|| MuddleError::InvalidHostCheckpoint {
                message: "missing table_aligned checkpoint field".to_string(),
            })?,
            broadcast_tuned: broadcast_tuned.ok_or_else(|| MuddleError::InvalidHostCheckpoint {
                message: "missing broadcast_tuned checkpoint field".to_string(),
            })?,
            hatch_unlocked: hatch_unlocked.ok_or_else(|| MuddleError::InvalidHostCheckpoint {
                message: "missing hatch_unlocked checkpoint field".to_string(),
            })?,
            hints_used: hints_used.ok_or_else(|| MuddleError::InvalidHostCheckpoint {
                message: "missing hints_used checkpoint field".to_string(),
            })?,
        };
        Ok(())
    }

    fn handle_command(
        &mut self,
        room_id: &str,
        command: &MuddleCommand,
    ) -> Result<MuddleCommandOutcome, MuddleError> {
        match (room_id, command.normalized().as_str()) {
            (_, "look") | (_, "status") => self.look(room_id),
            ("silverstream-entry", "go route") => Ok(MuddleCommandOutcome::move_to(
                "You step up to the route rail and postcard map.",
                "route-rail",
            )),
            ("route-rail", "sort postcards") => {
                self.state.route_planned = true;
                Ok(MuddleCommandOutcome::stay(
                    "You order the six postcards against the mounted route map. The breaker labels now read as a sequence.",
                ))
            }
            (_, "sort postcards") if !self.state.route_planned => {
                self.state.route_planned = true;
                Ok(MuddleCommandOutcome::stay(
                    "You backtrack to the route rail long enough to order the postcards. The breaker sequence is now readable.",
                ))
            }
            (_, "sort postcards") => Ok(MuddleCommandOutcome::stay(
                "The postcard route is already ordered and ready for the breaker panel.",
            )),
            ("route-rail", "request hint") => {
                self.state.hints_used += 1;
                Ok(MuddleCommandOutcome::stay(
                    "Operator hint: match postcard mileage plaques to the public route silhouettes.",
                ))
            }
            ("route-rail", "go entry") => Ok(MuddleCommandOutcome::move_to(
                "You return to the Silverstream entry.",
                "silverstream-entry",
            )),
            ("route-rail", "go breaker") => Ok(MuddleCommandOutcome::move_to(
                "You move to the low-voltage breaker panel.",
                "breaker-panel",
            )),
            ("breaker-panel", "set breakers") if !self.state.route_planned => {
                Ok(MuddleCommandOutcome::stay(
                    "The switches need a route order. Sort the postcards before setting breakers.",
                ))
            }
            ("breaker-panel", "set breakers") => {
                self.state.breakers_set = true;
                Ok(MuddleCommandOutcome::stay(
                    "The prop breakers click into route order. A safe reveal light wakes in the galley.",
                ))
            }
            ("breaker-panel", "request hint") => {
                self.state.hints_used += 1;
                Ok(MuddleCommandOutcome::stay(
                    "Operator hint: use the postcard order left-to-right on the breaker labels.",
                ))
            }
            ("breaker-panel", "go route") => Ok(MuddleCommandOutcome::move_to(
                "You return to the route rail.",
                "route-rail",
            )),
            ("breaker-panel", "go breaker") => Ok(MuddleCommandOutcome::stay(
                "You are already at the breaker panel.",
            )),
            ("breaker-panel", "go galley") => Ok(MuddleCommandOutcome::move_to(
                "You move to the galley cabinet and counted object tray.",
                "galley-cabinet",
            )),
            ("galley-cabinet", "sort galley") if !self.state.breakers_set => {
                Ok(MuddleCommandOutcome::stay(
                    "The cabinet is dark. Set the breakers before sorting objects.",
                ))
            }
            ("galley-cabinet", "sort galley") => {
                self.state.galley_sorted = true;
                Ok(MuddleCommandOutcome::stay(
                    "You return each chunky galley object to its silhouette. The frequency mark appears with a printed backup label.",
                ))
            }
            ("galley-cabinet", "request hint") => {
                self.state.hints_used += 1;
                Ok(MuddleCommandOutcome::stay(
                    "Operator hint: count the mug, tin plate, spice jar, towel tag, ticket stub, and key fob before reading the reveal.",
                ))
            }
            ("galley-cabinet", "go breaker") => Ok(MuddleCommandOutcome::move_to(
                "You return to the breaker panel.",
                "breaker-panel",
            )),
            ("galley-cabinet", "go table") => Ok(MuddleCommandOutcome::move_to(
                "You move to the fold-down transform table.",
                "fold-table",
            )),
            ("fold-table", "align table") if !self.state.galley_sorted => {
                Ok(MuddleCommandOutcome::stay(
                    "You need the galley frequency before the table bearing makes sense.",
                ))
            }
            ("fold-table", "align table") => {
                self.state.table_aligned = true;
                Ok(MuddleCommandOutcome::stay(
                    "The table rotates to its safe stop and exposes a compass bearing for the radio.",
                ))
            }
            ("fold-table", "request hint") => {
                self.state.hints_used += 1;
                Ok(MuddleCommandOutcome::stay(
                    "Operator hint: rotate only to the marked detent; the bearing is visual, not force-based.",
                ))
            }
            ("fold-table", "go galley") => Ok(MuddleCommandOutcome::move_to(
                "You return to the galley cabinet.",
                "galley-cabinet",
            )),
            ("fold-table", "go radio") => Ok(MuddleCommandOutcome::move_to(
                "You move to the radio nook.",
                "radio-nook",
            )),
            ("radio-nook", "tune radio") if !self.state.table_aligned => {
                Ok(MuddleCommandOutcome::stay(
                    "The radio stays in static until the table bearing is exposed.",
                ))
            }
            ("radio-nook", "tune radio") => {
                self.state.broadcast_tuned = true;
                Ok(MuddleCommandOutcome::stay(
                    "The tactile radio locks onto Silverstream. A final broadcast confirms the escape route.",
                ))
            }
            ("radio-nook", "unlock hatch") if !self.state.broadcast_tuned => Ok(
                MuddleCommandOutcome::stay("The hatch interlock waits for the final broadcast."),
            ),
            ("radio-nook", "unlock hatch") => {
                self.state.hatch_unlocked = true;
                Ok(MuddleCommandOutcome::stay(
                    "The hatch unlocks with a clean resettable latch motion.",
                ))
            }
            ("radio-nook", "request hint") => {
                self.state.hints_used += 1;
                Ok(MuddleCommandOutcome::stay(
                    "Operator hint: combine the printed frequency and table bearing on the radio dial.",
                ))
            }
            ("radio-nook", "go hatch") if self.state.hatch_unlocked => {
                Ok(MuddleCommandOutcome::move_to(
                    "You open the hatch and exit the Silverstream room.",
                    "hatch-exit",
                ))
            }
            ("radio-nook", "go hatch") => Ok(MuddleCommandOutcome::stay(
                "The hatch is still locked. Solve the receiver sequence first.",
            )),
            (_, "go hatch") if self.state.hatch_unlocked => Ok(MuddleCommandOutcome::move_to(
                "You follow the solved route to the open hatch and exit the Silverstream room.",
                "hatch-exit",
            )),
            (_, "go hatch") => Ok(MuddleCommandOutcome::stay(
                "The hatch route is visible, but it needs the route, breakers, galley, table, and radio solved first.",
            )),
            ("radio-nook", "go table") => Ok(MuddleCommandOutcome::move_to(
                "You return to the fold table.",
                "fold-table",
            )),
            ("hatch-exit", "go radio") => Ok(MuddleCommandOutcome::move_to(
                "You return to the radio nook for reset review.",
                "radio-nook",
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

fn amaze_room_token(
    id: &str,
    label: &str,
    room_id: &str,
    current_room: &str,
    order: i32,
) -> MuddleVisualNode {
    let frame = if current_room == room_id {
        "active"
    } else {
        "idle"
    };
    MuddleVisualNode::sprite(
        id,
        label,
        format!("sprites/amaze/{room_id}.png"),
        format!("{label} room token"),
    )
    .with_layer(10)
    .with_rect(order * 2 - 1, 2, 1, 1)
    .with_frame(frame)
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

fn parse_checkpoint_bool(key: &str, value: &str) -> Result<bool, MuddleError> {
    match value {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(MuddleError::InvalidHostCheckpoint {
            message: format!("invalid boolean checkpoint field `{key}={value}`"),
        }),
    }
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
        assert!(surface.rooms.iter().any(|room| room.id == "radio-nook"));
        assert!(surface
            .commands
            .iter()
            .any(|command| command.command == "tune radio"));
    }

    #[test]
    fn exposes_prism_vault_muddle_surface() {
        let surface = prism_vault_muddle_surface();

        assert_eq!(surface.host_name, "amaze-prism-vault");
        assert_eq!(surface.start_room, "prism-entry");
        assert!(surface.rooms.iter().any(|room| room.id == "vault-door"));
        assert!(surface
            .commands
            .iter()
            .any(|command| command.command == "unlock vault"));
    }

    #[test]
    fn product_owned_muddle_host_plays_silverstream_path() {
        let mut host = silverstream_muddle_host();
        let mut session = MuddleSession::for_host(&host).expect("host has start room");

        for command in [
            "go route",
            "sort postcards",
            "go breaker",
            "set breakers",
            "go galley",
            "sort galley",
            "go table",
            "align table",
            "go radio",
            "tune radio",
            "unlock hatch",
            "go hatch",
        ] {
            session
                .play_turn(&mut host, MuddleCommand::parse(command))
                .expect("command plays");
        }

        assert_eq!(session.current_room, "hatch-exit");
        assert!(host.state().hatch_unlocked);
        assert_eq!(host.resource_panel()[5].value, "unlocked");
        assert!(host
            .inventory_panel()
            .iter()
            .any(|item| item.label == "hatch reset key"));
    }

    #[test]
    fn product_owned_muddle_host_plays_prism_vault_path() {
        let mut host = prism_vault_muddle_host();
        let mut session = MuddleSession::for_host(&host).expect("host has start room");

        for command in [
            "go lens",
            "align lens",
            "go color",
            "mix color",
            "go mirrors",
            "set mirrors",
            "go vault",
            "unlock vault",
            "go exit",
        ] {
            session
                .play_turn(&mut host, MuddleCommand::parse(command))
                .expect("command plays");
        }

        assert_eq!(session.current_room, "garden-exit");
        assert!(host.state().exit_open);
        assert_eq!(host.resource_panel()[4].value, "escaped");
        assert!(host
            .inventory_panel()
            .iter()
            .any(|item| item.label == "prism reset key"));
    }

    #[test]
    fn product_owned_muddle_host_emits_visual_scene_nodes() {
        let mut host = silverstream_muddle_host();
        let mut session = MuddleSession::for_host(&host).expect("host has start room");
        session
            .play_turn(&mut host, MuddleCommand::parse("go route"))
            .expect("entry moves to route rail");
        session
            .play_turn(&mut host, MuddleCommand::parse("sort postcards"))
            .expect("route can be planned");

        let visuals = host.visual_nodes(&session.current_room);
        let scene = visuals.first().expect("scene is emitted");
        assert_eq!(scene.id, "silverstream-scene");
        assert!(scene.children.iter().any(|node| node.id == "route-token"
            && node
                .sprite
                .as_ref()
                .and_then(|sprite| sprite.frame.as_deref())
                == Some("active")));
        assert!(scene
            .children
            .iter()
            .any(|node| node.id == "route-solved-badge"));
    }

    #[test]
    fn product_owned_muddle_host_recovers_from_friction_path() {
        let mut host = silverstream_muddle_host();
        let mut session = MuddleSession::for_host(&host).expect("host has start room");
        for command in [
            "go hatch",
            "go route",
            "go breaker",
            "set breakers",
            "sort postcards",
            "go breaker",
            "set breakers",
            "go galley",
            "sort galley",
            "go table",
            "align table",
            "go radio",
            "unlock hatch",
            "tune radio",
            "unlock hatch",
            "go hatch",
        ] {
            session
                .play_turn(&mut host, MuddleCommand::parse(command))
                .expect("friction path remains recoverable");
        }

        assert_eq!(session.current_room, "hatch-exit");
        assert!(host.state().hatch_unlocked);
    }

    #[test]
    fn product_owned_prism_vault_recovers_from_friction_path() {
        let mut host = prism_vault_muddle_host();
        let mut session = MuddleSession::for_host(&host).expect("host has start room");
        for command in [
            "go exit",
            "go lens",
            "go color",
            "mix color",
            "go lens",
            "align lens",
            "go color",
            "mix color",
            "go mirrors",
            "set mirrors",
            "go vault",
            "go exit",
            "unlock vault",
            "go exit",
        ] {
            session
                .play_turn(&mut host, MuddleCommand::parse(command))
                .expect("friction path remains recoverable");
        }

        assert_eq!(session.current_room, "garden-exit");
        assert!(host.state().exit_open);
    }

    #[test]
    fn product_owned_muddle_snapshot_carries_visual_scene_nodes() {
        let host = silverstream_muddle_host();
        let session = MuddleSession::for_host(&host).expect("host has start room");
        let snapshot = session.client_snapshot(
            &host,
            muddle_core::MuddleClientInfo {
                host: "amaze-silverstream".to_string(),
                description: "AMAZE Silverstream".to_string(),
                suggested_commands: "look".to_string(),
            },
            "Ready.",
        );

        assert!(snapshot
            .visual_nodes
            .iter()
            .any(|node| node.id == "silverstream-scene"));
        assert!(snapshot
            .controls
            .iter()
            .any(|control| control.id == "visuals"));
    }

    #[test]
    fn product_owned_prism_vault_snapshot_carries_visual_scene_nodes() {
        let host = prism_vault_muddle_host();
        let session = MuddleSession::for_host(&host).expect("host has start room");
        let snapshot = session.client_snapshot(
            &host,
            muddle_core::MuddleClientInfo {
                host: "amaze-prism-vault".to_string(),
                description: "AMAZE Prism Vault".to_string(),
                suggested_commands: "look".to_string(),
            },
            "Ready.",
        );

        assert!(snapshot
            .visual_nodes
            .iter()
            .any(|node| node.id == "prism-vault-scene"));
        assert!(snapshot
            .controls
            .iter()
            .any(|control| control.id == "visuals"));
    }

    #[test]
    fn product_owned_muddle_host_resumes_from_command_save() {
        let mut host = silverstream_muddle_host();
        let mut session = MuddleSession::for_host(&host).expect("host has start room");
        session
            .play_turn(&mut host, MuddleCommand::parse("go route"))
            .expect("entry moves to route rail");
        session
            .play_turn(&mut host, MuddleCommand::parse("sort postcards"))
            .expect("route can be planned");

        let save = session.save();
        let mut resumed_host = silverstream_muddle_host();
        let mut resumed = MuddleSession::resume_for_host(&mut resumed_host, &save)
            .expect("session resumes by replaying commands");
        resumed
            .play_turn(&mut resumed_host, MuddleCommand::parse("go breaker"))
            .expect("resumed route can move to breakers");
        resumed
            .play_turn(&mut resumed_host, MuddleCommand::parse("set breakers"))
            .expect("resumed route state permits breaker sequence");

        assert_eq!(resumed.current_room, "breaker-panel");
        assert!(resumed_host.state().route_planned);
        assert!(resumed_host.state().breakers_set);
    }

    #[test]
    fn product_owned_muddle_host_resumes_from_checkpoint_save() {
        let mut host = silverstream_muddle_host();
        let mut session = MuddleSession::for_host(&host).expect("host has start room");
        for command in [
            "go route",
            "sort postcards",
            "request hint",
            "go breaker",
            "set breakers",
            "go galley",
            "sort galley",
            "go table",
            "align table",
            "go radio",
            "tune radio",
            "unlock hatch",
        ] {
            session
                .play_turn(&mut host, MuddleCommand::parse(command))
                .expect("command plays");
        }

        let save = session.save_for_host(&host);
        assert_eq!(
            save.host_checkpoint.as_deref(),
            Some("route_planned=true;breakers_set=true;galley_sorted=true;table_aligned=true;broadcast_tuned=true;hatch_unlocked=true;hints_used=1")
        );

        let checkpoint_only_save = muddle_core::MuddleSessionSave {
            current_room: "radio-nook".to_string(),
            commands: vec![
                "go route".to_string(),
                "go breaker".to_string(),
                "go galley".to_string(),
                "go table".to_string(),
                "go radio".to_string(),
            ],
            host_checkpoint: save.host_checkpoint,
        };
        let mut resumed_host = silverstream_muddle_host();
        let mut resumed = MuddleSession::resume_for_host(&mut resumed_host, &checkpoint_only_save)
            .expect("session resumes from host checkpoint");
        resumed
            .play_turn(&mut resumed_host, MuddleCommand::parse("go hatch"))
            .expect("checkpoint restored unlocked hatch");

        assert_eq!(resumed.current_room, "hatch-exit");
        assert!(resumed_host.state().hatch_unlocked);
        assert_eq!(resumed_host.state().hints_used, 1);
    }

    #[test]
    fn product_owned_prism_vault_resumes_from_checkpoint_save() {
        let mut host = prism_vault_muddle_host();
        let mut session = MuddleSession::for_host(&host).expect("host has start room");
        for command in [
            "go lens",
            "request hint",
            "align lens",
            "go color",
            "mix color",
            "go mirrors",
            "set mirrors",
            "go vault",
            "unlock vault",
        ] {
            session
                .play_turn(&mut host, MuddleCommand::parse(command))
                .expect("command plays");
        }

        let save = session.save_for_host(&host);
        assert_eq!(
            save.host_checkpoint.as_deref(),
            Some("lens_aligned=true;color_mixed=true;mirrors_set=true;vault_unlocked=true;exit_open=true;hints_used=1")
        );

        let checkpoint_only_save = muddle_core::MuddleSessionSave {
            current_room: "vault-door".to_string(),
            commands: vec![
                "go lens".to_string(),
                "go color".to_string(),
                "go mirrors".to_string(),
                "go vault".to_string(),
            ],
            host_checkpoint: save.host_checkpoint,
        };
        let mut resumed_host = prism_vault_muddle_host();
        let mut resumed = MuddleSession::resume_for_host(&mut resumed_host, &checkpoint_only_save)
            .expect("session resumes from host checkpoint");
        resumed
            .play_turn(&mut resumed_host, MuddleCommand::parse("go exit"))
            .expect("checkpoint restored open exit");

        assert_eq!(resumed.current_room, "garden-exit");
        assert!(resumed_host.state().exit_open);
        assert_eq!(resumed_host.state().hints_used, 1);
    }
}
