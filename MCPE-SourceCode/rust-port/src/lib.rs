#![recursion_limit = "512"]

pub mod mth;
pub mod random;
pub mod data_io;
pub mod perf_timer;
pub mod smooth_float;
pub mod string_utils;
pub mod weighed_random;
pub mod world_pos;
pub mod facing;
pub mod direction;
pub mod nbt;
pub mod level_data;
pub mod region_file;
pub mod storage_api;
pub mod folder_methods;
pub mod level_storage_source;
pub mod memory_storage;
pub mod sound;
pub mod vec3;
pub mod hit_result;
pub mod aabb;
pub mod chunk_pos;
pub mod tile_pos;
pub mod tick_next_tick_data;
pub mod data_layer;
pub mod chunk_codec;
pub mod level_chunk;
pub mod empty_level_chunk;
pub mod light_layer;
pub mod entity_core;
pub mod entity_pos;
pub mod mob_category;
pub mod motive;
pub mod app;
pub mod app_platform;
pub mod options;
pub mod timer;
pub mod minecraft;
pub mod ninecraft_app;
pub mod level;
pub mod renderer;
pub mod entity;
pub mod mob;
pub mod player;
pub mod local_player;
pub mod material;
pub mod tile;
pub mod item;
pub mod item_instance;
pub mod inventory;
pub mod feature;
pub mod gui;
pub mod tesselator;
pub mod textures;
pub mod mesh;
pub mod noise;
pub mod font;
pub mod screen;
pub mod mob_entity;
pub mod recipes;
pub mod projectile;
pub mod particle;
pub mod food_data;
pub mod pathfinder;
pub mod ai;
pub mod model;

#[cfg(test)]
mod tests {
    use crate::{
        data_io::{DataInput, DataOutput, MemoryDataInput, MemoryDataOutput},
        aabb::Aabb, chunk_codec::{ChunkBuffers, CHUNK_HEIGHT, CHUNK_WIDTH}, chunk_pos::ChunkPos, data_layer::DataLayer, empty_level_chunk::EmptyLevelChunk, entity_core::EntityCore, entity_pos::EntityPos, hit_result::HitResultType, level::Level, level_chunk::LevelChunk, level_data::LevelData, level_storage_source::{ExternalFileLevelStorageSource, LevelStorageSource}, light_layer::LightLayer, memory_storage::MemoryLevelStorageSource, mob_category, mth, motive, nbt, perf_timer, random::Random, region_file::RegionFile, smooth_float::SmoothFloat, storage_api, string_utils, tick_next_tick_data::TickNextTickData, tile_pos::TilePos, vec3::Vec3,
        weighed_random::{self, WeighedRandomItem}, world_pos::Pos, facing, direction,
    };
    use std::collections::BTreeMap;
    use std::fs;

    #[test]
    fn mth_core_functions_behave() {
        mth::init_mth();
        assert_eq!(mth::floor(1.2), 1);
        assert_eq!(mth::floor(-1.2), -2);
        assert_eq!(mth::clamp_i(12, 0, 10), 10);
        assert_eq!(mth::lerp_i(10, 20, 0.5), 15);
        assert_eq!(mth::int_floor_div(-3, 2), -2);
        let s = mth::sin(0.0);
        assert!(s.abs() < 0.001);
    }

    #[test]
    fn string_utils_behave() {
        assert!(string_utils::starts_with("minecraft", "mine"));
        assert_eq!(
            string_utils::string_replace("aaaa".to_string(), "a", "b", 2),
            "bbaa"
        );
        assert_eq!(string_utils::string_trim(" \n hi\t "), "hi");
        assert_eq!(string_utils::hash_code("abc"), 96354);
        assert_eq!(
            string_utils::remove_all("a-b-c".to_string(), &["-", "c"]),
            "ab"
        );
    }

    #[test]
    fn random_is_deterministic_for_seed() {
        let mut a = Random::new(12345);
        let mut b = Random::new(12345);
        assert_eq!(a.next_int(), b.next_int());
        assert_eq!(a.next_int_n(1000), b.next_int_n(1000));
        assert!((a.next_float() - b.next_float()).abs() < f32::EPSILON);
    }

