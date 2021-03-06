use crate::frontend;
use crate::*;

use math::round;
use nalgebra::Vector2;

// =========================== SPACER ===========================

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct SpacerInstance {
    spacing: f32,
}

pub struct Spacer {}

impl Component<SpacerInstance, ()> for Spacer {
    fn get_default_data(&self) -> Option<SpacerInstance> {
        Some(SpacerInstance { spacing: 1.0 })
    }

    fn max_children(&self) -> Option<u32> {
        Some(1)
    }

    fn get_name(&self) -> &'static str {
        "Spacer"
    }

    fn init_instance(&self, __ctx: &mut frontend::PresentationContext, __data: &SpacerInstance) {}

    fn draw(
        &self,
        ctx: &mut frontend::PresentationContext,
        zone: DrawZone,
        children: &mut [DrawChild],
        __internal_data: &mut (),
        data: &SpacerInstance,
    ) {
        assert!(children.len() == 1);

        let childzone = DrawZone {
            m: zone.m,
            size: zone.size * data.spacing,
        };

        children[0].as_mut()(ctx, childzone);
    }
}

// =========================== SPLIT ===========================

#[derive(serde::Serialize, serde::Deserialize, Clone, std::cmp::PartialEq)]
pub enum SplitDirection {
    Horizontal,
    Vertical,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, std::cmp::PartialEq)]
pub enum SplitMode {
    EqualArea,
    EqualSide,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct SplitInstance {
    spacing: f32,
    direction: SplitDirection,
    mode: SplitMode,
}

pub struct Split {
    spacer: Spacer,
}

struct SplitInternalData {
    sizes: Vec<Vector2<f32>>,
    primary_width: f32,
}

impl SplitInstance {
    // primary dimension = perpendicular to the separators
    fn pm<'a>(&self, vector: &'a mut Vector2<f32>) -> &'a mut f32 {
        if self.direction == SplitDirection::Horizontal {
            &mut vector.x
        } else {
            &mut vector.y
        }
    }

    fn p<'a>(&self, vector: &'a Vector2<f32>) -> &'a f32 {
        if self.direction == SplitDirection::Horizontal {
            &vector.x
        } else {
            &vector.y
        }
    }

    // secondary dimension
    fn sm<'a>(&self, vector: &'a mut Vector2<f32>) -> &'a mut f32 {
        if self.direction == SplitDirection::Horizontal {
            &mut vector.y
        } else {
            &mut vector.x
        }
    }

    fn s<'a>(&self, vector: &'a Vector2<f32>) -> &'a f32 {
        if self.direction == SplitDirection::Horizontal {
            &vector.y
        } else {
            &vector.x
        }
    }

    fn aspect_to_primary_to_secondary(&self, aspect: f32) -> f32 {
        if self.direction == SplitDirection::Horizontal {
            aspect
        } else {
            1.0 / aspect
        }
    }
}

impl Component<SplitInstance, SplitInternalData> for Split {
    fn get_default_data(&self) -> Option<SplitInstance> {
        Some(SplitInstance {
            spacing: 0.9,
            direction: SplitDirection::Horizontal,
            mode: SplitMode::EqualSide,
        })
    }

    fn init_instance(
        &self,
        ctx: &mut frontend::PresentationContext,
        data: &SplitInstance,
    ) -> SplitInternalData {
        SplitInternalData {
            sizes: Vec::new(),
            primary_width: 0.0,
        }
    }

    fn max_children(&self) -> Option<u32> {
        None
    }

