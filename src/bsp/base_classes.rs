//! A collection of useful base classes when working with a BSP workflow.

use fgd::{IntBool, IntBoolOverride, Srgb};

use crate::*;

pub struct BspBaseClassesPlugin;
impl Plugin for BspBaseClassesPlugin {
	fn build(&self, app: &mut App) {
		#[rustfmt::skip]
		app
			.register_type::<BspSolidEntity>()
			.register_type::<BspWorldspawn>()
			.register_type::<BspLight>()
		;
	}
}

/// Contains properties used by the `ericw-tools` compiler for any entity with a brush model.
#[derive(BaseClass, Component, Reflect, Debug, Clone, SmartDefault, Serialize, Deserialize)]
#[reflect(Component, Default, Serialize, Deserialize)]
#[no_register]
pub struct BspSolidEntity {
	/// `ericw-tools` `qbsp`
	///
	/// Generates an `LMSHIFT` BSPX lump for use by a light util. Note that both scaled and unscaled lighting will normally be used.
	pub _lmscale: Option<u32>,
	/// `ericw-tools` `qbsp`
	///
	/// Set to 1 to save mirrored inside faces for brush models, so when the player view is inside the model, they will still see the faces. (e.g. for func_water, or func_illusionary)
	pub _mirrorinside: IntBool,

	/// `ericw-tools` `light`
	///
	/// `worldspawn`: Set a global minimum light level of this value across the whole map.
	/// This is an easy way to eliminate completely dark areas of the level, however you may lose some contrast as a result, so use with care. Default 0.
	///
	/// `model entity`: Set the minimum light level for any surface of the brush model. Default 0.
	pub _minlight: f32,

	/// `ericw-tools` `light`
	///
	/// Specify red(r), green(g) and blue(b) components for the colour of the minlight. RGB component values are between 0 and 255 (between 0 and 1 is also accepted).
	/// Default is white light ("255 255 255").
	#[default(Srgb::WHITE_255)]
	pub _minlight_color: Srgb,

	/// `ericw-tools` `light`
	///
	/// Faces with the given texture are excluded from receiving minlight on this brush model.
	pub _minlight_exclude: Option<String>,

	/// `ericw-tools` `light`
	///
	/// If set to 1, this model will cast shadows on other models and itself (i.e. "_shadow" implies "_shadowself").
	/// Note that this doesn’t magically give Quake dynamic lighting powers, so the shadows will not move if the model moves.
	/// Set to -1 on func_detail/func_group to prevent them from casting shadows. Default 0.
	pub _shadow: IntBool, // This is IntBool because func_detail and func_group get compiled into worldspawn, so will be removed

	/// `ericw-tools` `light`
	///
	/// If set to 1, this model will cast shadows on itself if one part of the model blocks the light from another model surface.
	/// This can be a better compromise for moving models than full shadowing. Default 0.
	pub _shadowself: IntBool,

	/// `ericw-tools` `light`
	///
	/// If set to 1, this model will cast shadows on the world only (not other brush models).
	pub _shadowworldonly: IntBool,

	/// `ericw-tools` `light`
	///
	/// If set to 1, this model casts a shadow that can be switched on/off using QuakeC.
	/// To make this work, a lightstyle is automatically assigned and stored in a key called "switchshadstyle", which the QuakeC will need to read and call the "lightstyle()" builtin with "a" or "m" to switch the shadow on or off.
	/// Entities sharing the same targetname, and with "_switchableshadow" set to 1, will share the same lightstyle.
	pub _switchableshadow: IntBool,

	/// `ericw-tools` `light`
	///
	/// `worldspawn`: 1 enables dirtmapping (ambient occlusion) on all lights, borrowed from q3map2. This adds shadows to corners and crevices.
	/// You can override the global setting for specific lights with the "_dirt" light entity key or "_sunlight_dirt", "_sunlight2_dirt", and "_minlight_dirt" worldspawn keys.
	/// Default is no dirtmapping (-1).
	///
	/// `model entity`: For brush models, -1 prevents dirtmapping on the brush model. Useful it the brush model touches or sticks into the world, and you want to those ares from turning black. Default 0.
	pub _dirt: IntBoolOverride,