    #[test]
    fn data_io_roundtrip() {
        let mut out = MemoryDataOutput::new();
        out.write_int(42);
        out.write_float(3.5);
        out.write_string("hello");

        let mut inp = MemoryDataInput::new(out.into_inner());
        assert_eq!(inp.read_int(), 42);
        assert!((inp.read_float() - 3.5).abs() < f32::EPSILON);
        assert_eq!(inp.read_string(), "hello");
    }

    #[test]
    fn smooth_float_moves_towards_target() {
        let mut s = SmoothFloat::new();
        let d1 = s.get_new_delta_value(10.0, 0.2);
        let d2 = s.get_new_delta_value(0.0, 0.2);
        assert!(d1 > 0.0);
        assert!(d2 >= 0.0);
        assert!(s.target_value() >= 10.0);
    }

    #[test]
    fn weighed_random_selection_works() {
        let mut rng = Random::new(123);
        let items = [
            WeighedRandomItem::new(2),
            WeighedRandomItem::new(5),
            WeighedRandomItem::new(3),
        ];
        let total = weighed_random::get_total_weight(&items);
        assert_eq!(total, 10);
        let idx = weighed_random::get_random_item_index_auto(&mut rng, &items);
        assert!((0..items.len() as i32).contains(&idx));
    }

    #[test]
    fn perf_timer_basic_log() {
        perf_timer::set_enabled(true);
        perf_timer::reset();
        perf_timer::push("root");
        perf_timer::push("tick");
        perf_timer::pop();
        perf_timer::pop();
        let log = perf_timer::get_log("root");
        assert!(!log.is_empty());
        assert_eq!(log[0].name, "root");
    }

    #[test]
    fn world_pos_navigation() {
        let p = Pos::new(10, 20, 30);
        assert_eq!(p.above(), Pos::new(10, 21, 30));
        assert_eq!(p.north_n(2), Pos::new(10, 20, 28));
        assert_eq!(p.east_n(3), Pos::new(13, 20, 30));
        // Preserve source behavior: west_n ignores steps.
        assert_eq!(p.west_n(100), Pos::new(9, 20, 30));
    }

    #[test]
    fn facing_and_direction_tables_match_source() {
        assert_eq!(facing::to_string(facing::NORTH), "North");
        assert_eq!(facing::OPPOSITE_FACING[facing::EAST as usize], facing::WEST);
        assert_eq!(direction::DIRECTION_FACING[direction::SOUTH as usize], facing::SOUTH);
        assert_eq!(direction::FACING_DIRECTION[facing::UP as usize], direction::UNDEFINED);
        assert_eq!(
            direction::RELATIVE_DIRECTION_FACING[direction::EAST as usize][facing::NORTH as usize],
            facing::WEST
        );
    }

    #[test]
    fn nbt_named_tag_roundtrip() {
        let mut out = MemoryDataOutput::new();
        let mut root = BTreeMap::new();
        root.insert("health".to_string(), nbt::TagValue::Short(20));
        root.insert("name".to_string(), nbt::TagValue::String("Steve".to_string()));
        root.insert(
            "pos".to_string(),
            nbt::TagValue::List {
                element_type: nbt::TAG_FLOAT,
                items: vec![nbt::TagValue::Float(1.0), nbt::TagValue::Float(2.0), nbt::TagValue::Float(3.0)],
            },
        );
        nbt::write_root_compound(&mut out, "Player", root.clone());

        let mut inp = MemoryDataInput::new(out.into_inner());
        let parsed = nbt::read_root_compound(&mut inp).expect("expected valid root compound");
        assert_eq!(parsed.0, "Player");
        assert_eq!(parsed.1, root);
        assert_eq!(nbt::get_tag_name(nbt::TAG_COMPOUND), "TAG_Compound");
    }

    #[test]
    fn level_data_nbt_roundtrip() {
        let mut level = LevelData {
            seed: 12345,
            game_type: 0,
            level_name: "World1".to_string(),
            x_spawn: 10,
            y_spawn: 64,
            z_spawn: 10,
            time: 777,
            size_on_disk: 1024,
            storage_version: 3,
            ..LevelData::default()
        };
        let mut player = BTreeMap::new();
        player.insert("Dimension".to_string(), nbt::TagValue::Int(0));
        level.loaded_player_tag = Some(player.clone());

        let tag = level.to_nbt();
        let parsed = LevelData::from_nbt(&tag);

        assert_eq!(parsed.seed, 12345);
        assert_eq!(parsed.level_name, "World1");
        assert_eq!(parsed.time, 777);
        assert_eq!(parsed.storage_version, 3);
        assert_eq!(parsed.loaded_player_tag, Some(player));
    }

