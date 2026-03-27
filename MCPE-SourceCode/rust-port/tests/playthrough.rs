use rust_port::app::App;

#[test]
fn automated_playthrough() {
    // Automated headless playthrough that exercises core game logic without a display.
    let mut app = rust_port::ninecraft_app::NinecraftApp::new();
    app.init();

    // Ensure a level was selected
    let mc = app.minecraft_mut();
    assert!(mc.level_ref().is_some(), "Level should be initialized");

    // Verify spawn tile and basic tile ops
    {
        let lvl = mc.level_ref().unwrap();
        let spawn = lvl.get_tile(0, 64, 0);
        // Spawn should be a reasonable surface block (grass/sand/dirt/stone) or air in edge cases
        let ok = spawn == rust_port::tile::GRASS.id
            || spawn == rust_port::tile::SAND.id
            || spawn == rust_port::tile::DIRT.id
            || spawn == rust_port::tile::STONE.id
            || spawn == rust_port::tile::AIR.id;
        assert!(ok, "Unexpected spawn tile id: {}", spawn);
    }

    // Mutate blocks and validate get/set
    {
        let level_mut = mc.level_mut().expect("level_mut present");
        let set_ok = level_mut.set_tile(1, 64, 0, rust_port::tile::STONE.id);
        assert!(set_ok, "set_tile should succeed");
        assert_eq!(level_mut.get_tile(1, 64, 0), rust_port::tile::STONE.id);

        let set_ok2 = level_mut.set_tile(1, 64, 0, rust_port::tile::AIR.id);
        assert!(set_ok2, "set_tile to air should succeed");
        assert_eq!(level_mut.get_tile(1, 64, 0), rust_port::tile::AIR.id);

        // Exercise ray-clip (should not panic)
        let p1 = rust_port::vec3::Vec3::new(0.5, 65.0, 0.5);
        let p2 = rust_port::vec3::Vec3::new(0.5, 63.0, 0.5);
        let _ = level_mut.clip(p1, p2);
    }

    // Run several update ticks to ensure no panics in entity ticking
    for _ in 0..100 {
        app.update();
    }
}
