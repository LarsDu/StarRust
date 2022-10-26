# StarRust

# [Play Here](https://larsdu.github.io/StarRust/)

Open-source sidescrolling space shooter made with Rust and Bevy

Currently very unstable

TODO:

-   [x] Rework spawning to allow for arbritrary types to spawn.
-   [-] Figure out how to get the 2d particle system to actually render
    -   Answer: Not possible in current version of bevy
-   [ ] Set up PowerUps
    -   [ ] PowerUpBundle and PowerUpSystem
-   [ ] Migrate to Bevy `stageless` RFC
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
-   [ ] Performance improvements:
    -   [ ] Webassembly builds choke on asset loading (on pressing start button)
    -   [-] All materials in the game are the same for all 3d elements, but
        these materials are almost certainly not being batched/shared.
        -   Answer: Not possible in current version of Bevy (I think).
-   [ ] Multiple levels

### [Contributor Guidlines](https://github.com/LarsDu/StarRust/blob/main/CONTRIBUTING.md)
