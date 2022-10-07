use super::*;
use super::super::ship::yard::*;



pub struct SpawnSequence;

impl SpawnSequence {
    fn level0() -> Vec<SpawnInfo<AiShipBundle>>{
        return vec![
            SpawnInfo::<AiShipBundle>{
                frequency: 1.0,
                duration: 4.0,
                bundle: DefaultEnemyShip
            },
            SpawnInfo::<AiShipBundle>{
                frequency: 1.0,
                duration: 4.0,
                bundle: DefaultEnemyShip
            },
        ]
    }
}