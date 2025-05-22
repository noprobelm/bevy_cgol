use bevy::prelude::*;
use bevy::utils::Duration;

#[derive(Clone, Eq, PartialEq, Hash, Debug, Default, Resource, Reflect)]
pub struct ElapsedSteps(pub u64);

#[derive(Clone, Eq, PartialEq, Debug, Resource, Reflect)]
pub struct StepTimer(pub Timer);

impl Default for StepTimer {
    fn default() -> Self {
        StepTimer(Timer::new(Duration::from_millis(100), TimerMode::Repeating))
    }
}