	/// `ericw-tools` `light`
	///
	/// 1 enables phong shading on this model with a default _phong_angle of 89 (softens columns etc).
	pub _phong: IntBool,

	/// `ericw-tools` `light`
	///
	/// Enables phong shading on faces of this model with a custom angle. Adjacent faces with normals this many degrees apart (or less) will be smoothed.
	/// Consider setting "_anglescale" to "1" on lights or worldspawn to make the effect of phong shading more visible.
	/// Use the "-phongdebug" command-line flag to save the interpolated normals to the lightmap for previewing (use "r_lightmap 1" or "gl_lightmaps 1" in your engine to preview.)
	#[default(89.)]
	pub _phong_angle: f32,

	/// `ericw-tools` `light`
	///
	/// Optional key for setting a different angle threshold for concave joints.
	/// A pair of faces will either use "_phong_angle" or "_phong_angle_concave" as the smoothing threshold, depending on whether the joint between the faces is concave or not.
	/// "_phong_angle(_concave)" is the maximum angle (in degrees) between the face normals that will still cause the pair of faces to be smoothed.
	/// The minimum setting for "_phong_angle_concave" is 1, this should make all concave joints non-smoothed (unless they’re less than 1 degree apart, almost a flat plane.)
	/// If it’s 0 or unset, the same value as "_phong_angle" is used.
	pub _phong_angle_concave: Option<f32>,

	/// `ericw-tools` `light`
	///
	/// 1 makes a model receive minlight only, ignoring all lights / sunlight. Could be useful on rotators / trains.
	pub _lightignore: IntBool,
}

/// Contains properties used by the `ericw-tools` compiler for the `worldspawn` entity.
#[derive(BaseClass, Component, Reflect, Debug, Clone, SmartDefault, Serialize, Deserialize)]
#[reflect(Component, Default, Serialize, Deserialize)]
#[require(BspSolidEntity)]
#[no_register]
pub struct BspWorldspawn {
	/// `ericw-tools` `light`
	///
	/// Scales the fade distance of all lights by a factor of n. If n > 1 lights fade more quickly with distance and if n < 1, lights fade more slowly with distance and light reaches further.
	#[default(1.)]
	pub _dist: f32,

	/// `ericw-tools` `light`
	///
	/// Scales the brightness range of all lights without affecting their fade discance. Values of n > 0.5 makes lights brighter and n < 0.5 makes lights less bright. The same effect can be achieved on individual lights by adjusting both the "light" and "wait" attributes.
	#[default(0.5)]
	pub _range: f32,

	/// `ericw-tools` `light`
	///
	/// Set the brightness of the sunlight coming from an unseen sun in the sky. Sky brushes (or more accurately bsp leafs with sky contents) will emit sunlight at an angle specified by the "_sun_mangle" key. Default 0.
	pub _sunlight: f32,

	/// `ericw-tools` `light`
	///
	/// Set the scaling of sunlight brightness due to the angle of incidence with a surface (more detailed explanation in the "_anglescale" light entity key).
	#[default(0.5)]
	pub _anglescale: f32,

	/// `ericw-tools` `light`
	///
	/// Specifies the direction of sunlight using yaw, pitch and roll in degrees. Yaw specifies the angle around the Z-axis from 0 to 359 degrees and pitch specifies the angle from 90 (shining straight up) to -90 (shining straight down from above). Roll has no effect, so use any value (e.g. 0). Default is straight down ("0 -90 0").
	#[default(vec3(0., -90., 0.))]
	pub _sunlight_mangle: Vec3,

