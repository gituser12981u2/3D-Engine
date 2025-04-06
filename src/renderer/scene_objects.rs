use super::{render_core::Renderer, Color, NodeId, RendererError};
use glam::{Quat, Vec3};

/// High level API wrapper around a scene node
pub struct SceneObject {
    /// The node ID in the scene group
    node_id: NodeId,
}

impl SceneObject {
    /// Create a new object from an existing node ID
    pub(crate) fn from_node_id(node_id: NodeId) -> Self {
        Self { node_id }
    }

    /// Get the internal node ID
    pub fn node_id(&self) -> NodeId {
        self.node_id
    }

    /// Set the position of this object
    pub fn set_position(
        &self,
        renderer: &mut Renderer,
        position: Vec3,
    ) -> Result<(), RendererError> {
        // Get current transform
        let world_transform = renderer.get_node_world_transform(self.node_id)?;
        let (scale, rotation, _) = world_transform.to_scale_rotation_translation();

        // Update with new position
        renderer.set_node_transform(self.node_id, position, rotation, scale)
    }

    /// Set the rotation of this object
    pub fn set_rotation(
        &self,
        renderer: &mut crate::renderer::render_core::Renderer,
        rotation: Quat,
    ) -> Result<(), RendererError> {
        // Get current transform
        let world_transform = renderer.get_node_world_transform(self.node_id)?;
        let (scale, _, position) = world_transform.to_scale_rotation_translation();

        // Update with new rotation
        renderer.set_node_transform(self.node_id, position, rotation, scale)
    }

    /// Set the scale of this object
    pub fn set_scale(&self, renderer: &mut Renderer, scale: Vec3) -> Result<(), RendererError> {
        // Get current transform
        let world_transform = renderer.get_node_world_transform(self.node_id)?;
        let (_, rotation, position) = world_transform.to_scale_rotation_translation();

        // Update with new scale
        renderer.set_node_transform(self.node_id, position, rotation, scale)
    }

    pub fn set_transform(
        &self,
        renderer: &mut Renderer,
        position: Vec3,
        rotation: Quat,
        scale: Vec3,
    ) -> Result<(), RendererError> {
        renderer.set_node_transform(self.node_id, position, rotation, scale)
    }

    pub fn set_color(&self, renderer: &mut Renderer, color: Color) -> Result<(), RendererError> {
        renderer.set_node_color(self.node_id, color)
    }

    pub fn set_visible(&self, renderer: &mut Renderer, visible: bool) -> Result<(), RendererError> {
        renderer.set_node_visible(self.node_id, visible)
    }

    pub fn set_parent(
        &self,
        renderer: &mut Renderer,
        parent: Option<&SceneObject>,
    ) -> Result<(), RendererError> {
        renderer.set_node_parent(self.node_id, parent.map(|p| p.node_id))
    }
}
