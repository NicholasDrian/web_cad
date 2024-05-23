macro_rules! get_instance_mut {
    ($handle:expr) => {{
        crate::instance::INSTANCES
            .lock()
            .unwrap()
            .get_mut($handle)
            .unwrap()
    }};
}

pub(crate) use get_instance_mut;