	/// `ericw-tools` `light`
	///
	/// Specifies the penumbra width, in degrees, of sunlight. Useful values are 3-4 for a gentle soft edge, or 10-20+ for more diffuse sunlight. Default is 0.
	pub _sunlight_penumbra: f32,

	/// `ericw-tools` `light`
	///
	/// Specify red(r), green(g) and blue(b) components for the color of the sunlight. RGB component values are between 0 and 255 (between 0 and 1 is also accepted). Default is white light ("255 255 255").
	#[default(Srgb::WHITE_255)]
	pub _sunlight_color: Srgb,

	/// `ericw-tools` `light`
	///
	/// Set the brightness of a dome of lights arranged around the upper hemisphere. (i.e. ambient light, coming from above the horizon). Default 0.
	pub _sunlight2: f32,

	/// `ericw-tools` `light`
	///
	/// Specifies the colour of _sunlight2, same format as "_sunlight_color". Default is white light ("255 255 255").
	#[default(Srgb::WHITE_255)]
	pub _sunlight2_color: Srgb,

	/// `ericw-tools` `light`
	///
	/// Same as "_sunlight2", but for the bottom hemisphere (i.e. ambient light, coming from below the horizon). Combine "_sunlight2" and "_sunlight3" to have light coming equally from all directions, e.g. for levels floating in the clouds. Default 0.
	pub _sunlight3: f32,

	/// `ericw-tools` `light`
	///
	/// Specifies the colour of "_sunlight3". Default is white light ("255 255 255").
	#[default(Srgb::WHITE_255)]
	pub _sunlight3_color: Srgb,

	/// `ericw-tools` `light`
	///
	/// 1 enables dirtmapping (ambient occlusion) on sunlight, -1 to disable (making it illuminate the dirtmapping shadows). Default is to use the value of "_dirt".
	pub _sunlight_dirt: IntBoolOverride,

	/// `ericw-tools` `light`
	///
	/// 1 enables dirtmapping (ambient occlusion) on sunlight2/3, -1 to disable. Default is to use the value of "_dirt".
	pub _sunlight2_dirt: IntBoolOverride,

	/// `ericw-tools` `light`
	///
	/// 1 enables dirtmapping (ambient occlusion) on minlight, -1 to disable. Default is to use the value of "_dirt".
	pub _minlight_dirt: IntBoolOverride,

	/// `ericw-tools` `light`
	///
	/// Choose between ordered (0, default) and randomized (1) dirtmapping.
	pub _dirtmode: DirtMode,

	/// `ericw-tools` `light`
	///
	/// Maximum depth of occlusion checking for dirtmapping, default 128.
	#[default(128.)]
	pub _dirtdepth: f32,

	/// `ericw-tools` `light`
	///
	/// Scale factor used in dirt calculations, default 1. Lower values (e.g. 0.5) make the dirt fainter, 2.0 would create much darker shadows.
	#[default(1.)]
	pub _dirtscale: f32,

	/// `ericw-tools` `light`
	///
	/// Exponent used in dirt calculation, default 1. Lower values (e.g. 0.5) make the shadows darker and stretch further away from corners.
	#[default(1.)]
	pub _dirtgain: f32,

	/// `ericw-tools` `light`
	///
	/// Cone angle in degrees for occlusion testing, default 88. Allowed range 1-90. Lower values can avoid unwanted dirt on arches, pipe interiors, etc.
	#[default(88.)]
	pub _dirtangle: f32,

	/// `ericw-tools` `light`
	///
	/// Forces all surfaces+submodels to use this specific lightmap scale. Removes "LMSHIFT" field.
	pub _lightmap_scale: Option<f32>,

	/// `ericw-tools` `light`
	///
	/// 1 enables bounce lighting, disabled by default.
	pub _bounce: IntBool,

	/// `ericw-tools` `light`
	///
	/// Scales brightness of bounce lighting, default 1.
	#[default(1.)]
	pub _bouncescale: f32,

