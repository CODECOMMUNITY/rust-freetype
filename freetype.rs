/* automatically generated by rust-bindgen */
#[legacy_exports];

use libc::*;

type FT_Int16 = c_short;
type FT_UInt16 = c_ushort;
type FT_Int32 = c_int;
type FT_UInt32 = c_uint;
type FT_Fast = c_int;
type FT_UFast = c_uint;

type FT_Memory = *struct_FT_MemoryRec_;
type FT_Alloc_Func = *u8;
type FT_Free_Func = *u8;
type FT_Realloc_Func = *u8;

type struct_FT_MemoryRec_ = {
    user: *c_void,
    alloc: FT_Alloc_Func,
    free: FT_Free_Func,
    realloc: FT_Realloc_Func,
};

type FT_Stream = *struct_FT_StreamRec_;
type union_FT_StreamDesc_ = c_void /* FIXME: union type */;
type FT_StreamDesc = union_FT_StreamDesc_;
type FT_Stream_IoFunc = *u8;
type FT_Stream_CloseFunc = *u8;

type struct_FT_StreamRec_ = {
    base: *c_uchar,
    size: c_ulong,
    pos: c_ulong,
    descriptor: FT_StreamDesc,
    pathname: FT_StreamDesc,
    read: FT_Stream_IoFunc,
    close: FT_Stream_CloseFunc,
    memory: *c_void /* FT_Memory */,
    cursor: *c_uchar,
    limit: *c_uchar,
};

type FT_StreamRec = struct_FT_StreamRec_;
type FT_Pos = c_long;

type struct_FT_Vector_ = {
    x: FT_Pos,
    y: FT_Pos,
};

type FT_Vector = struct_FT_Vector_;

type struct_FT_BBox_ = {
    xMin: FT_Pos,
    yMin: FT_Pos,
    xMax: FT_Pos,
    yMax: FT_Pos,
};

type FT_BBox = struct_FT_BBox_;

type enum_FT_Pixel_Mode_ = c_uint;
const FT_PIXEL_MODE_NONE: u32 = 0_u32;
const FT_PIXEL_MODE_MONO: u32 = 1_u32;
const FT_PIXEL_MODE_GRAY: u32 = 2_u32;
const FT_PIXEL_MODE_GRAY2: u32 = 3_u32;
const FT_PIXEL_MODE_GRAY4: u32 = 4_u32;
const FT_PIXEL_MODE_LCD: u32 = 5_u32;
const FT_PIXEL_MODE_LCD_V: u32 = 6_u32;
const FT_PIXEL_MODE_MAX: u32 = 7_u32;

type FT_Pixel_Mode = enum_FT_Pixel_Mode_;

type struct_FT_Bitmap_ = {
    rows: c_int,
    width: c_int,
    pitch: c_int,
    buffer: *c_uchar,
    num_grays: c_short,
    pixel_mode: c_char,
    palette_mode: c_char,
    palette: *c_void,
};

type FT_Bitmap = struct_FT_Bitmap_;

type struct_FT_Outline_ = {
    n_contours: c_short,
    n_points: c_short,
    points: *FT_Vector,
    tags: *c_char,
    contours: *c_short,
    flags: c_int,
};

type FT_Outline = struct_FT_Outline_;
type FT_Outline_MoveToFunc = *u8;
type FT_Outline_LineToFunc = *u8;
type FT_Outline_ConicToFunc = *u8;
type FT_Outline_CubicToFunc = *u8;

type struct_FT_Outline_Funcs_ = {
    move_to: FT_Outline_MoveToFunc,
    line_to: FT_Outline_LineToFunc,
    conic_to: FT_Outline_ConicToFunc,
    cubic_to: FT_Outline_CubicToFunc,
    shift: c_int,
    delta: FT_Pos,
};

type FT_Outline_Funcs = struct_FT_Outline_Funcs_;

type enum_FT_Glyph_Format_ = c_uint;
const FT_GLYPH_FORMAT_NONE: u32 = 0_u32;
const FT_GLYPH_FORMAT_COMPOSITE: u32 = 1668246896_u32;
const FT_GLYPH_FORMAT_BITMAP: u32 = 1651078259_u32;
const FT_GLYPH_FORMAT_OUTLINE: u32 = 1869968492_u32;
const FT_GLYPH_FORMAT_PLOTTER: u32 = 1886154612_u32;

