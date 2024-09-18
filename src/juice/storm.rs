use crate::prelude::*;

fn startup_storm(mut commands: Commands) {
    commands.spawn((
        Name::new("steady_rain"),
        Pos::new(0.0, 0.0).to_spatial(ZIX_RAIN),
        AnimMan::<RainAnim>::new(),
    ));
}

pub(super) fn register_storm(app: &mut App) {
    app.add_systems(OnEnter(MetaStateKind::Level), startup_storm);
}