    fn get_name(&self) -> &'static str {
        "Split"
    }

    fn draw(
        &self,
        ctx: &mut frontend::PresentationContext,
        zone: DrawZone,
        children: &mut [DrawChild],
        internal_data: &mut SplitInternalData,
        data: &SplitInstance,
    ) {
        if internal_data.sizes.len() != children.len() {
            internal_data.primary_width = 0.0;
            internal_data.sizes.clear();

            for _ in 0..children.len() {
                internal_data.sizes.push(Vector2::new(1.0, 1.0));
                internal_data.primary_width += 1.0;
            }
        }
        /*
                if data.mode == SplitMode::EqualSide {
                    let mut internal_sizes: Vec<Vector2<f32>> = Vec::new();
                    let mut total_size = 0.0;

                    for size in sizes {
                        let aspect = match size.aspect {
                            Some(aspect) => aspect,
                            None => size.size_preference,
                        };

                        let relative_aspect = data.aspect_to_primary_to_secondary(aspect);

                        internal_sizes.push(Vector2::new(relative_aspect, 1.0));
                        total_size += relative_aspect;
                    }
                } else {
                    panic!();
                    /*
                    AfterInit{
                        aspect: sizes[0].aspect,
                        internal_data: SplitInternalData{
                            sizes:
                        }
                    }
                    */
                }
        */

        if data.mode == SplitMode::EqualSide {
            let space_per_unit = data.p(&zone.size) / internal_data.primary_width;
            let mut primary_cursor = *data.p(&zone.top_left());

            for i in 0..children.len() {
                let mut top_left = Vector2::new(0.0, 0.0);
                let mut bottom_right = Vector2::new(0.0, 0.0);

                *data.pm(&mut top_left) = primary_cursor;
                *data.sm(&mut top_left) = *data.s(&zone.top_left());

                *data.pm(&mut bottom_right) =
                    primary_cursor + internal_data.sizes[i].x * space_per_unit;
                *data.sm(&mut bottom_right) = *data.s(&zone.bottom_right());

                let zone = DrawZone::from_rect(top_left, bottom_right);

                self.spacer.draw(
                    ctx,
                    zone,
                    &mut children[i..i + 1],
                    &mut (),
                    &SpacerInstance {
                        spacing: data.spacing,
                    },
                );

                primary_cursor += internal_data.sizes[i].x * space_per_unit;
            }
        } else {
            panic!();
            /*
            AfterInit{
                aspect: sizes[0].aspect,
                internal_data: SplitInternalData{
                    sizes:
                }
            }
            */
        }
    }
}

// ===========================

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub enum GroupingBoxTitleSize {
    RelativeToHeight(f32),
    Absolute(f32),
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct GroupingBoxData {
    pub spacing: f32,
    pub title_size: GroupingBoxTitleSize,
    pub title: String,
}

pub struct GroupingBox {}

impl Component<GroupingBoxData, ()> for GroupingBox {
    // primary dimension = along split direction
    fn max_children(&self) -> Option<u32> {
        Some(1)
    }

    fn get_name(&self) -> &'static str {
        "GroupingBox"
    }

    fn get_default_data(&self) -> Option<GroupingBoxData> {
        Some(GroupingBoxData {
            spacing: 0.9,
            title_size: GroupingBoxTitleSize::RelativeToHeight(0.2),
            title: "GroupingBox".to_string(),
        })
    }

    fn init_instance(&self, __ctx: &mut frontend::PresentationContext, __data: &GroupingBoxData) {}

    fn draw(
        &self,
        ctx: &mut frontend::PresentationContext,
        zone: DrawZone,
        children: &mut [DrawChild],
        internal_data: &mut (),
        public_data: &GroupingBoxData,
    ) {
        let title_height = match public_data.title_size {
            GroupingBoxTitleSize::RelativeToHeight(height) => zone.size.y * height,
            GroupingBoxTitleSize::Absolute(height) => height,
        };

        let child_height = zone.size.y - title_height;

        let child_zone = DrawZone::from_rect(
            zone.top_left() + Vector2::new(0.0, title_height),
            zone.bottom_right(),
        );

        let text_zone = DrawZone::from_rect(
            zone.top_left(),
            zone.bottom_right() - Vector2::new(0.0, child_height),
        );

        let text_zone = DrawZone {
            m: text_zone.m,
            size: text_zone.size,
        };

        let child_zone = DrawZone {
            m: child_zone.m,
            size: child_zone.size * public_data.spacing,
        };

        let text_opts = nanovg::TextOptions {
            color: ctx.resources.palette.soft_front_color(),
            size: text_zone.size.y * 1.0,
            align: nanovg::Alignment::new().center().middle(),
            line_height: text_zone.size.y * 1.0,
            line_max_width: text_zone.size.x * 1.0,
            ..Default::default()
        };

        ctx.frame.text_box(
            ctx.resources.font,
            (text_zone.left(), text_zone.m.y),
            public_data.title.as_str(),
            text_opts,
        );

        let bounds = ctx.frame.text_box_bounds(
            ctx.resources.font,
            (0.0, 0.0),
            public_data.title.as_str(),
            text_opts,
        );

        let w = match public_data.title == "" {
            false => (bounds.max_x - text_zone.size.x / 2.0) * 1.2,
            true => 0.0,
        };

        ctx.frame.path(
            |mut path| {
                path.move_to((text_zone.m.x - w, text_zone.m.y));
                path.line_to((zone.left(), text_zone.m.y));
                path.line_to((zone.left(), zone.top()));
                path.line_to((zone.right(), zone.top()));
                path.line_to((zone.right(), text_zone.m.y));
                path.line_to((text_zone.m.x + w, text_zone.m.y));

                path.stroke(
                    ctx.resources.palette.soft_front_color(),
                    nanovg::StrokeOptions {
                        width: 3.0,
                        ..Default::default()
                    },
                );
            },
            Default::default(),
        );

        children[0].as_mut()(ctx, child_zone);
    }
}