type FT_Glyph_Format = enum_FT_Glyph_Format_;
type struct_FT_RasterRec_ = c_void;
type FT_Raster = *struct_FT_RasterRec_;

type struct_FT_Span_ = {
    x: c_short,
    len: c_ushort,
    coverage: c_uchar,
};

type FT_Span = struct_FT_Span_;
type FT_SpanFunc = *u8;
type FT_Raster_BitTest_Func = *u8;
type FT_Raster_BitSet_Func = *u8;

type struct_FT_Raster_Params_ = {
    target: *FT_Bitmap,
    source: *c_void,
    flags: c_int,
    gray_spans: FT_SpanFunc,
    black_spans: FT_SpanFunc,
    bit_test: FT_Raster_BitTest_Func,
    bit_set: FT_Raster_BitSet_Func,
    user: *c_void,
    clip_box: FT_BBox,
};

type FT_Raster_Params = struct_FT_Raster_Params_;
type FT_Raster_NewFunc = *u8;
type FT_Raster_DoneFunc = *u8;
type FT_Raster_ResetFunc = *u8;
type FT_Raster_SetModeFunc = *u8;
type FT_Raster_RenderFunc = *u8;

type struct_FT_Raster_Funcs_ = {
    glyph_format: FT_Glyph_Format,
    raster_new: FT_Raster_NewFunc,
    raster_reset: FT_Raster_ResetFunc,
    raster_set_mode: FT_Raster_SetModeFunc,
    raster_render: FT_Raster_RenderFunc,
    raster_done: FT_Raster_DoneFunc,
};

type FT_Raster_Funcs = struct_FT_Raster_Funcs_;
type FT_Bool = c_uchar;
type FT_FWord = c_short;
type FT_UFWord = c_ushort;
type FT_Char = c_char;
type FT_Byte = c_uchar;
type FT_Bytes = *FT_Byte;
type FT_Tag = FT_UInt32;
type FT_String = c_char;
type FT_Short = c_short;
type FT_UShort = c_ushort;
type FT_Int = c_int;
type FT_UInt = c_uint;
type FT_Long = c_long;
type FT_ULong = c_ulong;
type FT_F2Dot14 = c_short;
type FT_F26Dot6 = c_long;
type FT_Fixed = c_long;
type FT_Error = c_int;
type FT_Pointer = *c_void;
type FT_Offset = size_t;
type FT_PtrDist = ptrdiff_t;

type struct_FT_UnitVector_ = {
    x: FT_F2Dot14,
    y: FT_F2Dot14,
};

type FT_UnitVector = struct_FT_UnitVector_;

type struct_FT_Matrix_ = {
    xx: FT_Fixed,
    xy: FT_Fixed,
    yx: FT_Fixed,
    yy: FT_Fixed,
};

type FT_Matrix = struct_FT_Matrix_;

type struct_FT_Data_ = {
    pointer: *FT_Byte,
    length: FT_Int,
};

type FT_Data = struct_FT_Data_;
type FT_Generic_Finalizer = *u8;

type struct_FT_Generic_ = {
    data: *c_void,
    finalizer: FT_Generic_Finalizer,
};

type FT_Generic = struct_FT_Generic_;
type FT_ListNode = *struct_FT_ListNodeRec_;
type FT_List = *struct_FT_ListRec_;

type struct_FT_ListNodeRec_ = {
    prev: *c_void /* FT_ListNode */,
    next: *c_void /* FT_ListNode */,
    data: *c_void,
};

type FT_ListNodeRec = struct_FT_ListNodeRec_;

type struct_FT_ListRec_ = {
    head: *c_void /* FT_ListNode */,
    tail: *c_void /* FT_ListNode */,
};

type FT_ListRec = struct_FT_ListRec_;

