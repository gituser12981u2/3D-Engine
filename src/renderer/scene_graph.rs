use super::{
    mesh::MeshStorage, render_queue::RenderQueue, Color, DrawCommandBuilder, RendererError,
};
use glam::{Mat4, Quat, Vec3};
use log::{debug, trace};

/// Node ID for the scene graph
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct NodeId(pub usize);

/// Represents a node in the scene graph.
pub struct SceneNode {
    /// The unique ID of this node
    pub id: NodeId,
    /// The parent node ID, if any
    pub parent: Option<NodeId>,
    /// The mesh ID associated with this node, if any
    pub mesh_id: Option<usize>,
    /// The local position relative to the parent
    pub position: Vec3,
    /// The local rotation relative to the parent
    pub rotation: Quat,
    /// The local scale relative to the parent
    pub scale: Vec3,
    /// The color of this node
    pub color: Color,
    /// Children of this node
    pub children: Vec<NodeId>,
    /// Whether this node is visible
    pub visible: bool,
}

impl SceneNode {
    /// Creates a new scene node
    pub fn new(id: NodeId) -> Self {
        Self {
            id,
            parent: None,
            mesh_id: None,
            position: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
            color: Color::new(1.0, 1.0, 1.0, 1.0),
            children: Vec::new(),
            visible: true,
        }
    }

    /// Calculate the local transformation matrix for this node
    pub fn local_transform(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.position)
    }
}

/// Manages a hierarchical scene graph of objects
pub struct SceneGraph {
    /// All nodes in the scene graph
    nodes: Vec<SceneNode>,
    /// Root nodes
    root_nodes: Vec<NodeId>,
}

impl SceneGraph {
    pub fn new() -> Self {
        debug!("Creating new SceneGraph");
        Self {
            nodes: Vec::new(),
            root_nodes: Vec::new(),
        }
    }

    /// Creates a new node in the scene graph
    pub fn create_node(&mut self) -> NodeId {
        let id = NodeId(self.nodes.len());
        let node = SceneNode::new(id);
        self.nodes.push(node);
        self.root_nodes.push(id);
        debug!("Created new node with ID {:?}", id);
        id
    }

    /// Creates a new node with a mesh
    pub fn create_mesh_node(&mut self, mesh_id: usize) -> NodeId {
        let id = self.create_node();
        self.nodes[id.0].mesh_id = Some(mesh_id);
        id
    }

    /// Sets teh parent of a node
    pub fn set_parent(
        &mut self,
        node_id: NodeId,
        parent_id: Option<NodeId>,
    ) -> Result<(), RendererError> {
        // Ensure node exists
        if node_id.0 >= self.nodes.len() {
            return Err(RendererError::InvalidMeshId);
        }

        // If the node already has a parent, remove it from the parent's children
        if let Some(old_parent) = self.nodes[node_id.0].parent {
            if let Some(parent_idx) = self.nodes.iter().position(|n| n.id == old_parent) {
                let children = &mut self.nodes[parent_idx].children;
                if let Some(idx) = children.iter().position(|&id| id == node_id) {
                    children.swap_remove(idx);
                }
            }
        } else {
            // If it was a root node, remove from root_nodes
            if let Some(idx) = self.root_nodes.iter().position(|&id| id == node_id) {
                self.root_nodes.swap_remove(idx);
            }
        }

        // Update parent reference
        self.nodes[node_id.0].parent = parent_id;

        // Add to new parent's children if provided, or add to root_nodes if None
        if let Some(parent_id) = parent_id {
            if parent_id.0 >= self.nodes.len() {
                return Err(RendererError::InvalidMeshId);
            }

            // Prevent circular reference
            if self.would_create_cycle(node_id, parent_id) {
                self.nodes[node_id.0].parent = None;
                self.root_nodes.push(node_id);
                return Err(RendererError::InvalidMeshId);
            }

            self.nodes[parent_id.0].children.push(node_id);
        } else {
            // If no parent, add to root nodes
            self.root_nodes.push(node_id);
        }

        trace!("Set parent of node {:?} to {:?}", node_id, parent_id);
        Ok(())
    }

    /// Checks if setting a parent would create a cycle in the graph
    fn would_create_cycle(&self, node_id: NodeId, parent_id: NodeId) -> bool {
        // If the node that is to be set as parent is the same as the node, then it is a cycle
        if node_id == parent_id {
            return true;
        }

        // Check if the potential parent is a child of the node
        let mut to_check = vec![node_id];
        while let Some(current) = to_check.pop() {
            let children = &self.nodes[current.0].children;
            for &child in children {
                if child == parent_id {
                    return true;
                }
                to_check.push(child);
            }
        }

        false
    }

    /// Sets the affine transformation of a node
    pub fn set_transform(
        &mut self,
        node_id: NodeId,
        position: Vec3,
        rotation: Quat,
        scale: Vec3,
    ) -> Result<(), RendererError> {
        if node_id.0 >= self.nodes.len() {
            return Err(RendererError::InvalidMeshId);
        }

        let node = &mut self.nodes[node_id.0];
        node.position = position;
        node.rotation = rotation;
        node.scale = scale;

        trace!("Set transform of node {:?}", node_id);
        Ok(())
    }

