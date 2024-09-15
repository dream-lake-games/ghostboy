use crate::prelude::*;

/// It's a little cumbersome to get this info
/// Nice to abstract away and update in PreUpdate for other systems that need it
#[derive(Resource, Debug, Clone, Reflect, Default)]
pub struct CurrentLevelHelpers {
    pub bounds: Option<Rect>,
}

fn update_current_level_bounds(
    levels: Query<(&LevelIid, &GlobalTransform)>,
    ldtk_projects: Query<&Handle<LdtkProject>>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
    mut helpers: ResMut<CurrentLevelHelpers>,
) {
    if let Some(ldtk_project) = ldtk_project_assets.get(ldtk_projects.single()) {
        for (level_iid, level_transform) in levels.iter() {
            let level = ldtk_project
                .get_raw_level_by_iid(level_iid.get())
                .expect("level should exist in only project");
            let level_bounds = Rect {
                min: Vec2::new(
                    level_transform.translation().x,
                    level_transform.translation().y,
                ),
                max: Vec2::new(
                    level_transform.translation().x + level.px_wid as f32,
                    level_transform.translation().y + level.px_hei as f32,
                ),
            };
            helpers.bounds = Some(level_bounds);
        }
    } else {
        helpers.bounds = None;
    }
}

pub(super) fn register_my_ldtk_levels(app: &mut App) {
    reg_types!(app, CurrentLevelHelpers);

    app.insert_resource(CurrentLevelHelpers::default());
    app.add_systems(PreUpdate, update_current_level_bounds);
}