type struct_FT_Glyph_Metrics_ = {
    width: FT_Pos,
    height: FT_Pos,
    horiBearingX: FT_Pos,
    horiBearingY: FT_Pos,
    horiAdvance: FT_Pos,
    vertBearingX: FT_Pos,
    vertBearingY: FT_Pos,
    vertAdvance: FT_Pos,
};

type FT_Glyph_Metrics = struct_FT_Glyph_Metrics_;

type struct_FT_Bitmap_Size_ = {
    height: FT_Short,
    width: FT_Short,
    size: FT_Pos,
    x_ppem: FT_Pos,
    y_ppem: FT_Pos,
};

type FT_Bitmap_Size = struct_FT_Bitmap_Size_;

type struct_FT_LibraryRec_ = c_void;
type FT_Library = *struct_FT_LibraryRec_;

type struct_FT_ModuleRec_ = c_void;
type FT_Module = *struct_FT_ModuleRec_;

type struct_FT_DriverRec_ = c_void;
type FT_Driver = *struct_FT_DriverRec_;

type struct_FT_RendererRec_ = c_void;
type FT_Renderer = *struct_FT_RendererRec_;

type FT_Face = *struct_FT_FaceRec_;
type FT_Size = *struct_FT_SizeRec_;
type FT_GlyphSlot = *struct_FT_GlyphSlotRec_;
type FT_CharMap = *struct_FT_CharMapRec_;

type enum_FT_Encoding_ = c_uint;
const FT_ENCODING_NONE: u32 = 0_u32;
const FT_ENCODING_MS_SYMBOL: u32 = 1937337698_u32;
const FT_ENCODING_UNICODE: u32 = 1970170211_u32;
const FT_ENCODING_SJIS: u32 = 1936353651_u32;
const FT_ENCODING_GB2312: u32 = 1734484000_u32;
const FT_ENCODING_BIG5: u32 = 1651074869_u32;
const FT_ENCODING_WANSUNG: u32 = 2002873971_u32;
const FT_ENCODING_JOHAB: u32 = 1785686113_u32;
const FT_ENCODING_MS_SJIS: u32 = 1936353651_u32;
const FT_ENCODING_MS_GB2312: u32 = 1734484000_u32;
const FT_ENCODING_MS_BIG5: u32 = 1651074869_u32;
const FT_ENCODING_MS_WANSUNG: u32 = 2002873971_u32;
const FT_ENCODING_MS_JOHAB: u32 = 1785686113_u32;
const FT_ENCODING_ADOBE_STANDARD: u32 = 1094995778_u32;
const FT_ENCODING_ADOBE_EXPERT: u32 = 1094992453_u32;
const FT_ENCODING_ADOBE_CUSTOM: u32 = 1094992451_u32;
const FT_ENCODING_ADOBE_LATIN_1: u32 = 1818326065_u32;
const FT_ENCODING_OLD_LATIN_2: u32 = 1818326066_u32;
const FT_ENCODING_APPLE_ROMAN: u32 = 1634889070_u32;

const FT_LOAD_DEFAULT: u32 = 0x0;
const FT_LOAD_NO_SCALE: u32 = (0x1 << 1) as u32;
const FT_LOAD_NO_HINTING: u32 = (0x1 << 2) as u32;
const FT_LOAD_RENDER: u32 = (0x1 << 3) as u32;
const FT_LOAD_NO_BITMAP: u32 = (0x1 << 4) as u32;
const FT_LOAD_VERTICAL_LAYOUT: u32 = (0x1 << 5) as u32;
const FT_LOAD_FORCE_AUTOHINT: u32 = (0x1 << 6) as u32;
const FT_LOAD_CROP_BITMAP: u32 = (0x1 << 7) as u32;
const FT_LOAD_PENDANTIC: u32 = (0x1 << 8) as u32;
const FT_LOAD_IGNORE_GLOBAL_ADVANCE_WIDTH: u32 = (0x1 << 9) as u32;
const FT_LOAD_NO_RECURSE: u32 = (0x1 << 10) as u32;
const FT_LOAD_IGNORE_TRANSFORM: u32 = (0x1 << 11) as u32;
const FT_LOAD_MONOCHROME: u32 = (0x1 << 12) as u32;
const FT_LOAD_LINEAR_DESIGN: u32 = (0x1 << 13) as u32;
const FT_LOAD_NO_AUTOHINT: u32 = (0x1 << 15) as u32;

