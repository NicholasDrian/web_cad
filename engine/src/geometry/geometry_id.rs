use std::sync::Mutex;

pub type GeometryId = u64;

static mut GEOMETRY_ID_GENERATOR: Mutex<GeometryId> = Mutex::new(0u64);

pub fn new_geometry_id() -> GeometryId {
    unsafe {
        let mut changer = GEOMETRY_ID_GENERATOR.lock().unwrap();
        *changer += 1u64;
        *changer
    }
}
