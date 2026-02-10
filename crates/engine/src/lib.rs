pub mod debug {
    mod chunk_gen;

    pub use chunk_gen::*;
}

pub mod terrain {
    pub mod ecs {
        pub mod components {
            pub mod chunk;
        }
        pub mod resources {
            pub mod voxel;
            pub mod chunk;
        }
    }
    pub mod defs {
        pub mod voxel;
    }
    pub mod types {
        pub mod voxel;

        pub use voxel::Voxel;
    }
    pub mod plugins {
        mod terrain;
        pub use terrain::*;
    }
    pub mod tasks {
        mod terrain;
        pub use terrain::*;
    }
    pub mod generator {
        mod generator;
        mod heightmap;
        mod noise;

        pub use generator::TerrainManager;
    }
    pub mod constants;
    pub mod meshing {
        pub(crate) mod bevy_meshing;
        pub(crate) mod greedy;
        pub(crate) mod mesh_data;
    }
}

pub fn init() {
    println!("{} initialized", env!("CARGO_PKG_NAME"));
}