type FT_Encoding = enum_FT_Encoding_;

type struct_FT_CharMapRec_ = {
    face: *c_void /* FT_Face */,
    encoding: FT_Encoding,
    platform_id: FT_UShort,
    encoding_id: FT_UShort,
};

type FT_CharMapRec = struct_FT_CharMapRec_;

type struct_FT_Face_InternalRec_ = c_void;
type FT_Face_Internal = *struct_FT_Face_InternalRec_;

type struct_FT_FaceRec_ = {
    num_faces: FT_Long,
    face_index: FT_Long,
    face_flags: FT_Long,
    style_flags: FT_Long,
    num_glyphs: FT_Long,
    family_name: *FT_String,
    style_name: *FT_String,
    num_fixed_sizes: FT_Int,
    available_sizes: *FT_Bitmap_Size,
    num_charmaps: FT_Int,
    charmaps: **c_void /* FT_CharMap */,
    generic: FT_Generic,
    bbox: FT_BBox,
    units_per_EM: FT_UShort,
    ascender: FT_Short,
    descender: FT_Short,
    height: FT_Short,
    max_advance_width: FT_Short,
    max_advance_height: FT_Short,
    underline_position: FT_Short,
    underline_thickness: FT_Short,
    glyph: *c_void /* FT_GlyphSlot */,
    size: *c_void /* FT_Size */,
    charmap: *c_void /* FT_CharMap */,
    driver: *c_void /* FT_Driver */,
    memory: *c_void /* FT_Memory */,
    stream: *c_void /* FT_Stream */,
    sizes_list: FT_ListRec,
    autohint: FT_Generic,
    extensions: *c_void,
    internal: *c_void /* FT_Face_Internal */,
};

type FT_FaceRec = struct_FT_FaceRec_;

type struct_FT_Size_InternalRec_ = c_void;
type FT_Size_Internal = *struct_FT_Size_InternalRec_;

type struct_FT_Size_Metrics_ = {
    x_ppem: FT_UShort,
    y_ppem: FT_UShort,
    x_scale: FT_Fixed,
    y_scale: FT_Fixed,
    ascender: FT_Pos,
    descender: FT_Pos,
    height: FT_Pos,
    max_advance: FT_Pos,
};

type FT_Size_Metrics = struct_FT_Size_Metrics_;

type struct_FT_SizeRec_ = {
    face: *c_void /* FT_Face */,
    generic: FT_Generic,
    metrics: FT_Size_Metrics,
    internal: *c_void /* FT_Size_Internal */,
};

type FT_SizeRec = struct_FT_SizeRec_;

type struct_FT_SubGlyphRec_ = c_void;
type FT_SubGlyph = *struct_FT_SubGlyphRec_;

type struct_FT_Slot_InternalRec_ = c_void;
type FT_Slot_Internal = *struct_FT_Slot_InternalRec_;

type struct_FT_GlyphSlotRec_ = {
    library: *c_void /* FT_Library */,
    face: *c_void /* FT_Face */,
    next: *c_void /* FT_GlyphSlot */,
    reserved: FT_UInt,
    generic: FT_Generic,
    metrics: FT_Glyph_Metrics,
    linearHoriAdvance: FT_Fixed,
    linearVertAdvance: FT_Fixed,
    advance: FT_Vector,
    format: FT_Glyph_Format,
    bitmap: FT_Bitmap,
    bitmap_left: FT_Int,
    bitmap_top: FT_Int,
    outline: FT_Outline,
    num_subglyphs: FT_UInt,
    subglyphs: *c_void /* FT_SubGlyph */,
    control_data: *c_void,
    control_len: c_long,
    lsb_delta: FT_Pos,
    rsb_delta: FT_Pos,
    other: *c_void,
    internal: *c_void /* FT_Slot_Internal */,
};

type FT_GlyphSlotRec = struct_FT_GlyphSlotRec_;

type struct_FT_Parameter_ = {
    tag: FT_ULong,
    data: FT_Pointer,
};

