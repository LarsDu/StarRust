# StarRust

# [Play Here](https://larsdu.github.io/StarRust/)

Open-source sidescrolling space shooter made with Rust and Bevy

Currently very unstable

TODO:

-   [ ] Rework spawning to use enums to allow for mixed lists of bundles for
        each SpawnInfo.
-   [ ] Figure out how to get the 2d particle system to actually render
-   [ ] Set up PowerUps
    -   [ ] PowerUpBundle and PowerUpSystem
-   [ ] Migrate to Bevy `stageless` RFC
    -   [ ] Reintroduce FixedTimeStep
-   [ ] Implement new enemies
    -   [ ] Platform with turret
    -   [ ] Boss with multiple turret
-   [ ] UI
    -   [ ] Player Healthbar
-   [ ] PowerUps
    -   [ ] PowerUp Plugin
    -   [ ] PowerUp-specific bundles
    -   [ ] Powerup pickups
-   [ ] Weapon PowerUps
    -   [ ] 3x Cone shot
    -   [ ] 6x Cone shot
    -   [ ] "Bombs"
    -   [ ] Missiles with sinusoidal trajectories
-   [ ] Performance:
    -   [ ] Replace all death triggering events (player, camera shake, etc) with
            [removal detection](https://github.com/bevyengine/bevy/blob/main/examples/ecs/removal_detection.rs)
    -   [ ] All materials in the game are the same for all 3d elements, but
            these materials are almost certainly not being batched/shared.
-   [ ] Multiple levels

### [Contributor Guidlines](https://github.com/LarsDu/StarRust/blob/main/CONTRIBUTING.md)
