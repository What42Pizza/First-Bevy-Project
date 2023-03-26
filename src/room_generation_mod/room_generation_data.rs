use crate::prelude::*;





#[derive(Resource, SmartDefault, Deref, DerefMut)]
pub struct RoomEntities (pub HashMap<IVec2, Vec<Entity>>);



#[derive(Resource, SmartDefault, Deref, DerefMut)]
pub struct RoomDatasContainer (pub HashMap<IVec2, RoomData>);

impl RoomDatasContainer {
    pub fn get(&mut self, pos: IVec2) -> &mut RoomData {
        if self.len() > MAX_ROOM_DATA_COUNT {
            self.clear();
        }
        if !self.contains_key(&pos) {
            self.insert(pos, RoomData::new(pos));
        }
        self.get_mut(&pos).unwrap()
    }
}



pub struct RoomData {
    pub walls: (bool, bool),
    pub has_light: bool,
}

impl RoomData {
    pub fn new (pos: IVec2) -> Self {
        Self {
            walls: (fns::chance(0.4), fns::chance(0.4)),
            has_light: pos.x.is_even() && pos.y.is_even() && fns::chance(0.75),
        }
    }
}