type FT_Parameter = struct_FT_Parameter_;

type struct_FT_Open_Args_ = {
    flags: FT_UInt,
    memory_base: *FT_Byte,
    memory_size: FT_Long,
    pathname: *FT_String,
    stream: *c_void /* FT_Stream */,
    driver: *c_void /* FT_Module */,
    num_params: FT_Int,
    params: *FT_Parameter,
};

type FT_Open_Args = struct_FT_Open_Args_;

type enum_FT_Size_Request_Type_ = c_uint;
const FT_SIZE_REQUEST_TYPE_NOMINAL: u32 = 0_u32;
const FT_SIZE_REQUEST_TYPE_REAL_DIM: u32 = 1_u32;
const FT_SIZE_REQUEST_TYPE_BBOX: u32 = 2_u32;
const FT_SIZE_REQUEST_TYPE_CELL: u32 = 3_u32;
const FT_SIZE_REQUEST_TYPE_SCALES: u32 = 4_u32;
const FT_SIZE_REQUEST_TYPE_MAX: u32 = 5_u32;

type FT_Size_Request_Type = enum_FT_Size_Request_Type_;

type struct_FT_Size_RequestRec_ = {
    _type: FT_Size_Request_Type,
    width: FT_Long,
    height: FT_Long,
    horiResolution: FT_UInt,
    vertResolution: FT_UInt,
};

type FT_Size_RequestRec = struct_FT_Size_RequestRec_;
type FT_Size_Request = *struct_FT_Size_RequestRec_;

type enum_FT_Render_Mode_ = c_uint;
const FT_RENDER_MODE_NORMAL: u32 = 0_u32;
const FT_RENDER_MODE_LIGHT: u32 = 1_u32;
const FT_RENDER_MODE_MONO: u32 = 2_u32;
const FT_RENDER_MODE_LCD: u32 = 3_u32;
const FT_RENDER_MODE_LCD_V: u32 = 4_u32;
const FT_RENDER_MODE_MAX: u32 = 5_u32;

type FT_Render_Mode = enum_FT_Render_Mode_;

type enum_FT_Kerning_Mode_ = c_uint;
const FT_KERNING_DEFAULT: u32 = 0_u32;
const FT_KERNING_UNFITTED: u32 = 1_u32;
const FT_KERNING_UNSCALED: u32 = 2_u32;

type FT_Kerning_Mode = enum_FT_Kerning_Mode_;

type enum_unnamed1 = c_uint;
const FT_Mod_Err_Base: u32 = 0_u32;
const FT_Mod_Err_Autofit: u32 = 0_u32;
const FT_Mod_Err_BDF: u32 = 0_u32;
const FT_Mod_Err_Cache: u32 = 0_u32;
const FT_Mod_Err_CFF: u32 = 0_u32;
const FT_Mod_Err_CID: u32 = 0_u32;
const FT_Mod_Err_Gzip: u32 = 0_u32;
const FT_Mod_Err_LZW: u32 = 0_u32;
const FT_Mod_Err_OTvalid: u32 = 0_u32;
const FT_Mod_Err_PCF: u32 = 0_u32;
const FT_Mod_Err_PFR: u32 = 0_u32;
const FT_Mod_Err_PSaux: u32 = 0_u32;
const FT_Mod_Err_PShinter: u32 = 0_u32;
const FT_Mod_Err_PSnames: u32 = 0_u32;
const FT_Mod_Err_Raster: u32 = 0_u32;
const FT_Mod_Err_SFNT: u32 = 0_u32;
const FT_Mod_Err_Smooth: u32 = 0_u32;
const FT_Mod_Err_TrueType: u32 = 0_u32;
const FT_Mod_Err_Type1: u32 = 0_u32;
const FT_Mod_Err_Type42: u32 = 0_u32;
const FT_Mod_Err_Winfonts: u32 = 0_u32;
const FT_Mod_Err_Max: u32 = 1_u32;

