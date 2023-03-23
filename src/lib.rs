// Copyright (c) 2023 Xilexio
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and
// associated documentation files (the “Software”), to deal in the Software without restriction,
// including without limitation the rights to use, copy, modify, merge, publish, distribute,
// sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or
// substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT
// NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
// DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use screeps::{
    CircleStyle, LineStyle, PolyStyle, RectStyle, RoomCoordinate, RoomName, RoomVisual,
    RoomXY, StructureType,
};
use std::collections::HashSet;
use std::ops::Deref;

const DARK_COLOR: &str = "#181818";
const LIGHT_COLOR: &str = "#CCCCCC";
const OUTLINE_COLOR: &str = "#8FBB93";
const GRAY_COLOR: &str = "#555555";
const ENERGY_COLOR: &str = "#FFE87B";
const POWER_COLOR: &str = "#F53547";
const ROAD_COLOR: &str = "#666666";
const WHITE_COLOR: &str = "#FFFFFF";
const BLACK_COLOR: &str = "#000000";
const RAMPART_COLOR: &str = "#669966";
const RAMPART_OUTLINE_COLOR: &str = "#66CC66";
const FACTORY_OUTER_CIRCLE_FILL_COLOR: &str = "#232323";
const FACTORY_PARTS_COLOR: &str = "#140a0a";
const FACTORY_LEVEL_CIRCLE_FILL_COLOR: &str = "#302a2a";

pub struct RoomVisualExt {
    pub room_visual: RoomVisual,
    roads: HashSet<RoomXY>,
}

impl Deref for RoomVisualExt {
    type Target = RoomVisual;

    fn deref(&self) -> &Self::Target {
        &self.room_visual
    }
}

impl RoomVisualExt {
    pub fn new(room_name: RoomName) -> Self {
        RoomVisualExt {
            room_visual: RoomVisual::new(Some(room_name)),
            roads: HashSet::new(),
        }
    }

