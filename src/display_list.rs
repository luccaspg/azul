use webrender::api::*;
use constraints::{DisplayRect, CssConstraint};
use ui_description::{UiDescription, CssConstraintList};
use cassowary::{Constraint, Solver};

use css_parser::*;

pub(crate) struct DisplayList {
	pub(crate) rectangles: Vec<DisplayRectangle>
}

pub(crate) struct DisplayRectangle {
	/// The actual rectangle
	pub(crate) rect: DisplayRect,
	/// The constraints to be solved
	pub(crate) constraints: Vec<CssConstraint>,
	/// Background color of this rectangle
	pub(crate) background_color: Option<ColorU>,
	/// Shadow color
	pub(crate) shadow: Option<Shadow>,
	/// Gradient (location) + stops
	pub(crate) gradient: Option<(Gradient, Vec<GradientStop>)>,
	/// Opacity of this rectangle
	pub(crate) opacity: Option<f32>,
	/// Border
	pub(crate) border: Option<(BorderWidths, BorderDetails)>,
	/// border radius
	pub(crate) border_radius: Option<BorderRadius>,
}

impl DisplayRectangle {
	/// Returns an uninitialized rectangle
	#[inline]
	pub(crate) fn new() -> Self {
		Self {
			rect: DisplayRect::new(),
			constraints: Vec::new(),
			background_color: None,
			shadow: None,
			gradient: None,
			opacity: None,
			border: None,
			border_radius: None,
		}
	}
}

/*
pub struct BorderWidths {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}
*/
/*
pub struct NormalBorder {
    pub left: BorderSide,
    pub right: BorderSide,
    pub top: BorderSide,
    pub bottom: BorderSide,
    pub radius: BorderRadius,
}
*/
/*
pub struct BorderSide {
    pub color: ColorF,
    pub style: BorderStyle,
}
*/
impl DisplayList {

	pub fn new_from_ui_description(ui_description: &UiDescription) -> Self {

		let rects = ui_description.styled_nodes.iter().filter_map(|node| {

			// TODO: currently only styles divs
			if node.node.as_element().is_none() {
				return None;
			}

			let mut rect = DisplayRectangle::new();
			let mut css_constraints = Vec::<CssConstraint>::new();
			parse_css(&node.css_constraints, &mut rect, &mut css_constraints);
			rect.constraints = css_constraints;

			Some(rect)

		}).collect();

		Self {
			rectangles: rects,
		}
	}

	pub fn into_display_list_builder(&self, pipeline_id: PipelineId, layout_size: LayoutSize, solver: &mut Solver)
	-> DisplayListBuilder
	{
		let mut builder = DisplayListBuilder::new(pipeline_id, layout_size);

		for rect in &self.rectangles {

			// TODO: get these constraints to work properly
			let cassowary_constraints = css_constraints_to_cassowary_constraints(&rect.rect, &rect.constraints);
			solver.add_constraints(&cassowary_constraints).unwrap();

			for change in solver.fetch_changes() {
				println!("change: - {:?}", change);
			}

			let bounds = LayoutRect::new(
			    LayoutPoint::new(0.0, 0.0),
			    LayoutSize::new(200.0, 200.0),
			);

			let clip = if let Some(border_radius) = rect.border_radius {
				LocalClip::RoundedRect(bounds, ComplexClipRegion {
				    rect: bounds,
				    radii: border_radius,
				    mode: ClipMode::Clip,
				})
			} else {
				LocalClip::Rect(bounds)
			};

			let info = LayoutPrimitiveInfo {
				rect: bounds,
				is_backface_visible: false,
				tag: None, // todo: for hit testing !!!
			    local_clip: clip,
			};

			let opacity = 34.0;
			let opacity_key = PropertyBindingKey::new(43); // arbitrary magic number
			let property_key = PropertyBindingKey::new(42); // arbitrary magic number

			let filters = vec![
			    FilterOp::Opacity(PropertyBinding::Binding(opacity_key), opacity),
			];

			builder.push_stacking_context(
			    &info,
			    ScrollPolicy::Scrollable,
			    Some(PropertyBinding::Binding(property_key)),
			    TransformStyle::Flat,
			    None,
			    MixBlendMode::Normal,
			    filters,
			);

			builder.push_rect(&info, rect.background_color.unwrap_or(ColorU { r: 255, g: 0, b: 0, a: 255 }).into());

			if let Some((border_widths, mut border_details)) = rect.border {
				if let Some(border_radius) = rect.border_radius {
					if let BorderDetails::Normal(ref mut n) = border_details {
						n.radius = border_radius;
					}
				}
				builder.push_border(&info, border_widths, border_details);
			}

			builder.pop_stacking_context();
		}


		builder
	}
}

macro_rules! parse {
    ($id:ident, $key:expr, $replace:expr, $func:tt, $constraint_list:ident) => (
    	if let Some($id) = $constraint_list.get($key) {
    		match $func($id) {
    			Ok(r) => { $replace = Some(r); },
    			Err(e) => { println!("ERROR - invalid {:?}: {:?}", e, $key); }
    		}
    	}
    )
}

macro_rules! parse_css_size {
	($id:ident, $key:expr, $func:tt, $css_constraints:ident, $constraint_list:ident, $wrapper:path) => (
		if let Some($id) = $constraint_list.get($key) {
			match $func($id) {
				Ok(w) => { $css_constraints.push(CssConstraint::Size($wrapper(w.to_pixels()))); },
				Err(e) => { println!("ERROR - invalid {:?}: {:?}", e, $key); }
			}
		}
	)
}

/*
macro_rules! parse_css_padding {
	($id:ident, $key:expr, $func:tt, $css_constraints:ident, $constraint_list:ident, $wrapper:path, $variable:ident) => (
		if let Some($id) = $constraint_list.get($key) {
			match $func($id) {
				Ok(w) => { $css_constraints.push(CssConstraint::Padding($wrapper($variable))); },
				Err(e) => { println!("ERROR - invalid {:?}: {:?}", e, $key); }
			}
		}
	)
}
*/

/// Populate the constraint list
fn parse_css(constraint_list: &CssConstraintList, rect: &mut DisplayRectangle, css_constraints: &mut Vec<CssConstraint>)
{
	use constraints::{SizeConstraint, PaddingConstraint};

	let constraint_list = &constraint_list.list;

	parse!(radius, "border-radius", rect.border_radius, parse_css_border_radius, constraint_list);
	parse!(background_color, "background-color", rect.background_color, parse_css_background_color, constraint_list);
	parse!(border, "border", rect.border, parse_css_border, constraint_list);

	parse_css_size!(width, "width", parse_pixel_value, css_constraints, constraint_list, SizeConstraint::Width);
	parse_css_size!(height, "height", parse_pixel_value, css_constraints, constraint_list, SizeConstraint::Height);
	parse_css_size!(min_height, "min-height", parse_pixel_value, css_constraints, constraint_list, SizeConstraint::MinHeight);
	parse_css_size!(min_width, "min-width", parse_pixel_value, css_constraints, constraint_list, SizeConstraint::MinWidth);

}

fn css_constraints_to_cassowary_constraints(rect: &DisplayRect, css: &Vec<CssConstraint>)
-> Vec<Constraint>
{
	use self::CssConstraint::*;

	css.iter().flat_map(|constraint|
		match *constraint {
			Size(ref c) => { c.build(&rect, 100.0) }
			Padding(ref p) => { p.build(&rect, 50.0, 10.0) }
		}
	).collect()
}