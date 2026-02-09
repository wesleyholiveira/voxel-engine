pub mod debug {
    mod chunk_gen;

    pub use chunk_gen::spawn_test_chunk;
}

pub mod terrain {
    pub mod ecs {
        pub mod components {
            pub mod chunk;
        }
    }
    pub mod types {
        pub mod voxel;

        pub use voxel::Voxel;
    }
    pub mod generator {
        mod generator;
        mod heightmap;
        mod noise;

        pub use generator::TerrainGenerator;
    }
    pub mod constants;
}

pub fn init() {
    println!("{} initialized", env!("CARGO_PKG_NAME"));
}
