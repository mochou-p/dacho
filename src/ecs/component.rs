// dacho/src/ecs/component.rs

pub trait Component {
    fn name(&self) -> &str;
}