    /// Draws given structure in the (x, y) tile with given opacity.
    ///
    /// Arguments x and y should be integers to display the structure in the respective tile.
    /// Opacity 1.0 results in a fully opaque structure, 0.0 in a fully transparent. Using opacity
    /// smaller than 1.0 will result in mild visual glitches due to structure's parts visually
    /// overlapping each other instead of the ones in the front hiding the ones behind. However,
    /// everything is still perfectly recognizable even at very low opacities.
    /// The roads connect to each other automatically.
    /// For the most game-like results, draw the roads before containers on them, and draw the
    /// ramparts after structures below them.
    // Fixing the opacity issue would be be difficult without introducing many more points, which
    // would use up even more of the 500kB serialized visual data limit.
    pub fn structure(&mut self, x: f32, y: f32, structure_type: StructureType, opacity: f32) {
        match structure_type {
            StructureType::Spawn => {
                let spawn_circle_style = CircleStyle::default()
                    .radius(0.65)
                    .fill(DARK_COLOR)
                    .stroke(LIGHT_COLOR)
                    .stroke_width(0.1)
                    .opacity(opacity);
                self.circle(x, y, Some(spawn_circle_style));

                let energy_circle_style = CircleStyle::default()
                    .radius(0.4)
                    .fill(ENERGY_COLOR)
                    .opacity(opacity);
                self.circle(x, y, Some(energy_circle_style));
            }
            StructureType::Extension => {
                let extension_circle_style = CircleStyle::default()
                    .radius(0.45)
                    .fill(DARK_COLOR)
                    .stroke(OUTLINE_COLOR)
                    .stroke_width(0.05)
                    .opacity(opacity);
                self.circle(x, y, Some(extension_circle_style));

                let energy_circle_style = CircleStyle::default()
                    .radius(0.32)
                    .fill(ENERGY_COLOR)
                    .opacity(opacity);
                self.circle(x, y, Some(energy_circle_style));
            }
            StructureType::Road => {
                let road_circle_style = CircleStyle::default()
                    .radius(0.15)
                    .fill(ROAD_COLOR)
                    .opacity(opacity);
                self.circle(x, y, Some(road_circle_style));

                // Using unsafe as to not introduce error handling for the client just for connecting roads.
                // Wrong coordinates are not checked in other structures' visualizations anyway.
                let xy = unsafe {
                    RoomXY {
                        x: RoomCoordinate::unchecked_new(x as u8),
                        y: RoomCoordinate::unchecked_new(y as u8),
                    }
                };

                if self.roads.insert(xy) {
                    for near_y in (y as i16 - 1)..(y as i16 + 2) {
                        for near_x in (x as i16 - 1)..(x as i16 + 2) {
                            let near_xy = unsafe {
                                RoomXY {
                                    x: RoomCoordinate::unchecked_new(near_x as u8),
                                    y: RoomCoordinate::unchecked_new(near_y as u8),
                                }
                            };
                            if self.roads.contains(&near_xy) {
                                let road_line_style = LineStyle::default()
                                    .width(0.3)
                                    .color(ROAD_COLOR)
                                    .opacity(opacity);
                                self.line(
                                    (x, y),
                                    (f32::from(near_x), f32::from(near_y)),
                                    Some(road_line_style),
                                );
                            }
                        }
                    }
                };
            }
            StructureType::Wall => {
                let wall_rect_style = RectStyle::default()
                    .fill(DARK_COLOR)
                    .stroke(BLACK_COLOR)
                    .stroke_width(0.07)
                    .opacity(opacity);
                self.rect(x - 0.5, y - 0.5, 1.0, 1.0, Some(wall_rect_style));

                let wall_stripe_line_style1 = LineStyle::default()
                    .width(0.05)
                    .color(ROAD_COLOR)
                    .opacity(opacity);
                let wall_stripe_line_style2 = wall_stripe_line_style1.clone();
                self.line(
                    (x - 0.25, y - 0.15),
                    (x + 0.05, y - 0.15),
                    Some(wall_stripe_line_style1),
                );
                self.line(
                    (x - 0.05, y + 0.15),
                    (x + 0.25, y + 0.15),
                    Some(wall_stripe_line_style2),
                );
            }
            StructureType::Rampart => {
                let rampart_rect_style = RectStyle::default()
                    .fill(RAMPART_COLOR)
                    .stroke(RAMPART_OUTLINE_COLOR)
                    .stroke_width(0.07)
                    .opacity(opacity * 0.3);
                self.rect(x - 0.5, y - 0.5, 1.0, 1.0, Some(rampart_rect_style));
            }
            StructureType::Link => {
                let outline_poly_points = vec![
                    (0.0, -0.45),
                    (0.35, 0.0),
                    (0.0, 0.45),
                    (-0.35, 0.0),
                    (0.0, -0.45),
                ]
                    .into_iter()
                    .map(|(poly_x, poly_y)| (x + poly_x, y + poly_y))
                    .collect();
                let outline_poly_style = PolyStyle::default()
                    .fill(DARK_COLOR)
                    .stroke(OUTLINE_COLOR)
                    .stroke_width(0.06)
                    .opacity(opacity);
                self.poly(outline_poly_points, Some(outline_poly_style));

                let interior_poly_points = vec![
                    (0.0, -0.25),
                    (0.2, 0.0),
                    (0.0, 0.25),
                    (-0.2, 0.0),
                    (0.0, -0.25),
                ]
                    .into_iter()
                    .map(|(poly_x, poly_y)| (x + poly_x, y + poly_y))
                    .collect();
                let interior_poly_style = PolyStyle::default()
                    .fill(ENERGY_COLOR)
                    .stroke("transparent")
                    .opacity(opacity);
                self.poly(interior_poly_points, Some(interior_poly_style));
            }
            StructureType::Storage => {
                let outline_poly_points = vec![
                    (-0.45, -0.55),
                    (0.0, -0.65),
                    (0.45, -0.55),
                    (0.55, 0.0),
                    (0.45, 0.55),
                    (0.0, 0.65),
                    (-0.45, 0.55),
                    (-0.55, 0.0),
                    (-0.45, -0.55),
                ]
                    .into_iter()
                    .map(|(poly_x, poly_y)| (x + poly_x, y + poly_y))
                    .collect();
                let outline_poly_style = PolyStyle::default()
                    .fill(DARK_COLOR)
                    .stroke(OUTLINE_COLOR)
                    .stroke_width(0.07)
                    .opacity(opacity);
                self.poly(outline_poly_points, Some(outline_poly_style));

                let energy_rect_style = RectStyle::default().fill(ENERGY_COLOR).opacity(opacity);
                self.rect(x - 0.35, y - 0.45, 0.7, 0.9, Some(energy_rect_style));
            }
            StructureType::Tower => {
                let outline_circle_style = CircleStyle::default()
                    .radius(0.6)
                    .fill(DARK_COLOR)
                    .stroke(OUTLINE_COLOR)
                    .stroke_width(0.07)
                    .opacity(opacity);
                self.circle(x, y, Some(outline_circle_style));

                let energy_container_rect_style =
                    RectStyle::default().fill(ENERGY_COLOR).opacity(opacity);
                self.rect(
                    x - 0.4,
                    y - 0.3,
                    0.8,
                    0.6,
                    Some(energy_container_rect_style),
                );

                let barrel_rect_style = RectStyle::default()
                    .fill(LIGHT_COLOR)
                    .stroke(DARK_COLOR)
                    .stroke_width(0.07)
                    .opacity(opacity);
                self.rect(x - 0.2, y - 0.9, 0.4, 0.5, Some(barrel_rect_style));
            }
            StructureType::Observer => {
                let outer_circle_style = CircleStyle::default()
                    .radius(0.45)
                    .fill(DARK_COLOR)
                    .stroke(OUTLINE_COLOR)
                    .stroke_width(0.05)
                    .opacity(opacity);
                self.circle(x, y, Some(outer_circle_style));

                let inner_circle_style = CircleStyle::default()
                    .radius(0.2)
                    .fill(OUTLINE_COLOR)
                    .opacity(opacity);
                self.circle(x + 0.255, y, Some(inner_circle_style));
            }
            StructureType::PowerSpawn => {
                let power_spawn_circle_style = CircleStyle::default()
                    .radius(0.65)
                    .fill(DARK_COLOR)
                    .stroke(POWER_COLOR)
                    .stroke_width(0.1)
                    .opacity(opacity);
                self.circle(x, y, Some(power_spawn_circle_style));

                let power_circle_style = CircleStyle::default()
                    .radius(0.4)
                    .fill(ENERGY_COLOR)
                    .opacity(opacity);
                self.circle(x, y, Some(power_circle_style));
            }
            StructureType::Extractor => {
                let extractor_line_style1 = LineStyle::default()
                    .width(0.2)
                    .color(OUTLINE_COLOR)
                    .opacity(opacity);
                let extractor_line_style2 = extractor_line_style1.clone();
                let extractor_line_style3 = extractor_line_style1.clone();
                self.line(
                    (x - 0.4, y - 0.692820323027551),
                    (x + 0.4, y - 0.692820323027551),
                    Some(extractor_line_style1),
                );
                self.line(
                    (x + 0.8, y),
                    (x + 0.4, y + 0.692820323027551),
                    Some(extractor_line_style2),
                );
                self.line(
                    (x - 0.4, y + 0.692820323027551),
                    (x - 0.8, y),
                    Some(extractor_line_style3),
                );
            }
            StructureType::Lab => {
                let outer_circle_style = CircleStyle::default()
                    .radius(0.5)
                    .fill(DARK_COLOR)
                    .stroke(OUTLINE_COLOR)
                    .stroke_width(0.07)
                    .opacity(opacity);
                self.circle(x, y - 0.025, Some(outer_circle_style));

                let interior_circle_style = CircleStyle::default()
                    .radius(0.35)
                    .fill(GRAY_COLOR)
                    .opacity(opacity);
                self.circle(x, y - 0.025, Some(interior_circle_style));

                let mineral_circle_style = CircleStyle::default()
                    .radius(0.2)
                    .fill(WHITE_COLOR)
                    .opacity(opacity);
                self.circle(x, y + 0.12, Some(mineral_circle_style));

                let bottom_rect_style = RectStyle::default().fill(DARK_COLOR).opacity(opacity);
                self.rect(x - 0.45, y + 0.3, 0.9, 0.25, Some(bottom_rect_style));

                let energy_rect_style = RectStyle::default().fill(ENERGY_COLOR).opacity(opacity);
                self.rect(x - 0.2, y + 0.36, 0.4, 0.1, Some(energy_rect_style));

                let bottom_poly_points =
                    vec![(-0.45, 0.3), (-0.45, 0.55), (0.45, 0.55), (0.45, 0.3)]
                        .into_iter()
                        .map(|(poly_x, poly_y)| (x + poly_x, y + poly_y))
                        .collect();
                let bottom_poly_style = PolyStyle::default()
                    .stroke(OUTLINE_COLOR)
                    .stroke_width(0.07)
                    .opacity(opacity);
                self.poly(bottom_poly_points, Some(bottom_poly_style));
            }
            StructureType::Terminal => {
                let outline_poly_points = vec![
                    (0.0, -0.64),
                    (0.44, -0.44),
                    (0.64, 0.0),
                    (0.44, 0.44),
                    (0.0, 0.64),
                    (-0.44, 0.44),
                    (-0.64, 0.0),
                    (-0.44, -0.44),
                    (0.0, -0.64),
                ]
                    .into_iter()
                    .map(|(poly_x, poly_y)| (x + poly_x, y + poly_y))
                    .collect();
                let outline_poly_style = PolyStyle::default()
                    .fill(DARK_COLOR)
                    .stroke(OUTLINE_COLOR)
                    .stroke_width(0.07)
                    .opacity(opacity);
                self.poly(outline_poly_points, Some(outline_poly_style));

                let interior_poly_points = vec![
                    (0.0, -0.52),
                    (0.36, -0.36),
                    (0.52, 0.0),
                    (0.36, 0.36),
                    (0.0, 0.52),
                    (-0.36, 0.36),
                    (-0.52, 0.0),
                    (-0.36, -0.36),
                    (0.0, -0.52),
                ]
                    .into_iter()
                    .map(|(poly_x, poly_y)| (x + poly_x, y + poly_y))
                    .collect();
                let interior_poly_style = PolyStyle::default()
                    .fill(LIGHT_COLOR)
                    .stroke("transparent")
                    .opacity(opacity);
                self.poly(interior_poly_points, Some(interior_poly_style));

                let storage_rect_style = RectStyle::default()
                    .fill(GRAY_COLOR)
                    .stroke(DARK_COLOR)
                    .stroke_width(0.1)
                    .opacity(opacity);
                self.rect(x - 0.36, y - 0.36, 0.72, 0.72, Some(storage_rect_style));

                let mineral_rect_style = RectStyle::default()
                    .fill(WHITE_COLOR)
                    .stroke("transparent")
                    .opacity(opacity);
                self.rect(x - 0.2, y - 0.2, 0.4, 0.4, Some(mineral_rect_style));
            }
            StructureType::Container => {
                let container_rect = RectStyle::default()
                    .fill(GRAY_COLOR)
                    .stroke(DARK_COLOR)
                    .stroke_width(0.1)
                    .opacity(opacity);
                self.rect(x - 0.25, y - 0.3, 0.5, 0.6, Some(container_rect));

                let energy_rect = RectStyle::default().fill(ENERGY_COLOR).opacity(opacity);
                self.rect(x - 0.192, y + 0.04, 0.39, 0.23, Some(energy_rect));
            }
            StructureType::Nuker => {
                let outline_poly_points = vec![
                    (0.0, -1.0),
                    (-0.47, 0.2),
                    (-0.5, 0.5),
                    (0.5, 0.5),
                    (0.47, 0.2),
                    (0.0, -1.0),
                ]
                    .into_iter()
                    .map(|(poly_x, poly_y)| (x + poly_x, y + poly_y))
                    .collect();
                let outline_poly_style = PolyStyle::default()
                    .fill(DARK_COLOR)
                    .stroke(OUTLINE_COLOR)
                    .stroke_width(0.07)
                    .opacity(opacity);
                self.poly(outline_poly_points, Some(outline_poly_style));

                let energy_poly_points =
                    vec![(0.0, -0.75), (-0.35, 0.2), (0.35, 0.2), (0.0, -0.75)]
                        .into_iter()
                        .map(|(poly_x, poly_y)| (x + poly_x, y + poly_y))
                        .collect();
                let energy_poly_style = PolyStyle::default()
                    .fill(ENERGY_COLOR)
                    .stroke(OUTLINE_COLOR)
                    .stroke_width(0.01)
                    .opacity(opacity);
                self.poly(energy_poly_points, Some(energy_poly_style));

                let ghodium_rect_style = RectStyle::default().fill(WHITE_COLOR).opacity(opacity);
                self.rect(x - 0.35, y + 0.3, 0.7, 0.1, Some(ghodium_rect_style));
            }
            StructureType::Factory => {
                // Outline
                let outline_points = vec![
                    (-0.68, -0.11),
                    (-0.84, -0.18),
                    (-0.84, -0.32),
                    (-0.44, -0.44),
                    (-0.32, -0.84),
                    (-0.18, -0.84),
                    (-0.11, -0.68),
                    (0.11, -0.68),
                    (0.18, -0.84),
                    (0.32, -0.84),
                    (0.44, -0.44),
                    (0.84, -0.32),
                    (0.84, -0.18),
                    (0.68, -0.11),
                    (0.68, 0.11),
                    (0.84, 0.18),
                    (0.84, 0.32),
                    (0.44, 0.44),
                    (0.32, 0.84),
                    (0.18, 0.84),
                    (0.11, 0.68),
                    (-0.11, 0.68),
                    (-0.18, 0.84),
                    (-0.32, 0.84),
                    (-0.44, 0.44),
                    (-0.84, 0.32),
                    (-0.84, 0.18),
                    (-0.68, 0.11),
                    (-0.68, -0.11),
                ]
                    .into_iter()
                    .map(|(poly_x, poly_y)| (x + poly_x, y + poly_y))
                    .collect();
                let outline_style = PolyStyle::default()
                    .stroke(OUTLINE_COLOR)
                    .stroke_width(0.07)
                    .opacity(opacity);
                self.poly(outline_points, Some(outline_style));

                // Outer circle.
                let outer_circle_style = CircleStyle::default()
                    .radius(0.65)
                    .fill(FACTORY_OUTER_CIRCLE_FILL_COLOR)
                    .stroke(FACTORY_PARTS_COLOR)
                    .stroke_width(0.035)
                    .opacity(opacity);
                self.circle(x, y, Some(outer_circle_style));

                let spikes_points = vec![
                    (-0.4, -0.1),
                    (-0.8, -0.2),
                    (-0.8, -0.3),
                    (-0.4, -0.4),
                    (-0.3, -0.8),
                    (-0.2, -0.8),
                    (-0.1, -0.4),
                    (0.1, -0.4),
                    (0.2, -0.8),
                    (0.3, -0.8),
                    (0.4, -0.4),
                    (0.8, -0.3),
                    (0.8, -0.2),
                    (0.4, -0.1),
                    (0.4, 0.1),
                    (0.8, 0.2),
                    (0.8, 0.3),
                    (0.4, 0.4),
                    (0.3, 0.8),
                    (0.2, 0.8),
                    (0.1, 0.4),
                    (-0.1, 0.4),
                    (-0.2, 0.8),
                    (-0.3, 0.8),
                    (-0.4, 0.4),
                    (-0.8, 0.3),
                    (-0.8, 0.2),
                    (-0.4, 0.1),
                ]
                    .into_iter()
                    .map(|(poly_x, poly_y)| (x + poly_x, y + poly_y))
                    .collect();
                let spikes_style = PolyStyle::default()
                    .fill(GRAY_COLOR)
                    .stroke(FACTORY_PARTS_COLOR)
                    .stroke_width(0.04)
                    .opacity(opacity);
                self.poly(spikes_points, Some(spikes_style));

                // Factory level circle.
                let factory_level_circle_style = CircleStyle::default()
                    .radius(0.54)
                    .fill(FACTORY_LEVEL_CIRCLE_FILL_COLOR)
                    .stroke(FACTORY_PARTS_COLOR)
                    .stroke_width(0.04)
                    .opacity(opacity);
                self.circle(x, y, Some(factory_level_circle_style));

                let factory_level_gaps_points = vec![
                    (0.0, 0.0),
                    (-0.08, -0.52),
                    (0.06643048934977405, -0.5219070703532858),
                    (0.0, 0.0),
                    (0.4698280289234841, -0.23677335837858496),
                    (0.5168912703137297, -0.0980990044468282),
                    (0.0, 0.0),
                    (0.3703696907420818, 0.37366601689157475),
                    (0.25302588419222044, 0.46127855134261886),
                    (0.0, 0.0),
                    (-0.24092697164209015, 0.4677116572583705),
                    (-0.36051267384944263, 0.3831848274578801),
                    (0.0, 0.0),
                    (-0.5192707480234756, -0.08460431577136027),
                    (-0.47583497000628144, -0.22445730400038488),
                ]
                    .into_iter()
                    .map(|(poly_x, poly_y)| (x + poly_x, y + poly_y))
                    .collect();
                let factory_level_gaps_style = PolyStyle::default()
                    .fill(FACTORY_PARTS_COLOR)
                    .stroke("transparent")
                    .opacity(opacity);
                self.poly(factory_level_gaps_points, Some(factory_level_gaps_style));

                // Inner black circle.
                let inner_circle_style = CircleStyle::default()
                    .radius(0.42)
                    .fill(FACTORY_PARTS_COLOR)
                    .opacity(opacity);
                self.circle(x, y, Some(inner_circle_style));

                let mineral_rect_style = RectStyle::default().fill(WHITE_COLOR).opacity(opacity);
                self.rect(x - 0.24, y - 0.24, 0.48, 0.24, Some(mineral_rect_style));

                let energy_rect_style = RectStyle::default().fill(ENERGY_COLOR).opacity(opacity);
                self.rect(x - 0.24, y, 0.48, 0.24, Some(energy_rect_style));
            }
            _ => {}
        };
    }

    pub fn structure_roomxy(&mut self, xy: RoomXY, structure_type: StructureType, opacity: f32) {
        self.structure(xy.x.u8() as f32, xy.y.u8() as f32, structure_type, opacity)
    }
}