use data_types::{BlockID, ChunkData, Coord, DeSerializable};
#[cfg(feature = "create")]
use noise::{Fbm, utils::{NoiseMap, NoiseMapBuilder, PlaneMapBuilder}, MultiFractal, Seedable};
use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
pub struct BlockConfig {
    block: BlockID,
    start_level: f64,
    end_level: f64,
    overwrite: bool,
}

impl BlockConfig {
    pub fn new(block: BlockID, start_level: f64, end_level: f64, overwrite: bool) -> Self {
        Self {
            block,
            start_level,
            end_level,
            overwrite
        }
    }
}

impl Default for BlockConfig {
    fn default() -> Self {
        Self::new(BlockID::default(), 0.0, 0.0, false)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub default_block: BlockID,
    pub blocks: Vec<BlockConfig>,
    pub map_options: MapOptions,
}

impl Config {
    pub fn new(default_block: BlockID, blocks: Vec<BlockConfig>, map_options: MapOptions) -> Self {
        Self {
            default_block,
            blocks,
            map_options
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new(BlockID::default(), vec![BlockConfig::default(); 1], MapOptions::default())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MapOptions {
    pub pos: Coord,
    pub scale: f64,
    pub octaves: usize,
    pub frequency: f64,
    pub lacunarity: f64,
    pub persistence: f64,
    pub seed: u32,
}

impl MapOptions {
    pub fn new(pos: Coord, scale: f64, octaves: usize, frequency: f64, lacunarity: f64, persistence: f64, seed: u32) -> Self {
        Self {
            pos,
            scale,
            octaves,
            frequency,
            lacunarity,
            persistence,
            seed,
        }
    }
}

impl Default for MapOptions {
    fn default() -> Self {
        Self::new(Coord::new(0,0), 1., 8, 1., 0.5, 0.5, 0)
    }
}

#[cfg(feature = "create")]
fn generate_noise_map<const X: usize, const Y: usize>(options: &MapOptions) -> NoiseMap {
    let fbm = Fbm::new()
        .set_octaves(options.octaves)
        .set_frequency(options.frequency)
        .set_lacunarity(options.lacunarity)
        .set_persistence(options.persistence)
        .set_seed(options.seed);
    println!("Copied config data");
    // We subtract 1 from the map sizes as we use zero indexing.
    let x_bounds = (
        (options.pos.0 * X as f64) * options.scale,
        (((options.pos.0+1.) * X as f64) - 1.) * options.scale,
    );
    let y_bounds = (
        (options.pos.1 * Y as f64) * options.scale,
        (((options.pos.1+1.) * Y as f64) - 1.) * options.scale,
    );
    println!("Seed: {}", options.seed);
    println!("Map size: {:?}", (X, Y));
    println!("Pos: {:?}", options.pos);
    println!("X bounds: {:?}", x_bounds);
    println!("Y bounds: {:?}", y_bounds);
    PlaneMapBuilder::new(&fbm)
        .set_size(X, Y)
        .set_x_bounds(
            x_bounds.0,
            x_bounds.1,
        )
        .set_y_bounds(
            y_bounds.0,
            y_bounds.1,
        )
        .build()
}

#[cfg(feature = "create")]
fn add_terrain<const X: usize, const Y: usize>(
    options: &MapOptions,
    config: &BlockConfig,
    chunk: &mut ChunkData<X, Y>
) {
    println!("Generating noise map");
    let noise_map = generate_noise_map::<X, Y>(options);
    println!("Noise map generated");
    for x in 0..(X-1) {
        for y in 0..(Y-1) {
            if (config.overwrite || chunk[x][y] == 0)
                && config.start_level <= noise_map.get_value(x, y)
                && noise_map.get_value(x, y) < config.end_level
            {
                chunk[x][y] = config.block;
            }
        }
    }
}

#[cfg(feature = "create")]
pub fn generate_chunk<const X: usize, const Y: usize>(config: &Config) -> ChunkData<X, Y> {
    let mut chunk = ChunkData::new(config.default_block);
    for block in &config.blocks {
        println!("Add terrain for {:?}", block);
        add_terrain(&config.map_options, block, &mut chunk);
    }
    chunk
}