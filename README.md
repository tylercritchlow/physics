Goals of this project:

- In the end, I should have a collection of 20-40 demos of the engine, not including "stress tests". i.e. only cool demos.
- bevy_ecs as the only library dependency, should build my own vector math functions, etc.

Blog Series: Building a 3D Physics Engine From Scratch

# Part 1: Rigid Body Dynamics

## Chapter 1: The Heartbeat - The Physics Loop & Time

What is a Game Loop? Introduction to the core update and render cycle.

The Problem with Time: Why using a variable frame rate can lead to unstable physics.

The Solution: A Fixed Timestep: Implementing a fixed-update loop using an "accumulator" for stable, deterministic simulations.

Our First "Body": Creating a data structure to hold 3D position, velocity, and acceleration using 3D vectors.

Goal: A sphere that moves based on its velocity, updated within a stable physics loop.

## Chapter 2: The Language of Motion - 3D Vector Math

Vectors in 3D: Representing position, velocity, force, and direction in 3D space (x, y, z).

Core Vector Operations: Addition, Subtraction, Scalar Multiplication, Magnitude, and Normalization in 3D.

The Dot Product: Our tool for calculating angles and projections.

The Cross Product: A new, essential tool for 3D physics used to calculate torque and surface normals.

Goal: A library of 3D vector functions that will be the foundation of our engine.

## Chapter 3: It's a Material World - Forces & Integration

Newton's Second Law in 3D: Applying forces (like gravity) to our 3D objects.

Integrators (Euler, Verlet): The methods used to update an object's position and velocity over time, now operating on 3D vectors.

Goal: Make our sphere respond to gravity and other applied forces, like a player pushing it.

## Chapter 4: The Big Question - 3D Collision Detection

Collision Primitives: Defining 3D shapes for our bodies: Spheres, Planes, and Axis-Aligned Bounding Boxes (AABBs).

Narrowphase Algorithms: Writing functions to detect overlap between pairs of 3D shapes:

Sphere vs. Sphere

Sphere vs. Plane

AABB vs. AABB

Sphere vs. AABB

Advanced Collision: An introduction to the Separating Axis Theorem (SAT) for detecting collisions between convex polyhedra (like boxes).

Broadphase (The Theory): Discussing why O(n^2) checks are too slow and introducing 3D concepts like Octrees or Bounding Volume Hierarchies (BVH).

Goal: A program that can reliably detect when any two 3D shapes are overlapping.

## Chapter 5: The Reaction - 3D Collision Resolution

The Collision Normal: Finding the 3D direction of the collision.

Impulse-Based Resolution: Applying an instantaneous change in velocity to resolve collisions.

Calculating the 3D Impulse: Using mass, velocity, and the 3D collision normal to calculate the change in velocities.

Adding "Bounciness" & Friction: Introducing restitution and friction to create more realistic surface interactions.

Goal: 3D objects that realistically bounce off of each other and slide with friction.

## Chapter 6: A World of Spin - 3D Rotational Physics

The Problem with 3D Rotation: Why simple Euler angles fail (gimbal lock).

The Solution: Quaternions: Introducing quaternions as a robust way to represent 3D orientation.

Angular Velocity & Torque: Representing these as 3D vectors.

Moment of Inertia in 3D: Introducing the Inertia Tensor, a 3x3 matrix that represents an object's resistance to rotation along different axes.

Updating the Resolution: Modifying our impulse calculations to affect angular velocity, using the inertia tensor and quaternions.

Goal: Objects that can spin and tumble realistically in 3D space.

## Chapter 7: Making Connections - 3D Constraints & Joints

What is a Constraint? A rule that removes degrees of freedom between rigid bodies in 3D space.

Building a Ball-and-Socket Joint: A constraint that allows free rotation around a single point.

Building a Hinge Joint: A constraint that allows rotation around a single axis, like a door hinge.

How They Work: A high-level look at the iterative solvers needed to enforce these complex 3D constraints.

Goal: The ability to create complex compound objects, like a pendulum or a ragdoll.

## Chapter 8: Making it Fast - Rigid Body Optimizations

Implementing a Broadphase: Choosing and implementing a 3D spatial partitioning scheme, like an Octree, to reduce collision checks.

Putting Bodies to Sleep: An essential optimization where objects at rest are temporarily removed from the simulation.

Other Tips & Tricks: Discussing memory management, data structures, and profiling in a 3D context.

Goal: A physics engine that can handle hundreds of 3D objects without slowing down.

# Part 2: Deformable Bodies & Fluids

## Chapter 9: Intro to Deformable Bodies - Mass-Spring Systems

From Rigid to Deformable: Shifting our thinking from single objects to particle systems.

Mass-Spring Systems: The most common approach: representing an object as a lattice of point masses connected by springs.

Types of Springs: Structural springs (for shape), shear springs (for resistance to twisting), and bend springs (for maintaining angles).

Collision with Soft Bodies: How to handle collisions and resolution for a deformable object made of many particles.

Goal: Create a wobbly, deformable cube that realistically interacts with the rigid bodies in our world.

## Chapter 10: Advanced Soft Bodies - Shape Matching & PBD

The Problem with Mass-Spring: Discussing the limitations, such as unnatural stretching and difficulty in tuning spring stiffness.

Position Based Dynamics (PBD): A modern, stable, and highly controllable alternative to springs. We'll learn to directly manipulate particle positions to satisfy constraints.

Implementing a Distance Constraint: The PBD equivalent of a spring.

Intro to Shape Matching: A powerful PBD technique where we force a cloud of particles to conform to its original "rest" shape, creating very robust and art-directable soft bodies.

Goal: A soft body that holds its volume and shape much more realistically than a simple mass-spring system.

## Chapter 11: Simulating Cloth

Cloth as a 2D Particle Grid: Setting up a grid of particles connected by mass-spring or PBD distance constraints.

Handling Self-Collision: Why cloth is a unique challenge and the basic strategies to prevent it from passing through itself.

Aerodynamic Forces: Simulating wind by applying forces based on the orientation of cloth triangles (using the cross product!).

Tearing and Cutting: A brief look at how to dynamically remove constraints to simulate cloth tearing.

Goal: A piece of cloth that can drape over rigid bodies and react to wind.

## Chapter 12: Intro to Fluid Simulation - Smoothed Particle Hydrodynamics (SPH)

Fluids as Particles: Representing a volume of fluid as a large collection of free-moving particles, each with properties like density and pressure.

The "Smoothing Kernel": The core mathematical concept of SPH, where properties at any point are determined by a weighted average of nearby particles.

Calculating Density and Pressure: Implementing the core SPH equations to make particles push away from each other in high-density areas.

Adding Viscosity: Simulating "thicker" fluids like honey or oil.

Goal: A basic 2D or 3D container of particles that behave like a fluid, sloshing and settling.

## Chapter 13: Making it Fast - Deformable Body Optimizations

Spatial Grids for Particle Neighborhoods: The most critical optimization for SPH and PBD is finding nearby particles quickly. We'll implement a uniform grid to accelerate these "neighbor searches."

Parallelism: Discussing how particle-based physics (PBD, SPH, cloth) is often highly parallelizable and how to approach it using modern CPU or GPU techniques.

Goal: An optimized simulation that can handle thousands of particles (for cloth or fluids) in real-time.