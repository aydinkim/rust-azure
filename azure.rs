/* automatically generated by rust-bindgen */

import libc::*;

type AzFloat = c_float;

type enum_AzSurfaceType = c_uint;
const AZ_SURFACE_DATA: u32 = 0_u32;
const AZ_SURFACE_D2D1_BITMAP: u32 = 1_u32;
const AZ_SURFACE_D2D1_DRAWTARGET: u32 = 2_u32;
const AZ_SURFACE_CAIRO: u32 = 3_u32;
const AZ_SURFACE_CAIRO_IMAGE: u32 = 4_u32;
const AZ_SURFACE_COREGRAPHICS_IMAGE: u32 = 5_u32;
const AZ_SURFACE_COREGRAPHICS_CGCONTEXT: u32 = 6_u32;
const AZ_SURFACE_SKIA: u32 = 7_u32;
const AZ_SURFACE_DUAL_DT: u32 = 8_u32;

type enum_AzSurfaceFormat = c_uint;
const AZ_FORMAT_B8G8R8A8: u32 = 0_u32;
const AZ_FORMAT_B8G8R8X8: u32 = 1_u32;
const AZ_FORMAT_R5G6B5: u32 = 2_u32;
const AZ_FORMAT_A8: u32 = 3_u32;

type enum_AzBackendType = c_uint;
const AZ_BACKEND_NONE: u32 = 0_u32;
const AZ_BACKEND_DIRECT2D: u32 = 1_u32;
const AZ_BACKEND_COREGRAPHICS: u32 = 2_u32;
const AZ_BACKEND_CAIRO: u32 = 3_u32;
const AZ_BACKEND_SKIA: u32 = 4_u32;

type enum_AzFontType = c_uint;
const AZ_FONT_DWRITE: u32 = 0_u32;
const AZ_FONT_GDI: u32 = 1_u32;
const AZ_FONT_MAC: u32 = 2_u32;
const AZ_FONT_SKIA: u32 = 3_u32;
const AZ_FONT_CAIRO: u32 = 4_u32;
const AZ_FONT_COREGRAPHICS: u32 = 5_u32;

type enum_AzNativeSurfaceType = c_uint;
const AZ_NATIVE_SURFACE_D3D10_TEXTURE: u32 = 0_u32;
const AZ_NATIVE_SURFACE_CAIRO_SURFACE: u32 = 1_u32;
const AZ_NATIVE_SURFACE_CGCONTEXT: u32 = 2_u32;

type enum_AzNativeFontType = c_uint;
const AZ_NATIVE_FONT_DWRITE_FONT_FACE: u32 = 0_u32;
const AZ_NATIVE_FONT_GDI_FONT_FACE: u32 = 1_u32;
const AZ_NATIVE_FONT_MAC_FONT_FACE: u32 = 2_u32;
const AZ_NATIVE_FONT_SKIA_FONT_FACE: u32 = 3_u32;
const AZ_NATIVE_FONT_CAIRO_FONT_FACE: u32 = 4_u32;

type enum_AzCompositionOp = c_uint;
const AZ_OP_OVER: u32 = 0_u32;
const AZ_OP_ADD: u32 = 1_u32;
const AZ_OP_ATOP: u32 = 2_u32;
const AZ_OP_OUT: u32 = 3_u32;
const AZ_OP_IN: u32 = 4_u32;
const AZ_OP_SOURCE: u32 = 5_u32;
const AZ_OP_DEST_IN: u32 = 6_u32;
const AZ_OP_DEST_OUT: u32 = 7_u32;
const AZ_OP_DEST_OVER: u32 = 8_u32;
const AZ_OP_DEST_ATOP: u32 = 9_u32;
const AZ_OP_XOR: u32 = 10_u32;
const AZ_OP_COUNT: u32 = 11_u32;

type enum_AzExtendMode = c_uint;
const AZ_EXTEND_CLAMP: u32 = 0_u32;
const AZ_EXTEND_REPEAT: u32 = 1_u32;
const AZ_EXTEND_REFLECT: u32 = 2_u32;

type enum_AzFillRule = c_uint;
const AZ_FILL_WINDING: u32 = 0_u32;
const AZ_FILL_EVEN_ODD: u32 = 1_u32;

type enum_AzAntialiasMode = c_uint;
const AZ_AA_NONE: u32 = 0_u32;
const AZ_AA_GRAY: u32 = 1_u32;
const AZ_AA_SUBPIXEL: u32 = 2_u32;

type enum_AzSnapping = c_uint;
const AZ_SNAP_NONE: u32 = 0_u32;
const AZ_SNAP_ALIGNED: u32 = 1_u32;

type enum_AzFilter = c_uint;
const AZ_FILTER_LINEAR: u32 = 0_u32;
const AZ_FILTER_POINT: u32 = 1_u32;

