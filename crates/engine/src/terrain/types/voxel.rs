#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Voxel {
    Air = 0,
    Solid = 1,
}

impl Voxel {
    #[inline]
    pub fn is_solid(self) -> bool {
        self != Voxel::Air
    }
}