    /// Sets the mesh ID for a node
    pub fn set_mesh(
        &mut self,
        node_id: NodeId,
        mesh_id: Option<usize>,
    ) -> Result<(), RendererError> {
        if node_id.0 >= self.nodes.len() {
            return Err(RendererError::InvalidMeshId);
        }

        self.nodes[node_id.0].mesh_id = mesh_id;
        trace!("Set mesh of node {:?} to {:?}", node_id, mesh_id);
        Ok(())
    }

    /// Sets the color for a node
    pub fn set_color(&mut self, node_id: NodeId, color: Color) -> Result<(), RendererError> {
        if node_id.0 >= self.nodes.len() {
            return Err(RendererError::InvalidMeshId);
        }

        self.nodes[node_id.0].color = color;
        trace!("Set color of node {:?} to {:?}", node_id, color);
        Ok(())
    }

    /// Sets teh visibility of a node and its children
    pub fn set_visible(&mut self, node_id: NodeId, visible: bool) -> Result<(), RendererError> {
        if node_id.0 >= self.nodes.len() {
            return Err(RendererError::InvalidMeshId);
        }

        self.nodes[node_id.0].visible = visible;
        trace!("Set visibility of node {:?} tp {visible}", node_id);
        Ok(())
    }

    /// Calculate the world transform for a node
    pub fn get_world_transform(&self, node_id: NodeId) -> Result<Mat4, RendererError> {
        if node_id.0 >= self.nodes.len() {
            return Err(RendererError::InvalidMeshId);
        }

        let mut transform = self.nodes[node_id.0].local_transform();
        let mut current_node = &self.nodes[node_id.0];

        // Apply parent transforms
        while let Some(parent_id) = current_node.parent {
            let parent = &self.nodes[parent_id.0];
            transform = parent.local_transform() * transform;
            current_node = parent;
        }

        Ok(transform)
    }

    /// Removes a node and optionally all its children
    pub fn remove_node(&mut self, node_id: NodeId, recursive: bool) -> Result<(), RendererError> {
        if node_id.0 >= self.nodes.len() {
            return Err(RendererError::InvalidMeshId);
        }

        // First collect nodes to remove
        let mut to_remove = vec![node_id];

        if recursive {
            let mut i = 0;
            while i < to_remove.len() {
                let children = self.nodes[to_remove[i].0].children.clone();
                i += 1;
            }
        } else {
            // Reparent children to this node's parent
            let parent = self.nodes[node_id.0].parent;
            for &child in &self.nodes[node_id.0].children.clone() {
                self.set_parent(child, parent)?;
            }
        }

        // Remove from parent's children list
        if let Some(parent_id) = self.nodes[node_id.0].parent {
            let parent = &mut self.nodes[parent_id.0];
            if let Some(idx) = parent.children.iter().position(|&id| id == node_id) {
                parent.children.swap_remove(idx);
            }
        } else {
            // Remove from root nodes
            if let Some(idx) = self.root_nodes.iter().position(|&id| id == node_id) {
                self.root_nodes.swap_remove(idx);
            }
        }

        // Mark the nodes as "removed" by setting mesh_id to None
        for &id in &to_remove {
            self.nodes[id.0].visible = false;
            self.nodes[id.0].mesh_id = None;
            self.nodes[id.0].children.clear();
        }

        debug!("Removed node {:?} (recursive: {})", node_id, recursive);
        Ok(())
    }

    /// Generate draw commands for the entire scene
    pub fn generate_draw_commands(
        &self,
        render_queue: &mut RenderQueue,
        mesh_storage: &MeshStorage,
    ) -> Result<(), RendererError> {
        debug!("Generating draw commands for scene graph");

        // Process all root nodes
        for &root_id in &self.root_nodes {
            self.process_node(root_id, Mat4::IDENTITY, render_queue, mesh_storage)?;
        }

        Ok(())
    }