	/// `ericw-tools` `light`
	///
	/// Weight for bounce lighting to use texture colors from the map: 0=ignore map textures (default), 1=multiply bounce light color by texture color.
	pub _bouncecolorscale: f32,

	/// `ericw-tools` `light`
	///
	/// 1 makes styled lights bounce (e.g. flickering or switchable lights), default is 0, they do not bounce.
	pub _bouncestyled: IntBool,

	/// `ericw-tools` `light`
	///
	/// When set to 1, spotlight falloff is calculated from the distance to the targeted info_null. Ignored when "_falloff" is not 0. Default 0.
	pub _spotlightautofalloff: IntBool,
}

#[derive(FgdType, Reflect, Debug, Clone, Default, Serialize, Deserialize)]
#[number_key]
pub enum DirtMode {
	#[default]
	Ordered = 0,
	Randomized = 1,
}

/// Contains properties used by the `ericw-tools` compiler for any entity with a classname starting with the first five letters "light". E.g. "light", "light_globe", "light_flame_small_yellow", etc.
#[derive(BaseClass, Component, Reflect, Debug, Clone, SmartDefault, Serialize, Deserialize)]
#[reflect(Component, Default, Serialize, Deserialize)]
#[no_register]
pub struct BspLight {
	/// Set the light intensity. Negative values are also allowed and will cause the entity to subtract light cast by other entities. Default 300.
	#[default(300.)]
	pub light: f32,

	/// Scale the fade distance of the light by the value specified. Values of n > 1 make the light fade more quickly with distance, and values < 1 make the light fade more slowly (and thus reach further). Default 1.
	#[default(1.)]
	pub wait: f32,

	/// The attenuation formula for the light.
	pub delay: BspLightAttenuation,

	/// Sets the distance at which the light drops to 0, in map units.
	///
	/// In this mode, "wait" is ignored and "light" only controls the brightness at the center of the light, and no longer affects the falloff distance.
	///
	/// Only supported on linear attenuation (delay 0) lights currently.
	pub _falloff: Option<f32>,

	/// Specify red(r), green(g) and blue(b) components for the colour of the light. RGB component values are between 0 and 255 (between 0 and 1 is also accepted). Default is white light ("255 255 255").
	#[default(Srgb::WHITE_255)]
	pub _color: Srgb,

	/// Turns the light into a spotlight, with the direction of light being towards another entity with it’s "targetname" key set to this value.
	pub target: Option<String>,

	/// Turns the light into a spotlight and specifies the direction of light using yaw, pitch and roll in degrees.
	/// Yaw specifies the angle around the Z-axis from 0 to 359 degrees and pitch specifies the angle from 90 (straight up) to -90 (straight down).
	/// Roll has no effect, so use any value (e.g. 0). Often easier than the "target" method.
	pub mangle: Vec3,

	/// Specifies the angle in degrees for a spotlight cone. Default 40.
	#[default(40.)]
	pub angle: f32,

	/// Specifies the angle in degrees for an inner spotlight cone (must be less than the "angle" cone. Creates a softer transition between the full brightness of the inner cone to the edge of the outer cone. Default 0 (disabled).
	pub _softangle: f32,

	/// Turns the light into a switchable light, toggled by another entity targeting it’s name.
	pub targetname: Option<String>,

	/// Set the animated light style. Default 0.
	#[default(LightmapStyle::NORMAL)]
	pub style: LightmapStyle,

	/// Sets a scaling factor for how much influence the angle of incidence of light on a surface has on the brightness of the surface.
	/// Value must be between 0.0 and 1.0. Smaller values mean less attenuation, with zero meaning that angle of incidence has no effect at all on the brightness. Default 0.5.
	#[default(0.5)]
	pub _anglescale: f32,

	/// Override the global "_dirtscale" setting to change how this light is affected by dirtmapping (ambient occlusion). See descriptions of this key in the worldspawn section.
	pub _dirtscale: Option<f32>,

