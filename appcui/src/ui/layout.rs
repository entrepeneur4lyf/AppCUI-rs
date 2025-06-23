//! A layout system for positioning and sizing UI controls.
//!
//! The Layout component provides tools for arranging controls within containers.
//! It supports anchoring, docking, proportional sizing, and alignment options.

mod alignament;
mod anchors;
mod coordonate16;
mod coordonate;
mod dimension16;
mod dimension;
mod layout_parameters;
mod layout_mode;
mod parameter;
mod macros;
mod control_layout;
mod absolute_layout;
mod point_and_size_layout;
mod left_right_anchors_layout;
mod top_bottom_anchors_layout;
mod left_top_right_anchors_layout;
mod left_bottom_right_anchors_layout;
mod top_left_bottom_anchors_layout;
mod top_right_bottom_anchors_layout;
mod all_anchors_layout;
#[cfg(test)]
mod tests;

use alignament::Alignament;
use anchors::Anchors;
use layout_parameters::LayoutParameters;
use layout_mode::LayoutMode;
use parameter::Parameter;
use absolute_layout::AbsoluteLayout;
use point_and_size_layout::PointAndSizeLayout;
use left_right_anchors_layout::LeftRightAnchorsLayout;
use top_bottom_anchors_layout::TopBottomAnchorsLayout;
use left_top_right_anchors_layout::LeftTopRightAnchorsLayout;
use left_bottom_right_anchors_layout::LeftBottomRightAnchorsLayout;
use top_left_bottom_anchors_layout::TopLeftBottomAnchorsLayout;
use top_right_bottom_anchors_layout::TopRightBottomAnchorsLayout;
use all_anchors_layout::AllAnchorsLayout;
use macros::should_not_use;
pub use control_layout::Layout;
pub use coordonate::Coordonate;
pub use dimension::Dimension;
pub (in crate) use dimension16::Dimension16;
pub (in crate) use coordonate16::Coordonate16;
pub (in crate) use control_layout::ControlLayout;