    /// Recursively process a node and its children
    fn process_node(
        &self,
        node_id: NodeId,
        parent_transform: Mat4,
        render_queue: &mut RenderQueue,
        mesh_storage: &MeshStorage,
    ) -> Result<(), RendererError> {
        let node = &self.nodes[node_id.0];

        if !node.visible {
            return Ok(());
        }

        // Calculate combined transform
        let local_transform = node.local_transform();
        let world_transform = parent_transform * local_transform;

        // if has mesh, add draw command
        if let Some(mesh_id) = node.mesh_id {
            if let Some(mesh) = mesh_storage.get_mesh(mesh_id) {
                let command = DrawCommandBuilder::new_mesh(mesh_id)
                    .with_transform(world_transform)
                    .build();

                render_queue.add_draw_command(command);
                trace!(
                    "Added draw command for node {:?} with mesh {:?}",
                    node_id,
                    mesh_id
                );
            }
        }

        // Process children
        for &child_id in &node.children {
            self.process_node(child_id, world_transform, render_queue, mesh_storage)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use glam::{Quat, Vec3};

    use crate::{
        renderer::{
            common::Vertex,
            mesh::{Mesh, MeshStorage},
            render_queue::{DrawCommand, RenderQueue},
            scene_graph::SceneGraph,
        },
        MeshBuilder,
    };

    fn create_test_mesh() -> Mesh {
        let vertices = vec![
            Vertex {
                position: [0.0, 0.5, 0.0],
                color: [1.0, 0.0, 0.0, 1.0],
            },
            Vertex {
                position: [-0.5, -0.5, 0.0],
                color: [0.0, 1.0, 0.0, 1.0],
            },
            Vertex {
                position: [0.5, -0.5, 0.0],
                color: [0.0, 0.0, 1.0, 1.0],
            },
        ];

        let mesh_builder =
            MeshBuilder::new(vertices, crate::renderer::common::PrimitiveType::Triangle);
        Mesh::new(mesh_builder)
    }

    #[test]
    fn test_create_node() {
        let mut graph = SceneGraph::new();
        let node_id = graph.create_node();

        assert_eq!(node_id.0, 0);
        assert_eq!(graph.nodes.len(), 1);
        assert_eq!(graph.root_nodes.len(), 1);
        assert_eq!(graph.root_nodes[0], node_id);
    }

    #[test]
    fn test_parent_child_relationship() {
        let mut graph = SceneGraph::new();
        let parent_id = graph.create_node();
        let child_id = graph.create_node();

        assert_eq!(graph.root_nodes.len(), 2);

        graph.set_parent(child_id, Some(parent_id)).unwrap();

        assert_eq!(graph.nodes[child_id.0].parent, Some(parent_id));
        assert!(graph.nodes[parent_id.0].children.contains(&child_id));
        assert_eq!(graph.root_nodes.len(), 1);
        assert_eq!(graph.root_nodes[0], parent_id);
    }

    #[test]
    fn test_world_transform() {
        let mut graph = SceneGraph::new();
        let parent_id = graph.create_node();
        let child_id = graph.create_node();

        // Parent at (1, 0, 0) with identity rotation and scale
        graph
            .set_transform(
                parent_id,
                Vec3::new(1.0, 0.0, 0.0),
                Quat::IDENTITY,
                Vec3::ONE,
            )
            .unwrap();

        // Child at (0, 1, 0) relative to parent
        graph
            .set_transform(
                child_id,
                Vec3::new(0.0, 1.0, 0.0),
                Quat::IDENTITY,
                Vec3::ONE,
            )
            .unwrap();

        // Set parent-child relationship
        graph.set_parent(child_id, Some(parent_id)).unwrap();

        // World transform of child be at (1, 1, 0)
        let world_transform = graph.get_world_transform(child_id).unwrap();
        let world_pos = world_transform.transform_point3(Vec3::ZERO);

        assert!((world_pos - Vec3::new(1.0, 1.0, 0.0)).length() < 1e-15);
    }

    #[test]
    fn test_cycle_prevention() {
        let mut graph = SceneGraph::new();
        let node1 = graph.create_node();
        let node2 = graph.create_node();
        let node3 = graph.create_node();

        // Create a valid chain: node1 -> node2 -> node3
        graph.set_parent(node2, Some(node1)).unwrap();
        graph.set_parent(node3, Some(node2)).unwrap();

        // Try to create an invalid cycle: node1 -> node2 -> node3 -> node1
        let result = graph.set_parent(node1, Some(node3));

        // Should fail
        assert!(result.is_err());

        // Original structure should remain
        assert_eq!(graph.nodes[node1.0].parent, None);
        assert_eq!(graph.nodes[node2.0].parent, Some(node1));
        assert_eq!(graph.nodes[node3.0].parent, Some(node2));
    }

    #[test]
    fn test_generate_draw_commands() {
        let mut graph = SceneGraph::new();
        let mut mesh_storage = MeshStorage::new();
        let mut render_queue = RenderQueue::new();

        // Add a test mesh to storage
        let mesh = create_test_mesh();
        let mesh_id =
            mesh_storage.add_mesh(MeshBuilder::new(mesh.vertices.clone(), mesh.primitive_type));

        // Create a node with the mesh
        let node_id = graph.create_mesh_node(mesh_id);

        // Generate draw commands
        graph
            .generate_draw_commands(&mut render_queue, &mesh_storage)
            .unwrap();

        // Should have one draw command
        assert_eq!(render_queue.draw_commands.len(), 1);

        // Check it is the right type
        match &render_queue.draw_commands[0] {
            DrawCommand::Mesh {
                mesh_id: cmd_mesh_id,
                ..
            } => {
                assert_eq!(*cmd_mesh_id, mesh_id);
            }
            _ => panic!("Wrong draw command type"),
        }
    }
}
