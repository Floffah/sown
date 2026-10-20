declare namespace SownAPI {
    export enum NodeType {
        Node3D,
        Mesh
    }

    type NodeID = string
    export interface Node {
        id: NodeID
        name: string
        kind: NodeType
    }
}

declare module "sown:ecs" {
    export function getNodeByID(id: SownAPI.NodeID): SownAPI.Node
}