type enum_AzPatternType = c_uint;
const AZ_PATTERN_COLOR: u32 = 0_u32;
const AZ_PATTERN_SURFACE: u32 = 1_u32;
const AZ_PATTERN_LINEAR_GRADIENT: u32 = 2_u32;
const AZ_PATTERN_RADIAL_GRADIENT: u32 = 3_u32;

type enum_AzJoinStyle = c_uint;
const AZ_JOIN_BEVEL: u32 = 0_u32;
const AZ_JOIN_ROUND: u32 = 1_u32;
const AZ_JOIN_MITER: u32 = 2_u32;
const AZ_JOIN_MITER_OR_BEVEL: u32 = 3_u32;

type enum_AzCapStyle = c_uint;
const AZ_CAP_BUTT: u32 = 0_u32;
const AZ_CAP_ROUND: u32 = 1_u32;
const AZ_CAP_SQUARE: u32 = 2_u32;

type enum_AzSamplingBounds = c_uint;
const AZ_SAMPLING_UNBOUNDED: u32 = 0_u32;
const AZ_SAMPLING_BOUNDED: u32 = 1_u32;

type enum_AzSide = c_uint;
const AZ_eSideTop: u32 = 0_u32;
const AZ_eSideRight: u32 = 1_u32;
const AZ_eSideBottom: u32 = 2_u32;
const AZ_eSideLeft: u32 = 3_u32;

type struct__AzColor = {
    r: AzFloat,
    g: AzFloat,
    b: AzFloat,
    a: AzFloat,
};

type AzColor = struct__AzColor;

type struct__AzGradientStop = {
    offset: AzFloat,
    color: AzColor,
};

type AzGradientStop = struct__AzGradientStop;

type struct__AzIntRect = {
    x: int32_t,
    y: int32_t,
    width: int32_t,
    height: int32_t,
};

type AzIntRect = struct__AzIntRect;

type struct__AzRect = {
    x: AzFloat,
    y: AzFloat,
    width: AzFloat,
    height: AzFloat,
};

type AzRect = struct__AzRect;

type struct__AzIntPoint = {
    x: int32_t,
    y: int32_t,
};

type AzIntPoint = struct__AzIntPoint;

type struct__AzPoint = {
    x: AzFloat,
    y: AzFloat,
};

type AzPoint = struct__AzPoint;

type struct__AzIntSize = {
    width: int32_t,
    height: int32_t,
};

type AzIntSize = struct__AzIntSize;

type struct__AzSize = {
    width: AzFloat,
    height: AzFloat,
};

type AzSize = struct__AzSize;

type struct__AzMatrix = {
    _11: AzFloat,
    _12: AzFloat,
    _21: AzFloat,
    _22: AzFloat,
    _31: AzFloat,
    _32: AzFloat,
};

type AzMatrix = struct__AzMatrix;

type struct__AzDrawOptions = {
    mAlpha: AzFloat,
    fields: uint16_t,
};

type AzDrawOptions = struct__AzDrawOptions;

type struct__AzStrokeOptions = {
    mLineWidth: AzFloat,
    mMiterLimit: AzFloat,
    mDashPattern: *AzFloat,
    mDashLength: size_t,
    mDashOffset: AzFloat,
    fields: uint16_t,
};

type AzStrokeOptions = struct__AzStrokeOptions;

type struct__AzDrawSurfaceOptions = {
    fields: uint32_t,
};

type AzDrawSurfaceOptions = struct__AzDrawSurfaceOptions;

type AzGradientStopsRef = *c_void;

type AzDrawTargetRef = *c_void;

type AzPatternRef = *c_void;

type AzColorPatternRef = *c_void;

#[link_name="azure"]
native mod bindgen {

fn AzSanityCheck(/* FIXME: variadic function */);

fn AzCreateColorPattern(++arg0: *AzColor) -> AzColorPatternRef;

fn AzReleaseColorPattern(++arg0: AzColorPatternRef);

fn AzCreateDrawTargetForCairoSurface(++arg0: *cairo_surface_t) -> AzDrawTargetRef;

fn AzReleaseDrawTarget(++arg0: AzDrawTargetRef);

fn AzDrawTargetGetSize(++arg0: AzDrawTargetRef) -> AzIntSize;

fn AzDrawTargetFlush(++arg0: AzDrawTargetRef);

fn AzDrawTargetClearRect(++arg0: AzDrawTargetRef, ++arg1: *AzRect);

fn AzDrawTargetFillRect(++arg0: AzDrawTargetRef, ++arg1: *AzRect, ++arg2: AzPatternRef);

fn AzDrawTargetStrokeRect(++arg0: AzDrawTargetRef, ++arg1: *AzRect, ++arg2: AzPatternRef, ++arg3: *AzStrokeOptions, ++arg4: *AzDrawOptions);

fn AzDrawTargetStrokeLine(++arg0: AzDrawTargetRef, ++arg1: *AzPoint, ++arg2: *AzPoint, ++arg3: *AzPatternRef, ++arg4: *AzStrokeOptions, ++arg5: *AzDrawOptions);

}