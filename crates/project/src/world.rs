// shortcut: world types are not wired up yet; remove this allowance when they are used.
#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct WorldSource {
    pub(crate) format_version: u32,
    #[allow(dead_code)]
    pub(crate) name: String,
    pub(crate) root: NodeSource,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct NodeSource {
    pub(crate) id: String,
    #[allow(dead_code)]
    pub(crate) name: String,
    #[serde(rename = "type")]
    pub(crate) kind: String,
    transform: Option<TransformSource>,
    mesh: Option<MeshSource>,
    collider: Option<ColliderSource>,
    #[serde(default)]
    pub(crate) behaviours: Vec<String>,
    pub(crate) scope: Option<Scope>,
    #[serde(default)]
    pub(crate) children: Vec<NodeSource>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum Scope {
    Replicated,
    Server,
    Client,
}

pub(crate) struct BehaviourAttachment<'source> {
    pub(crate) uri: &'source str,
    pub(crate) pointer: String,
    pub(crate) node_id: &'source str,
    pub(crate) scope: Scope,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct TransformSource {
    position: Option<[f64; 3]>,
    scale: Option<[f64; 3]>,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct MeshSource {
    primitive: String,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct ColliderSource {
    shape: String,
}