    #[test]
    fn region_file_chunk_roundtrip() {
        let mut dir = std::env::temp_dir();
        dir.push("rust_port_region_test");
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).expect("failed to create temp dir");

        let mut region = RegionFile::new(&dir);
        region.open().expect("failed to open region");
        let payload = vec![1u8, 2, 3, 4, 5, 6, 7, 8, 9];
        region
            .write_chunk(2, 3, &payload)
            .expect("failed to write chunk");
        let out = region
            .read_chunk(2, 3)
            .expect("failed to read chunk")
            .expect("chunk should exist");
        assert_eq!(out, payload);
        region.close();

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn storage_api_level_and_chunk_roundtrip() {
        let mut dir = std::env::temp_dir();
        dir.push("rust_port_storage_api_test");
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).expect("failed to create test dir");

        let data = LevelData {
            seed: 999,
            game_type: 0,
            level_name: "ApiWorld".to_string(),
            x_spawn: 1,
            y_spawn: 70,
            z_spawn: 2,
            time: 44,
            size_on_disk: 88,
            storage_version: 2,
            ..LevelData::default()
        };

        storage_api::save_level_data(&dir, &data).expect("save_level_data failed");
        let loaded = storage_api::load_level_data(&dir)
            .expect("load_level_data failed")
            .expect("level.dat should exist");
        assert_eq!(loaded.seed, 999);
        assert_eq!(loaded.level_name, "ApiWorld");

        let payload = vec![10u8, 20, 30, 40];
        storage_api::save_chunk(&dir, 1, 1, &payload).expect("save_chunk failed");
        let chunk = storage_api::load_chunk(&dir, 1, 1)
            .expect("load_chunk failed")
            .expect("chunk should exist");
        assert_eq!(chunk, payload);

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn level_storage_source_list_rename_delete() {
        let mut base = std::env::temp_dir();
        base.push("rust_port_storage_source_base");
        let mut tmp = std::env::temp_dir();
        tmp.push("rust_port_storage_source_tmp");
        let _ = fs::remove_dir_all(&base);
        let _ = fs::remove_dir_all(&tmp);
        fs::create_dir_all(&base).expect("base create failed");
        fs::create_dir_all(&tmp).expect("tmp create failed");

        let src = ExternalFileLevelStorageSource::new(&base, &tmp, true).expect("source create failed");

        let world_a = src.get_full_path("WorldA");
        let world_b = src.get_full_path("WorldB");
        fs::create_dir_all(&world_a).expect("world a create");
        fs::create_dir_all(&world_b).expect("world b create");

        storage_api::save_level_data(
            &world_a,
            &LevelData { level_name: "First".to_string(), last_played: 100, ..LevelData::default() },
        )
        .expect("save world a");
        storage_api::save_level_data(
            &world_b,
            &LevelData { level_name: "Second".to_string(), last_played: 200, ..LevelData::default() },
        )
        .expect("save world b");

        let list = src.get_level_list().expect("list failed");
        assert_eq!(list.len(), 2);
        assert!(list.iter().any(|s| s.id == "WorldA"));
        assert!(list.iter().any(|s| s.id == "WorldB"));

        let renamed_id = src
            .rename_level("WorldA", "Renamed/World")
            .expect("rename failed");
        assert!(renamed_id.starts_with("RenamedWorld"));
        assert!(src.get_full_path(&renamed_id).exists());

        src.delete_level("WorldB").expect("delete failed");
        assert!(!src.get_full_path("WorldB").exists());

        let _ = fs::remove_dir_all(&base);
        let _ = fs::remove_dir_all(&tmp);
    }

    #[test]
    fn memory_storage_source_basic() {
        let src = MemoryLevelStorageSource::new();
        assert_eq!(src.get_name(), "Memory Storage");
        assert!(src.is_new_level_id_acceptable("anything"));

        let mut level = src.select_level("L1");
        let storage = level.create_chunk_storage();
        storage.save_chunk(0, 0, vec![1, 2, 3]);
        assert_eq!(storage.load_chunk(0, 0), Some(&[1, 2, 3][..]));
    }

