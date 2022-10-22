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
    -   [ ] Entirely unclear whether materials are being shared amongst gltf
            models. Look into this.
    -   [ ] Figure out if draw calls are being optimized.
-   [ ] Multiple levels

### [Contributor Guidlines](https://github.com/LarsDu/StarRust/blob/main/CONTRIBUTING.md)