	/// Override the global "_dirtgain" setting to change how this light is affected by dirtmapping (ambient occlusion). See descriptions of this key in the worldspawn section.
	pub _dirtgain: Option<f32>,

	/// Overrides the worldspawn setting of "_dirt" for this particular light.
	/// -1 to disable dirtmapping (ambient occlusion) for this light, making it illuminate the dirtmapping shadows.
	/// 1 to enable ambient occlusion for this light. Default is to defer to the worldspawn setting.
	pub _dirt: IntBoolOverride,

	/// Split up the light into a sphere of randomly positioned lights within the radius denoted by this value (in world units).
	/// Useful to give shadows a wider penumbra. "_samples" specifies the number of lights in the sphere.
	/// The "light" value is automatically scaled down for most lighting formulas (except linear and non-additive minlight) to attempt to keep the brightness equal.
	/// Default is 0, do not split up lights.
	pub _deviance: f32,

	/// Number of lights to use for "_deviance". Default 16 (only used if "_deviance" is set).
	#[default(16)]
	pub _samples: u32,

	/// Makes surfaces with the given texture name emit light, by using this light as a template which is copied across those surfaces.
	/// Lights are spaced about 128 units (though possibly closer due to bsp splitting) apart and positioned 2 units above the surfaces.
	pub _surface: Option<String>,

	/// Controls the offset lights are placed above surfaces for "_surface" (world units). Default 2.
	#[default(2.)]
	pub _surface_offset: f32,

	/// For a surface light template (i.e. a light with "_surface" set), setting this to "1" makes each instance into a spotlight,
	/// with the direction of light pointing along the surface normal. In other words, it automatically sets "mangle" on each of the generated lights.
	pub _surface_spotlight: IntBool,

	/// Specifies that a light should project this texture. The texture must be used in the map somewhere.
	pub _project_texture: Option<String>,

	/// Specifies the yaw/pitch/roll angles for a texture projection (overriding mangle).
	pub _project_mangle: Option<Vec3>,

	/// Specifies the fov angle for a texture projection. Default 90.
	#[default(90.)]
	pub _project_fov: f32,

	/// Scales the amount of light that is contributed by bounces. Default is 1.0, 0.0 disables bounce lighting for this light.
	#[default(1.)]
	pub _bouncescale: f32,

	/// Set to 1 to make this entity a sun, as an alternative to using the sunlight worldspawn keys.
	/// If the light targets an info_null entity, the direction towards that entity sets sun direction.
	/// The light itself is disabled, so it can be placed anywhere in the map.
	///
	///
	/// The following light properties correspond to these sunlight settings:
	/// - light => _sunlight
	/// - mangle => _sunlight_mangle
	/// - deviance => _sunlight_penumbra
	/// - _color => _sunlight_color
	/// - _dirt => _sunlight_dirt
	/// - _anglescale => _anglescale
	pub _sun: IntBool,
}

/// How light fades over distance. Used in the `delay` property of light entities.
#[derive(FgdType, Reflect, Debug, Clone, Default, Serialize, Deserialize)]
#[number_key]
pub enum BspLightAttenuation {
	/// Linear attenuation (default)
	#[default]
	Linear = 0,
	/// 1/x attenuation
	Reciprocal = 1,
	/// 1/(x^2) attenuation
	ReciprocalSquare = 2,
	/// No attenuation (same brightness at any distance)
	None = 3,
	/// No attenuation, and like minlight
	/// it won’t raise the lighting above it’s light value.
	/// Unlike minlight, it will only affect surfaces within
	/// line of sight of the entity.
	LocalMinLight = 4,
	/// 1/(x^2) attenuation, but slightly more attenuated and
	/// without the extra bright effect that [`ReciprocalSquare`](BspLightDelay::ReciprocalSquare) has
	/// near the source.
	ReciprocalSquareTweaked = 5,
}