// ===========================
/*
#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub enum GridDimensions {
    Fixed((i32, i32)), //tuple for better serialization
    Auto,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct GridData {
    pub spacing: f32,
    pub dimensions: GridDimensions,
}

pub struct Grid {}

struct GridInternalData {
    aspects: Vec<Option<f32>>,
}

impl Component<GridData, GridInternalData> for Grid {
    fn max_children(&self) -> Option<u32> {
        None
    }

    fn get_name(&self) -> &'static str {
        "Grid"
    }

    fn get_default_data(&self) -> Option<GridData> {
        Some(GridData {
            spacing: 0.9,
            dimensions: GridDimensions::Auto,
        })
    }

    fn init_instance(
        &self,
        __ctx: &mut frontend::PresentationContext,
        data: &GridData,
        children_sizes: &[DrawChild],
    ) -> GridInternalData {
        let mut total_aspect = 0.0;
        let mut children_with_aspect = 0;

        for size in children_sizes {
            match size.aspect {
                Some(aspect) => {
                    total_aspect += aspect;
                    children_with_aspect += 1;
                }
                _ => {}
            }
        }

        let dims = match data.dimensions {
            GridDimensions::Fixed((w, h)) => (w, h),
            GridDimensions::Auto => (
                round::ceil((children_sizes.len() as f64).sqrt(), 0) as i32,
                round::ceil((children_sizes.len() as f64).sqrt(), 0) as i32,
            ),
        };

        let mean_aspect = match children_sizes.len() {
            0 => None,
            _ => Some(total_aspect / (children_with_aspect as f32)),
        };

        let mut children_aspects: Vec<Option<f32>> = Vec::new();
        for size in children_sizes {
            children_aspects.push(size.aspect);
        }

        AfterInit {
            aspect: match mean_aspect {
                Some(aspect) => Some(aspect * (dims.0 as f32) / (dims.1 as f32)),
                None => None,
            },
            internal_data: GridInternalData {
                aspects: children_aspects,
            },
        }
    }

    fn draw(
        &self,
        __ctx: &mut frontend::PresentationContext,
        zone: DrawZone,
        children: &mut [Box<dyn FnMut(DrawZone) + '_>],
        internal_data: &mut GridInternalData,
        public_data: &GridData,
    ) {
        let dims = match public_data.dimensions {
            GridDimensions::Fixed((w, h)) => (w, h),
            GridDimensions::Auto => (
                round::ceil((children.len() as f64).sqrt(), 0) as i32,
                round::ceil((children.len() as f64).sqrt(), 0) as i32,
            ),
        };

        let (xstep, ystep) = (zone.size.x / (dims.0 as f32), zone.size.y / (dims.1 as f32));
        let mut child_id = 0;

        for y in 0..dims.1 {
            for x in 0..dims.0 {
                if child_id < children.len() {
                    let childzone = DrawZone::from_rect(
                        zone.top_left() + Vector2::new((x as f32) * xstep, (y as f32) * ystep),
                        zone.top_left()
                            + Vector2::new(((x + 1) as f32) * xstep, ((y + 1) as f32) * ystep),
                    );

                    let absolute_spacing = childzone.size.norm() * (1.0 - public_data.spacing);

                    let childzone = DrawZone {
                        m: childzone.m,
                        size: childzone.size - Vector2::new(absolute_spacing, absolute_spacing),
                    };

                    let childzone = childzone.constraint_to_aspect(internal_data.aspects[child_id]);

                    children[child_id].as_mut()(childzone);

                    child_id += 1;
                } else {
                    break;
                }
            }
        }
    }
}
*/
// =========================== UTILS ===========================

pub fn components() -> impl Fn(&mut Manager) {
    |manager: &mut Manager| {
        let split = Box::new(Split { spacer: Spacer {} });
        let spacer = Box::new(Spacer {});
        let grouping_box = Box::new(GroupingBox {});
        //let grid = Box::new(Grid{});

        manager.register_component_type(split);
        manager.register_component_type(spacer);
        manager.register_component_type(grouping_box);
        //manager.register_component_type(grid);
    }
}
