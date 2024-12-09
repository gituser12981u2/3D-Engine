// use crate::common::vector3::PhysicsVector3;

// // Enum to represent different types of physical entities
// #[derive(Clone, Copy, Debug)]
// pub enum EntityType {
//     QuantumParticle,
//     SubatomicParticle,
//     Atom,
//     MolecularCloud,
//     Star,
//     Planet,
//     Galaxy,
//     DarkMatterHalo,
//     BlackHole,
// }

// // Struct to hold physical properties common to all entity types
// #[derive(Clone, Debug)]
// pub struct PhysicalProperties {
//     mass: f64,
//     position: PhysicsVector3,
//     velocity: PhysicsVector3,
//     charge: f64,
//     spin: f64,
// }

// // Struct to represent a physical entity
// #[derive(Clone, Debug)]
// pub struct Entity {
//     entity_type: EntityType,
//     properties: PhysicalProperties,
// }

// // Universal physics system
// pub struct UniversalPhysics {
//     entities: Vec<Entity>,
//     time_scale: f64,
//     space_scale: f64,
// }

// impl UniversalPhysics {
//     pub fn new() -> Self {
//         UniversalPhysics {
//             entities: Vec::new(),
//             time_scale: 1.0,
//             space_scale: 1.0,
//         }
//     }

//     pub fn add_entity(&mut self, entity_type: EntityType, properties: PhysicalProperties) -> usize {
//         let index = self.entities.len();
//         self.entities.push(Entity {
//             entity_type,
//             properties,
//         });
//         index
//     }

//     pub fn update(&mut self, dt: f64) {
//         self.update_quantum_scale(dt);
//         self.update_atomic_scale(dt);
//         self.update_stellar_scale(dt);
//         self.update_galactic_scale(dt);
//         self.update_cosmic_scale(dt);
//     }

//     fn update_quantum_scale(&mut self, dt: f64) {
//         // Implement quantum fluctuations, wave function collapse, etc.
//     }

//     fn update_atomic_scale(&mut self, dt: f64) {
//         // Implement atomic interactions, chemical reactions, etc.
//     }

//     fn update_stellar_scale(&mut self, dt: f64) {
//         // Implement stellar formation, planetary dynamics, etc.
//     }

//     fn update_galactic_scale(&mut self, dt: f64) {
//         // Implement galactic rotation, star cluster dynamics, etc.
//     }

//     fn update_cosmic_scale(&mut self, dt: f64) {
//         // Implement cosmic expansion, dark matter dynamics, etc.
//     }

//     pub fn apply_force(&mut self, index: usize, force: PhysicsVector3) {
//         let entity = &mut self.entities[index];
//         match entity.entity_type {
//             EntityType::QuantumParticle => {
//                 // Apply quantum forces
//             }
//             EntityType::SubatomicParticle | EntityType::Atom => {
//                 // Apply electromagnetic and nuclear forces
//             }
//             EntityType::MolecularCloud | EntityType::Star | EntityType::Planet => {
//                 // Apply gravitational forces
//                 entity.properties.velocity += force / entity.properties.mass * self.time_scale;
//             }
//             EntityType::Galaxy | EntityType::DarkMatterHalo | EntityType::BlackHole => {
//                 // Apply large-scale gravitational effects
//             }
//         }
//     }

//     pub fn set_time_scale(&mut self, scale: f64) {
//         self.time_scale = scale;
//     }

//     pub fn set_space_scale(&mut self, scale: f64) {
//         self.space_scale = scale;
//     }

//     // Getter methods
//     pub fn entity_position(&self, index: usize) -> PhysicsVector3 {
//         self.entities[index].properties.position
//     }
//     pub fn entity_velocity(&self, index: usize) -> PhysicsVector3 {
//         self.entities[index].properties.velocity
//     }
//     pub fn entity_mass(&self, index: usize) -> f64 {
//         self.entities[index].properties.mass
//     }
//     pub fn entity_count(&self) -> usize {
//         self.entities.len()
//     }
// }