type enum_unnamed2 = c_uint;
const FT_Err_Ok: u32 = 0_u32;
const FT_Err_Cannot_Open_Resource: u32 = 1_u32;
const FT_Err_Unknown_File_Format: u32 = 2_u32;
const FT_Err_Invalid_File_Format: u32 = 3_u32;
const FT_Err_Invalid_Version: u32 = 4_u32;
const FT_Err_Lower_Module_Version: u32 = 5_u32;
const FT_Err_Invalid_Argument: u32 = 6_u32;
const FT_Err_Unimplemented_Feature: u32 = 7_u32;
const FT_Err_Invalid_Table: u32 = 8_u32;
const FT_Err_Invalid_Offset: u32 = 9_u32;
const FT_Err_Array_Too_Large: u32 = 10_u32;
const FT_Err_Invalid_Glyph_Index: u32 = 16_u32;
const FT_Err_Invalid_Character_Code: u32 = 17_u32;
const FT_Err_Invalid_Glyph_Format: u32 = 18_u32;
const FT_Err_Cannot_Render_Glyph: u32 = 19_u32;
const FT_Err_Invalid_Outline: u32 = 20_u32;
const FT_Err_Invalid_Composite: u32 = 21_u32;
const FT_Err_Too_Many_Hints: u32 = 22_u32;
const FT_Err_Invalid_Pixel_Size: u32 = 23_u32;
const FT_Err_Invalid_Handle: u32 = 32_u32;
const FT_Err_Invalid_Library_Handle: u32 = 33_u32;
const FT_Err_Invalid_Driver_Handle: u32 = 34_u32;
const FT_Err_Invalid_Face_Handle: u32 = 35_u32;
const FT_Err_Invalid_Size_Handle: u32 = 36_u32;
const FT_Err_Invalid_Slot_Handle: u32 = 37_u32;
const FT_Err_Invalid_CharMap_Handle: u32 = 38_u32;
const FT_Err_Invalid_Cache_Handle: u32 = 39_u32;
const FT_Err_Invalid_Stream_Handle: u32 = 40_u32;
const FT_Err_Too_Many_Drivers: u32 = 48_u32;
const FT_Err_Too_Many_Extensions: u32 = 49_u32;
const FT_Err_Out_Of_Memory: u32 = 64_u32;
const FT_Err_Unlisted_Object: u32 = 65_u32;
const FT_Err_Cannot_Open_Stream: u32 = 81_u32;
const FT_Err_Invalid_Stream_Seek: u32 = 82_u32;
const FT_Err_Invalid_Stream_Skip: u32 = 83_u32;
const FT_Err_Invalid_Stream_Read: u32 = 84_u32;
const FT_Err_Invalid_Stream_Operation: u32 = 85_u32;
const FT_Err_Invalid_Frame_Operation: u32 = 86_u32;
const FT_Err_Nested_Frame_Access: u32 = 87_u32;
const FT_Err_Invalid_Frame_Read: u32 = 88_u32;
const FT_Err_Raster_Uninitialized: u32 = 96_u32;
const FT_Err_Raster_Corrupted: u32 = 97_u32;
const FT_Err_Raster_Overflow: u32 = 98_u32;
const FT_Err_Raster_Negative_Height: u32 = 99_u32;
const FT_Err_Too_Many_Caches: u32 = 112_u32;
const FT_Err_Invalid_Opcode: u32 = 128_u32;
const FT_Err_Too_Few_Arguments: u32 = 129_u32;
const FT_Err_Stack_Overflow: u32 = 130_u32;
const FT_Err_Code_Overflow: u32 = 131_u32;
const FT_Err_Bad_Argument: u32 = 132_u32;
const FT_Err_Divide_By_Zero: u32 = 133_u32;
const FT_Err_Invalid_Reference: u32 = 134_u32;
const FT_Err_Debug_OpCode: u32 = 135_u32;
const FT_Err_ENDF_In_Exec_Stream: u32 = 136_u32;
const FT_Err_Nested_DEFS: u32 = 137_u32;
const FT_Err_Invalid_CodeRange: u32 = 138_u32;
const FT_Err_Execution_Too_Long: u32 = 139_u32;
const FT_Err_Too_Many_Function_Defs: u32 = 140_u32;
const FT_Err_Too_Many_Instruction_Defs: u32 = 141_u32;
const FT_Err_Table_Missing: u32 = 142_u32;
const FT_Err_Horiz_Header_Missing: u32 = 143_u32;
const FT_Err_Locations_Missing: u32 = 144_u32;
const FT_Err_Name_Table_Missing: u32 = 145_u32;
const FT_Err_CMap_Table_Missing: u32 = 146_u32;
const FT_Err_Hmtx_Table_Missing: u32 = 147_u32;
const FT_Err_Post_Table_Missing: u32 = 148_u32;
const FT_Err_Invalid_Horiz_Metrics: u32 = 149_u32;
const FT_Err_Invalid_CharMap_Format: u32 = 150_u32;
const FT_Err_Invalid_PPem: u32 = 151_u32;
const FT_Err_Invalid_Vert_Metrics: u32 = 152_u32;
const FT_Err_Could_Not_Find_Context: u32 = 153_u32;
const FT_Err_Invalid_Post_Table_Format: u32 = 154_u32;
const FT_Err_Invalid_Post_Table: u32 = 155_u32;
const FT_Err_Syntax_Error: u32 = 160_u32;
const FT_Err_Stack_Underflow: u32 = 161_u32;
const FT_Err_Ignore: u32 = 162_u32;
const FT_Err_No_Unicode_Glyph_Name: u32 = 163_u32;
const FT_Err_Missing_Startfont_Field: u32 = 176_u32;
const FT_Err_Missing_Font_Field: u32 = 177_u32;
const FT_Err_Missing_Size_Field: u32 = 178_u32;
const FT_Err_Missing_Fontboundingbox_Field: u32 = 179_u32;
const FT_Err_Missing_Chars_Field: u32 = 180_u32;
const FT_Err_Missing_Startchar_Field: u32 = 181_u32;
const FT_Err_Missing_Encoding_Field: u32 = 182_u32;
const FT_Err_Missing_Bbx_Field: u32 = 183_u32;
const FT_Err_Bbx_Too_Big: u32 = 184_u32;
const FT_Err_Corrupted_Font_Header: u32 = 185_u32;
const FT_Err_Corrupted_Font_Glyphs: u32 = 186_u32;
const FT_Err_Max: u32 = 187_u32;