    #[test]
    fn vec3_and_aabb_clip() {
        let a = Vec3::new(-1.0, 0.5, 0.5);
        let b = Vec3::new(2.0, 0.5, 0.5);
        let box1 = Aabb::new(0.0, 0.0, 0.0, 1.0, 1.0, 1.0);
        let hit = box1.clip(a, b);
        assert!(hit.is_hit());
        assert_eq!(hit.hit_type, HitResultType::Tile);
        assert_eq!(hit.f, 4);
        assert!((hit.pos.x - 0.0).abs() < 0.0001);
    }

    #[test]
    fn hit_result_distance() {
        let h = crate::hit_result::HitResult::entity(7, Vec3::new(1.0, 2.0, 3.0));
        let d = h.distance_to_xyz_sqr(Vec3::new(2.0, 2.0, 3.0));
        assert!((d - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn chunk_and_tile_pos_hashes() {
        let c = ChunkPos::new(-1, 2);
        assert_eq!(c.hash_code(), ChunkPos::hash_code_xy(-1, 2));
        let d2 = c.distance_to_sqr_entity_pos(0.0, 0.0);
        assert!(d2 > 0.0);

        let t = TilePos::new(1, 2, 3);
        assert_eq!(t.hash_code(), 8_976_890 + 1_962_262 + 3);
    }

    #[test]
    fn tick_next_tick_data_ordering() {
        let a = TickNextTickData::new(0, 0, 0, 1).set_delay(10);
        let b = TickNextTickData::new(0, 0, 1, 1).set_delay(20);
        assert!(a < b);
        assert_ne!(a.hash_code(), b.hash_code());
        assert_ne!(a, b);
    }

    #[test]
    fn data_layer_nibbles_work() {
        let mut dl = DataLayer::new(16);
        dl.set(0, 0x0a);
        dl.set(1, 0x0b);
        assert_eq!(dl.get(0), 0x0a);
        assert_eq!(dl.get(1), 0x0b);
        let mut world_dl = DataLayer::new(16 * 16 * 128);
        world_dl.set_xyz(1, 2, 3, 0x0c);
        assert_eq!(world_dl.get_xyz(1, 2, 3), 0x0c);
    }

    #[test]
    fn chunk_codec_roundtrip_range() {
        let mut c = ChunkBuffers::default();
        c.blocks[0] = 42;
        c.blocks[(1 << 11) | (2 << 7) | 3] = 77;
        c.data.set_xyz(1, 2, 3, 5);
        c.block_light.set_xyz(1, 2, 3, 7);
        c.sky_light.set_xyz(1, 2, 3, 9);

        let encoded = c.encode_range(0, 0, 0, CHUNK_WIDTH, CHUNK_HEIGHT, CHUNK_WIDTH);
        let mut d = ChunkBuffers::default();
        let consumed = d.decode_range(&encoded, 0, 0, 0, CHUNK_WIDTH, CHUNK_HEIGHT, CHUNK_WIDTH);
        assert_eq!(consumed, encoded.len());
        assert_eq!(d.blocks[0], 42);
        assert_eq!(d.blocks[(1 << 11) | (2 << 7) | 3], 77);
        assert_eq!(d.data.get_xyz(1, 2, 3), 5);
        assert_eq!(d.block_light.get_xyz(1, 2, 3), 7);
        assert_eq!(d.sky_light.get_xyz(1, 2, 3), 9);
    }

    #[test]
    fn level_chunk_core_tile_data_and_update_map() {
        let mut c = LevelChunk::new(2, 3);
        assert!(c.is_at(2, 3));
        assert_eq!(c.xt, 32);
        assert_eq!(c.zt, 48);

        assert!(c.set_tile_and_data(1, 20, 2, 5, 7));
        assert_eq!(c.get_tile(1, 20, 2), 5);
        assert_eq!(c.get_data(1, 20, 2), 7);
        assert!(c.unsaved);
        assert_ne!(c.update_map[1 | (2 << 4)], 0);

        let before = c.update_map;
        assert!(!c.set_tile_and_data(1, 20, 2, 5, 7));
        assert_eq!(c.update_map, before);

        c.clear_update_map();
        assert!(!c.unsaved);
        assert!(c.update_map.iter().all(|b| *b == 0));
    }

    #[test]
    fn level_chunk_entity_buckets() {
        let mut c = LevelChunk::new(0, 0);
        let b0 = c.add_entity_stub(1, -4.0);
        let b1 = c.add_entity_stub(2, 17.0);
        assert_eq!(b0, 0);
        assert_eq!(b1, 1);
        assert_eq!(c.count_entities(), 2);
        c.remove_entity_stub(1, 0);
        assert_eq!(c.count_entities(), 1);
    }

    #[test]
    fn level_basic_tile_access() {
        let source = crate::memory_storage::MemoryLevelStorageSource::new();
        let mut level = Level::new(Box::new(source), "TestLevel", (), 0);
        assert_eq!(level.get_tile(0, 64, 0), crate::tile::GRASS.id);
        assert_eq!(level.get_tile(0, 63, 0), crate::tile::STONE.id);
        assert!(level.set_tile(1, 64, 1, crate::tile::STONE.id));
        assert_eq!(level.get_tile(1, 64, 1), crate::tile::STONE.id);
    }

    #[test]
    fn empty_level_chunk_basics() {
        let c = EmptyLevelChunk::new(4, 5, 255);
        assert!(c.inner.dont_save);
        assert_eq!(c.get_tile(0, 0, 0), 255);
        assert!(c.is_empty());
    }

    #[test]
    fn level_chunk_brightness_paths() {
        let mut c = LevelChunk::new(0, 0);
        c.set_brightness(LightLayer::Sky, 1, 2, 3, 10);
        c.set_brightness(LightLayer::Block, 1, 2, 3, 7);
        assert_eq!(c.get_brightness(LightLayer::Sky, 1, 2, 3), 10);
        assert_eq!(c.get_brightness(LightLayer::Block, 1, 2, 3), 7);

        LevelChunk::clear_touched_sky();
        assert_eq!(c.get_raw_brightness(1, 2, 3, 2), 8);
        assert!(LevelChunk::touched_sky());
        assert_eq!(LightLayer::Sky.surrounding(), 15);
        assert_eq!(LightLayer::Block.surrounding(), 0);
    }

    #[test]
    fn entity_core_position_distance_and_render() {
        let mut a = EntityCore::default();
        let mut b = EntityCore::default();

        a.set_pos(1.0, 2.0, 3.0);
        b.move_to(4.0, 2.0, 3.0, 0.0, 0.0);

        assert!((a.distance_to_entity(&b) - 3.0).abs() < 0.0001);
        assert!((a.distance_to_sqr_xyz(4.0, 2.0, 3.0) - 9.0).abs() < 0.0001);

        a.push(0.1, 0.2, 0.3);
        assert!((a.xd - 0.1).abs() < f32::EPSILON);
        assert!(a.is_alive());
        a.remove();
        assert!(!a.is_alive());
        assert!(b.should_render(Vec3::new(0.0, 0.0, 0.0)));
    }

    #[test]
    fn entity_pos_variants() {
        let a = EntityPos::with_move_and_rot(1.0, 2.0, 3.0, 90.0, 15.0);
        assert!(a.mov && a.rot);
        let b = EntityPos::with_move(4.0, 5.0, 6.0);
        assert!(b.mov && !b.rot);
        let c = EntityPos::with_rot(30.0, -10.0);
        assert!(!c.mov && c.rot);
    }

    #[test]
    fn mob_category_constants() {
        assert_eq!(mob_category::MONSTER.max_instances_per_chunk(), 10);
        assert_eq!(mob_category::WATER_CREATURE.max_instances_per_level(), 10);
        assert_eq!(mob_category::VALUES.len(), 3);
        assert!(!mob_category::MONSTER.is_friendly());
    }

    #[test]
    fn motive_lookup() {
        let m = motive::get_motive_by_name("DonkeyKong");
        assert_eq!(m.name, "DonkeyKong");
        let fallback = motive::get_motive_by_name("not-real");
        assert_eq!(fallback.name, motive::DEFAULT_IMAGE.name);
        assert!(motive::Motive::MAX_MOTIVE_NAME_LENGTH >= 13);
    }
}
