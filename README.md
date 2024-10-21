# StarRust

# [Play Here](https://larsdu.github.io/StarRust/)

Open-source sidescrolling space shooter made with Rust and Bevy

Currently very unstable.

NOTE (8/4/2023): This project is generally shelved until Bevy is more mature. Namely GPU instancing and/or batched rendering, and a particle system that works with WASM builds would be highly desirable!

TODO:

- [x] Rework spawning to allow for arbritrary types to spawn.
- [ ] Adopt Bevy Hanabi particle system once [WASM compatibility is implemented](https://github.com/djeedai/bevy_hanabi/issues/41)
- [ ] Setup Shader instancing for 3d Models
- [ ] UI
  - [ ] Player Healthbar
- [ ] Set up PowerUps
  - [ ] PowerUpBundle and PowerUpSystem
- [x] Migrate to Bevy `stageless` RFC
- [ ] Implement new enemies
  - [ ] Platform with turret
  - [ ] Boss with multiple turret
- [ ] PowerUps
  - [ ] PowerUp Plugin
  - [ ] PowerUp-specific bundles
  - [ ] Powerup pickups
- [ ] Weapon PowerUps
  - [ ] 3x Cone shot
  - [ ] 6x Cone shot
  - [ ] "Bombs"
  - [ ] Missiles with sinusoidal trajectories
- [ ] Performance improvements:
  - [ ] Webassembly builds choke on asset loading (on pressing start button)
  - [-] Adopt GPU instances for 3d meshes [once this is implemented in bevy](https://github.com/bevyengine/bevy/issues/89)
- [ ] Multiple levels

### [Contributor Guidlines](https://github.com/LarsDu/StarRust/blob/main/CONTRIBUTING.md)