#[link_name="freetype"]
extern mod bindgen {

fn FT_Init_FreeType(++alibrary: *FT_Library) -> FT_Error;

fn FT_Done_FreeType(++library: FT_Library) -> FT_Error;

fn FT_New_Face(++library: FT_Library, ++filepathname: *c_char, ++face_index: FT_Long, ++aface: *FT_Face) -> FT_Error;

fn FT_New_Memory_Face(++library: FT_Library, ++file_base: *FT_Byte, ++file_size: FT_Long, ++face_index: FT_Long, ++aface: *FT_Face) -> FT_Error;

fn FT_Open_Face(++library: FT_Library, ++args: *FT_Open_Args, ++face_index: FT_Long, ++aface: *FT_Face) -> FT_Error;

fn FT_Attach_File(++face: FT_Face, ++filepathname: *c_char) -> FT_Error;

fn FT_Attach_Stream(++face: FT_Face, ++parameters: *FT_Open_Args) -> FT_Error;

fn FT_Reference_Face(++face: FT_Face) -> FT_Error;

fn FT_Done_Face(++face: FT_Face) -> FT_Error;

fn FT_Select_Size(++face: FT_Face, ++strike_index: FT_Int) -> FT_Error;

fn FT_Request_Size(++face: FT_Face, ++req: FT_Size_Request) -> FT_Error;

fn FT_Set_Char_Size(++face: FT_Face, ++char_width: FT_F26Dot6, ++char_height: FT_F26Dot6, ++horz_resolution: FT_UInt, ++vert_resolution: FT_UInt) -> FT_Error;

fn FT_Set_Pixel_Sizes(++face: FT_Face, ++pixel_width: FT_UInt, ++pixel_height: FT_UInt) -> FT_Error;

fn FT_Load_Glyph(++face: FT_Face, ++glyph_index: FT_UInt, ++load_flags: FT_Int32) -> FT_Error;

fn FT_Load_Char(++face: FT_Face, ++char_code: FT_ULong, ++load_flags: FT_Int32) -> FT_Error;

fn FT_Set_Transform(++face: FT_Face, ++matrix: *FT_Matrix, ++delta: *FT_Vector);

fn FT_Render_Glyph(++slot: FT_GlyphSlot, ++render_mode: FT_Render_Mode) -> FT_Error;

fn FT_Get_Kerning(++face: FT_Face, ++left_glyph: FT_UInt, ++right_glyph: FT_UInt, ++kern_mode: FT_UInt, ++akerning: *FT_Vector) -> FT_Error;

fn FT_Get_Track_Kerning(++face: FT_Face, ++point_size: FT_Fixed, ++degree: FT_Int, ++akerning: *FT_Fixed) -> FT_Error;

fn FT_Get_Glyph_Name(++face: FT_Face, ++glyph_index: FT_UInt, ++buffer: FT_Pointer, ++buffer_max: FT_UInt) -> FT_Error;

fn FT_Get_Postscript_Name(++face: FT_Face) -> *c_char;

fn FT_Select_Charmap(++face: FT_Face, ++encoding: FT_Encoding) -> FT_Error;

fn FT_Set_Charmap(++face: FT_Face, ++charmap: FT_CharMap) -> FT_Error;

fn FT_Get_Charmap_Index(++charmap: FT_CharMap) -> FT_Int;

fn FT_Get_Char_Index(++face: FT_Face, ++charcode: FT_ULong) -> FT_UInt;

fn FT_Get_First_Char(++face: FT_Face, ++agindex: *FT_UInt) -> FT_ULong;

fn FT_Get_Next_Char(++face: FT_Face, ++char_code: FT_ULong, ++agindex: *FT_UInt) -> FT_ULong;

fn FT_Get_Name_Index(++face: FT_Face, ++glyph_name: *FT_String) -> FT_UInt;

fn FT_Get_SubGlyph_Info(++glyph: FT_GlyphSlot, ++sub_index: FT_UInt, ++p_index: *FT_Int, ++p_flags: *FT_UInt, ++p_arg1: *FT_Int, ++p_arg2: *FT_Int, ++p_transform: *FT_Matrix) -> FT_Error;

fn FT_Get_FSType_Flags(++face: FT_Face) -> FT_UShort;

fn FT_Face_GetCharVariantIndex(++face: FT_Face, ++charcode: FT_ULong, ++variantSelector: FT_ULong) -> FT_UInt;

fn FT_Face_GetCharVariantIsDefault(++face: FT_Face, ++charcode: FT_ULong, ++variantSelector: FT_ULong) -> FT_Int;

fn FT_Face_GetVariantSelectors(++face: FT_Face) -> *FT_UInt32;

fn FT_Face_GetVariantsOfChar(++face: FT_Face, ++charcode: FT_ULong) -> *FT_UInt32;

fn FT_Face_GetCharsOfVariant(++face: FT_Face, ++variantSelector: FT_ULong) -> *FT_UInt32;

fn FT_MulDiv(++a: FT_Long, ++b: FT_Long, ++c: FT_Long) -> FT_Long;

fn FT_MulFix(++a: FT_Long, ++b: FT_Long) -> FT_Long;

fn FT_DivFix(++a: FT_Long, ++b: FT_Long) -> FT_Long;

fn FT_RoundFix(++a: FT_Fixed) -> FT_Fixed;

fn FT_CeilFix(++a: FT_Fixed) -> FT_Fixed;

fn FT_FloorFix(++a: FT_Fixed) -> FT_Fixed;

fn FT_Vector_Transform(++vec: *FT_Vector, ++matrix: *FT_Matrix);

fn FT_Library_Version(++library: FT_Library, ++amajor: *FT_Int, ++aminor: *FT_Int, ++apatch: *FT_Int);

fn FT_Face_CheckTrueTypePatents(++face: FT_Face) -> FT_Bool;

fn FT_Face_SetUnpatentedHinting(++face: FT_Face, ++value: FT_Bool) -> FT_Bool;

}
