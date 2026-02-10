use bevy::ecs::resource::Resource;

#[derive(Resource)]
pub struct ChunkSemaphore {
    permits: usize,
    max: usize,
}

impl ChunkSemaphore {
    pub fn new(max: usize) -> Self {
        Self { permits: max, max }
    }

    /// Returns true if we successfully took a permit
    pub fn try_acquire(&mut self) -> bool {
        if self.permits > 0 {
            self.permits -= 1;
            true
        } else {
            false
        }
    }

    pub fn release(&mut self) {
        self.permits = (self.permits + 1).min(self.max);
    }
}