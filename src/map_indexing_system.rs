use super::{BlocksTile, Map, Position};
use specs::prelude::*;

pub struct MapIndexingSystem {}

impl<'a> System<'a> for MapIndexingSystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, BlocksTile>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, position, blockers) = data;

        map.populate_blocked();
        for (pos, blocks) in (&position, &blockers).join() {
            let i = map.xy_idx(pos.x, pos.y);
            map.blocked[i] = true;
        }
    }
}
