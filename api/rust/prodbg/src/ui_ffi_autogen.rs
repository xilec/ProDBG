// !!! Autogenerated with codespawn (0.3.1) - do not modify. !!!
bitflags! {
    flags PDUIWindowFlags_: c_uint {
        const PDUIWINDOWFLAGS_NOTITLEBAR = 1 as c_uint,
        const PDUIWINDOWFLAGS_NORESIZE = 2 as c_uint,
        const PDUIWINDOWFLAGS_NOMOVE = 4 as c_uint,
        const PDUIWINDOWFLAGS_NOSCROLLBAR = 8 as c_uint,
        const PDUIWINDOWFLAGS_NOSCROLLWITHMOUSE = 16 as c_uint,
        const PDUIWINDOWFLAGS_NOCOLLAPSE = 32 as c_uint,
        const PDUIWINDOWFLAGS_ALWAYSAUTORESIZE = 64 as c_uint,
        const PDUIWINDOWFLAGS_SHOWBORDERS = 128 as c_uint,
        const PDUIWINDOWFLAGS_NOSAVEDSETTINGS  = 256 as c_uint,
        const PDUIWINDOWFLAGS_NOINPUTS  = 512 as c_uint,
        const PDUIWINDOWFLAGS_MENUBAR = 1024 as c_uint,
        const PDUIWINDOWFLAGS_HORIZONTALSCROLLBAR = 2048 as c_uint,
        const PDUIWINDOWFLAGS_NOFOCUSONAPPEARING = 4096 as c_uint,
        const PDUIWINDOWFLAGS_NOBRINGTOFRONTONFOCUS = 8192 as c_uint,
        const PDUIWINDOWFLAGS_ALWAYSVERTICALSCROLLBAR = 16384 as c_uint,
        const PDUIWINDOWFLAGS_ALWAYSHORIZONTALSCROLLBAR = 32768 as c_uint,
        const PDUIWINDOWFLAGS_ALWAYSUSEWINDOWPADDING = 65536 as c_uint,
    }
}

bitflags! {
    flags PDUIInputTextFlags_: c_uint {
        const PDUIINPUTTEXTFLAGS_CHARSDECIMAL = 1 as c_uint,
        const PDUIINPUTTEXTFLAGS_CHARSHEXADECIMAL = 2 as c_uint,
        const PDUIINPUTTEXTFLAGS_CHARSUPPERCASE = 4 as c_uint,
        const PDUIINPUTTEXTFLAGS_CHARSNOBLANK = 8 as c_uint,
        const PDUIINPUTTEXTFLAGS_AUTOSELECTALL = 16 as c_uint,
        const PDUIINPUTTEXTFLAGS_ENTERRETURNSTRUE = 32 as c_uint,
        const PDUIINPUTTEXTFLAGS_CALLBACKCOMPLETION = 64 as c_uint,
        const PDUIINPUTTEXTFLAGS_CALLBACKHISTORY = 128 as c_uint,
        const PDUIINPUTTEXTFLAGS_CALLBACKALWAYS  = 256 as c_uint,
        const PDUIINPUTTEXTFLAGS_CALLBACKCHARFILTER  = 512 as c_uint,
        const PDUIINPUTTEXTFLAGS_ALLOWTABINPUT = 1024 as c_uint,
        const PDUIINPUTTEXTFLAGS_CTRLENTERFORNEWLINE = 2048 as c_uint,
        const PDUIINPUTTEXTFLAGS_NOHORIZONTALSCROLL = 4096 as c_uint,
        const PDUIINPUTTEXTFLAGS_ALWAYSINSERTMODE = 8192 as c_uint,
        const PDUIINPUTTEXTFLAGS_READONLY = 16384 as c_uint,
        const PDUIINPUTTEXTFLAGS_PASSWORD = 32768 as c_uint,
    }
}


#[repr(C)]
#[derive(Debug)]
pub struct PDVec2 {
    pub x: c_float,
    pub y: c_float,
}


#[repr(C)]
pub struct PDVec4 {
    pub x: c_float,
    pub y: c_float,
    pub z: c_float,
    pub w: c_float,
}


#[repr(C)]
pub struct PDRect {
    pub x: c_float,
    pub y: c_float,
    pub width: c_float,
    pub height: c_float,
}


#[repr(C)]
pub struct PDUIInputTextCallbackData {
    pub event_flag: c_uint,
    pub flags: c_uint,
    pub user_data: *mut c_void,
    pub event_char: c_ushort,
    pub event_key: c_ushort,
    pub buf: *mut c_char,
    pub buf_size: usize,
    pub buf_dirty: c_int,
    pub cursor_pos: c_int,
    pub selection_start: c_int,
    pub selection_end: c_int,
    pub delete_chars: Option<extern "C" fn(data: *mut PDUIInputTextCallbackData, pos: c_int, byteCount: c_int)>,
    pub insert_chars: Option<extern "C" fn(data: *mut PDUIInputTextCallbackData, pos: c_int, text: *const c_char, textEnd: *const c_char)>,
}


#[repr(C)]
pub struct CPdUI {
    pub private_data: *mut c_void,
    pub set_title: extern fn(private_data: *mut c_void, title: *const c_char),
    pub get_window_size: extern fn() -> PDVec2,
    pub get_window_pos: extern fn() -> PDVec2,
    pub begin_child: extern fn(stringId: *const c_char, size: PDVec2, border: c_int, extraFlags: c_uint),
    pub end_child: extern fn(),
    pub get_scroll_y: extern fn() -> c_float,
    pub get_scroll_max_y: extern fn() -> c_float,
    pub set_scroll_y: extern fn(scrollY: c_float),
    pub set_scroll_here: extern fn(centerYratio: c_float),
    pub set_scroll_from_pos_y: extern fn(posY: c_float, centerYratio: c_float),
    pub set_keyboard_focus_here: extern fn(offset: c_int),
    pub push_font: extern fn(font: *mut c_void),
    pub pop_font: *mut extern fn() -> c_void,
    pub push_style_color: extern fn(idx: c_uint, col: c_uint),
    pub pop_style_color: extern fn(count: c_int),
    pub push_style_var: extern fn(idx: c_uint, val: c_float),
    pub push_style_var_vec: extern fn(idx: c_uint, val: PDVec2),
    pub pop_style_var: extern fn(count: c_int),
    pub get_font_size: extern fn() -> c_float,
    pub push_item_width: extern fn(item_width: c_float),
    pub pop_item_width: extern fn() -> c_void,
    pub calc_item_width: *mut extern fn() -> c_float,
    pub push_allow_keyboard_focus: extern fn(v: c_int),
    pub pop_allow_keyboard_focus: *mut extern fn() -> c_void,
    pub push_text_wrap_pos: extern fn(wrapPosX: c_float),
    pub pop_text_wrap_pos: *mut extern fn() -> c_void,
    pub push_button_repeat: extern fn(repeat: c_int),
    pub pop_button_repeat: *mut extern fn() -> c_void,
    pub begin_group: *mut extern fn() -> c_void,
    pub end_group: *mut extern fn() -> c_void,
    pub separator: extern fn() -> c_void,
    pub same_line: extern fn(columnX: c_int, spacingW: c_int),
    pub spacing: *mut extern fn() -> c_void,
    pub dummy: extern fn(size: PDVec2),
    pub indent: *mut extern fn() -> c_void,
    pub un_indent: *mut extern fn() -> c_void,
    pub columns: extern fn(count: c_int, id: *const c_char, border: c_int),
    pub next_column: extern fn(),
    pub get_column_index: extern fn() -> c_int,
    pub get_column_offset: extern fn(column_index: c_int) -> c_float,
    pub set_column_offset: extern fn(column_index: c_int, offset_x: c_float),
    pub get_column_width: extern fn(column_index: c_int) -> c_float,
    pub get_columns_count: extern fn() -> c_int,
    pub get_cursor_pos: extern fn() -> PDVec2,
    pub get_cursor_pos_x: extern fn() -> c_float,
    pub get_cursor_pos_y: extern fn() -> c_float,
    pub set_cursor_pos: extern fn(pos: PDVec2),
    pub set_cursor_pos_x: extern fn(x: c_float),
    pub set_cursor_pos_y: extern fn(y: c_float),
    pub get_cursor_screen_pos: extern fn() -> PDVec2,
    pub set_cursor_screen_pos: extern fn(pos: PDVec2),
    pub align_first_text_height_to_widgets: *mut extern fn() -> c_void,
    pub get_text_line_height: *mut extern fn() -> c_float,
    pub get_text_line_height_with_spacing: extern fn() -> c_float,
    pub get_items_line_height_with_spacing: *mut extern fn() -> c_float,
    pub push_id_str: extern fn(strId: *const c_char),
    pub push_id_str_range: extern fn(strBegin: *const c_char, strEnd: *const c_char),
    pub push_id_ptr: extern fn(ptrId: *const c_void),
    pub push_id_int: extern fn(intId: c_int),
    pub pop_id: extern fn() -> c_void,
    pub get_id_str: extern fn(strId: *const c_char) -> c_uint,
    pub get_id_str_range: extern fn(strBegin: *const c_char, strEnd: *const c_char) -> c_uint,
    pub get_id_ptr: extern fn(ptrId: *const c_void) -> c_uint,
    pub text: extern fn(fmt: *const c_char),
    pub text_format: extern fn(fmt: *const c_char),
    pub text_v: extern fn(fmt: *const c_char, args: c_int),
    pub text_colored: extern fn(col: c_uint, fmt: *const c_char),
    pub text_colored_v: extern fn(col: c_uint, fmt: *const c_char, args: c_int),
    pub text_disabled: extern fn(fmt: *const c_char),
    pub text_disabled_v: extern fn(fmt: *const c_char, args: c_int),
    pub text_wrapped: extern fn(fmt: *const c_char),
    pub text_wrapped_v: extern fn(fmt: *const c_char, args: c_int),
    pub text_unformatted: extern fn(text: *const c_char, text_end: *const c_char),
    pub label_text: extern fn(label: *const c_char, fmt: *const c_char),
    pub label_text_v: extern fn(label: *const c_char, fmt: *const c_char, args: c_int),
    pub bullet: *mut extern fn() -> c_void,
    pub bullet_text: extern fn(fmt: *const c_char),
    pub bullet_text_v: extern fn(fmt: *const c_char, args: c_int),
    pub button: extern fn(label: *const c_char, size: PDVec2) -> c_int,
    pub small_button: extern fn(label: *const c_char) -> c_int,
    pub invisible_button: extern fn(strId: *const c_char, size: PDVec2) -> c_int,
    pub image: extern fn(user_texture_id: *mut c_void, size: PDVec2, uv0: PDVec2, uv1: PDVec2, tintColor: c_uint, borderColor: c_uint),
    pub image_button: extern fn(user_texture_id: *mut c_void, size: PDVec2, uv0: PDVec2, uv1: PDVec2, framePadding: c_int, bgColor: c_uint, tintCol: c_uint) -> c_int,
    pub collapsing_header: extern fn(label: *const c_char, strId: *const c_char, displayFrame: c_int, defaultOpen: c_int) -> c_int,
    pub checkbox: extern fn(label: *const c_char, v: *mut c_int) -> c_int,
    pub checkbox_flags: extern fn(label: *const c_char, flags: *mut c_uint, flagsValue: c_uint) -> c_int,
    pub radio_button_bool: extern fn(label: *const c_char, active: c_int) -> c_int,
    pub radio_button: extern fn(label: *const c_char, v: *mut c_int, v_button: c_int) -> c_int,
    pub combo: extern fn(label: *const c_char, currentItem: *mut c_int, items: *mut *const c_char, itemsCount: c_int, heightInItems: c_int) -> c_int,
    pub combo2: extern fn(label: *const c_char, currentItem: *mut c_int, itemsSeparatedByZeros: *const c_char, heightInItems: c_int) -> c_int,
    pub combo3: extern fn(label: *const c_char, currentItem: *mut c_int, itemsGetter: extern fn(data: *mut c_void, idx: c_int, out_text: *mut *const c_char) -> c_int, data: *mut c_void, itemsCount: c_int, heightInItems: c_int) -> c_int,
    pub color_button: extern fn(col: c_uint, smallHeight: c_int, outlineBorder: c_int) -> c_int,
    pub color_edit3: extern fn(label: *const c_char, col3: *mut c_float) -> c_int,
    pub color_edit4: extern fn(label: *const c_char, col4: *mut c_float, showAlpha: c_int) -> c_int,
    pub color_edit_mode: extern fn(mode: c_uint),
    pub plot_lines: extern fn(label: *const c_char, values: *const c_float, valuesCount: c_int, valuesOffset: c_int, overlayText: *const c_char, scaleMin: c_float, scaleMax: c_float, graphSize: PDVec2, stride: usize),
    pub plot_lines2: extern fn(label: *const c_char, valuesGetter: extern fn(data: *mut c_void, idx: c_int) -> c_float, data: *mut c_void, valuesCount: c_int, valuesOffset: c_int, overlayText: *const c_char, scaleMin: c_float, scaleMax: c_float, graphSize: PDVec2),
    pub plot_histogram: extern fn(label: *const c_char, values: *const c_float, valuesCount: c_int, valuesOffset: c_int, overlayText: *const c_char, scaleMin: c_float, scaleMax: c_float, graphSize: PDVec2, stride: usize),
    pub plot_histogram2: extern fn(label: *const c_char, valuesGetter: extern fn(data: *mut c_void, idx: c_int) -> c_float, data: *mut c_void, valuesCount: c_int, valuesOffset: c_int, overlayText: *const c_char, scaleMin: c_float, scaleMax: c_float, graphSize: PDVec2),
    pub sc_input_text: extern fn(label: *const c_char, xSize: c_float, ySize: c_float) -> *mut PDUISCInterface,
    pub slider_float: extern fn(label: *const c_char, v: *mut c_float, vMin: c_float, vMax: c_float, displayFormat: *const c_char, power: c_float) -> c_int,
    pub slider_float2: extern fn(label: *const c_char, v2: *mut c_float, vMin: c_float, vMax: c_float, displayFormat: *const c_char, power: c_float) -> c_int,
    pub slider_float3: extern fn(label: *const c_char, v3: *mut c_float, vMin: c_float, vMax: c_float, displayFormat: *const c_char, power: c_float) -> c_int,
    pub slider_float4: extern fn(label: *const c_char, v4: *mut c_float, vMin: c_float, vMax: c_float, displayFormat: *const c_char, power: c_float) -> c_int,
    pub slider_angle: extern fn(label: *const c_char, v_rad: *mut c_float, vDegreesMin: c_float, vDegreesMax: c_float) -> c_int,
    pub slider_int: extern fn(label: *const c_char, v: *mut c_int, vMin: c_int, vMax: c_int, displayFormat: *const c_char) -> c_int,
    pub slider_int2: extern fn(label: *const c_char, v2: *mut c_int, vMin: c_int, vMax: c_int, displayFormat: *const c_char) -> c_int,
    pub slider_int3: extern fn(label: *const c_char, v3: *mut c_int, vMin: c_int, vMax: c_int, displayFormat: *const c_char) -> c_int,
    pub slider_int4: extern fn(label: *const c_char, v4: *mut c_int, vMin: c_int, vMax: c_int, displayFormat: *const c_char) -> c_int,
    pub vslider_float: extern fn(label: *const c_char, size: PDVec2, v: *mut c_float, vMin: c_float, vMax: c_float, displayFormat: *const c_char, power: c_float) -> c_int,
    pub vslider_int: extern fn(label: *const c_char, size: PDVec2, v: *mut c_int, vMin: c_int, vMax: c_int, displayFormat: *const c_char) -> c_int,
    pub drag_float: extern fn(label: *const c_char, v: *mut c_float, vSpeed: c_float, vMin: c_float, vMax: c_float, displayFormat: *const c_char, power: c_float) -> c_int,
    pub drag_float2: extern fn(label: *const c_char, v2: *mut c_float, vSpeed: c_float, vMin: c_float, vMax: c_float, displayFormat: *const c_char, power: c_float) -> c_int,
    pub drag_float3: extern fn(label: *const c_char, v3: *mut c_float, vSpeed: c_float, vMin: c_float, vMax: c_float, displayFormat: *const c_char, power: c_float) -> c_int,
    pub drag_float4: extern fn(label: *const c_char, v4: *mut c_float, vSpeed: c_float, vMin: c_float, vMax: c_float, displayFormat: *const c_char, power: c_float) -> c_int,
    pub drag_int: extern fn(label: *const c_char, v: *mut c_int, vSpeed: c_float, vMin: c_int, vMax: c_int, displayFormat: *const c_char) -> c_int,
    pub drag_int2: extern fn(label: *const c_char, v2: *mut c_int, vSpeed: c_float, vMin: c_int, vMax: c_int, displayFormat: *const c_char) -> c_int,
    pub drag_int3: extern fn(label: *const c_char, v3: *mut c_int, vSpeed: c_float, vMin: c_int, vMax: c_int, displayFormat: *const c_char) -> c_int,
    pub drag_int4: extern fn(label: *const c_char, v4: *mut c_int, vSpeed: c_float, vMin: c_int, vMax: c_int, displayFormat: *const c_char) -> c_int,
    pub input_text: extern fn(label: *const c_char, buf: *mut c_char, buf_size: c_int, flags: c_uint, callback: extern fn(*mut PDUIInputTextCallbackData), user_data: *mut c_void) -> c_int,
    pub input_text_multiline: extern fn(label: *const c_char, buf: *mut c_char, buf_size: usize, size: PDVec2, flags: c_uint, callback: extern fn(*mut PDUIInputTextCallbackData) -> c_void, user_data: *mut c_void) -> c_int,
    pub input_float: extern fn(label: *const c_char, v: *mut c_float, step: c_float, step_fast: c_float, decimal_precision: c_int, extraFlags: c_uint) -> c_int,
    pub input_float2: extern fn(label: *const c_char, v2: *mut c_float, decimal_precision: c_int, extraFlags: c_uint) -> c_int,
    pub input_float3: extern fn(label: *const c_char, v3: *mut c_float, decimal_precision: c_int, extraFlags: c_uint) -> c_int,
    pub input_float4: extern fn(label: *const c_char, v4: *mut c_float, decimal_precision: c_int, extraFlags: c_uint) -> c_int,
    pub input_int: extern fn(label: *const c_char, v: *mut c_int, step: c_int, step_fast: c_int, extraFlags: c_uint) -> c_int,
    pub input_int2: extern fn(label: *const c_char, v2: *mut c_int, extraFlags: c_uint) -> c_int,
    pub input_int3: extern fn(label: *const c_char, v3: *mut c_int, extraFlags: c_uint) -> c_int,
    pub input_int4: extern fn(label: *const c_char, v4: *mut c_int, extraFlags: c_uint) -> c_int,
    pub tree_node: extern fn(str_label_id: *const c_char) -> c_int,
    pub tree_node_str: extern fn(strId: *const c_char, fmt: *const c_char) -> c_int,
    pub tree_node_ptr: extern fn(ptrId: *const c_void, fmt: *const c_char) -> c_int,
    pub tree_node_str_v: extern fn(strId: *const c_char, fmt: *const c_char, args: c_int) -> c_int,
    pub tree_node_ptr_v: extern fn(ptrId: *const c_void, fmt: *const c_char, args: c_int) -> c_int,
    pub tree_push_str: extern fn(strId: *const c_char),
    pub tree_push_ptr: extern fn(ptrId: *const c_void),
    pub tree_pop: extern fn(),
    pub set_next_tree_node_opened: extern fn(opened: c_int, cond: c_uint),
    pub selectable: extern fn(label: *const c_char, selected: c_int, flags: c_uint, size: PDVec2) -> c_int,
    pub selectable_ex: extern fn(label: *const c_char, p_selected: *mut c_int, flags: c_uint, size: PDVec2) -> c_int,
    pub list_box: extern fn(label: *const c_char, currentItem: *mut c_int, items: *mut *const c_char, itemsCount: c_int, heightInItems: c_int) -> c_int,
    pub list_box2: extern fn(label: *const c_char, currentItem: *mut c_int, itemsGetter: extern fn(data: *mut c_void, idx: c_int, out_text: *mut *const c_char) -> c_int, data: *mut c_void, itemsCount: c_int, heightInItems: c_int) -> c_int,
    pub list_box_header: extern fn(label: *const c_char, size: PDVec2) -> c_int,
    pub list_box_header2: extern fn(label: *const c_char, itemsCount: c_int, heightInItems: c_int) -> c_int,
    pub list_box_footer: *mut extern fn() -> c_void,
    pub set_tooltip: extern fn(fmt: *const c_char),
    pub set_tooltip_v: extern fn(fmt: *const c_char, args: c_int),
    pub begin_tooltip: *mut extern fn() -> c_void,
    pub end_tooltip: *mut extern fn() -> c_void,
    pub begin_main_menu_bar: extern fn() -> c_int,
    pub end_main_menu_bar: extern fn() -> c_void,
    pub begin_menu_bar: *mut extern fn() -> c_int,
    pub end_menu_bar: *mut extern fn() -> c_void,
    pub begin_menu: extern fn(label: *const c_char, enabled: c_int) -> c_int,
    pub end_menu: extern fn(),
    pub menu_item: extern fn(label: *const c_char, shortcut: *const c_char, selected: c_int, enabled: c_int) -> c_int,
    pub menu_item_ptr: extern fn(label: *const c_char, shortcut: *const c_char, p_selected: *mut c_int, enabled: c_int) -> c_int,
    pub open_popup: extern fn(strId: *const c_char),
    pub begin_popup: extern fn(strId: *const c_char) -> c_int,
    pub begin_popup_modal: extern fn(name: *const c_char, p_opened: *mut c_int, extraFlags: c_uint) -> c_int,
    pub begin_popup_context_item: extern fn(strId: *const c_char, mouse_button: c_int) -> c_int,
    pub begin_popup_context_window: extern fn(also_over_items: c_int, strId: *const c_char, mouse_button: c_int) -> c_int,
    pub begin_popup_context_void: extern fn(strId: *const c_char, mouse_button: c_int) -> c_int,
    pub end_popup: extern fn(),
    pub close_current_popup: extern fn(),
    pub begin_popup_context: extern fn(priv_data: *mut c_void) -> c_int,
    pub end_popup_context: extern fn(priv_data: *mut c_void),
    pub value_bool: extern fn(prefix: *const c_char, b: c_int),
    pub value_int: extern fn(prefix: *const c_char, v: c_int),
    pub value_u_int: extern fn(prefix: *const c_char, v: c_uint),
    pub value_float: extern fn(prefix: *const c_char, v: c_float, float_format: *const c_char),
    pub color: extern fn(prefix: *const c_char, col: c_uint),
    pub log_to_tty: extern fn(maxDepth: c_int),
    pub log_to_file: extern fn(maxDepth: c_int, filename: *const c_char),
    pub log_to_clipboard: extern fn(maxDepth: c_int),
    pub log_finish: *mut extern fn() -> c_void,
    pub log_buttons: *mut extern fn() -> c_void,
    pub is_item_hovered: extern fn() -> c_int,
    pub is_item_hovered_rect: *mut extern fn() -> c_int,
    pub is_item_active: *mut extern fn() -> c_int,
    pub is_item_visible: *mut extern fn() -> c_int,
    pub is_any_item_hovered: *mut extern fn() -> c_int,
    pub is_any_item_active: *mut extern fn() -> c_int,
    pub get_item_rect_min: extern fn() -> PDVec2,
    pub get_item_rect_max: extern fn() -> PDVec2,
    pub get_item_rect_size: extern fn() -> PDVec2,
    pub is_window_hovered: *mut extern fn() -> c_int,
    pub is_window_focused: *mut extern fn() -> c_int,
    pub is_root_window_focused: *mut extern fn() -> c_int,
    pub is_root_window_or_any_child_focused: *mut extern fn() -> c_int,
    pub is_rect_visible: extern fn(itemSize: PDVec2) -> c_int,
    pub is_pos_hovering_any_window: extern fn(pos: PDVec2) -> c_int,
    pub get_time: *mut extern fn() -> c_float,
    pub get_frame_count: *mut extern fn() -> c_int,
    pub get_style_col_name: extern fn(idx: c_uint) -> *const c_char,
    pub calc_item_rect_closest_point: extern fn(pos: PDVec2, on_edge: c_int, outward: c_float) -> PDVec2,
    pub calc_text_size: extern fn(text: *const c_char, text_end: *const u8, hide_text_after_double_hash: c_int, wrap_width: c_float) -> PDVec2,
    pub calc_list_clipping: extern fn(items_count: c_int, items_height: c_float, out_items_display_start: *mut c_int, out_items_display_end: *mut c_int),
    pub begin_child_frame: extern fn(id: c_uint, size: PDVec2) -> c_int,
    pub end_child_frame: *mut extern fn() -> c_void,
    pub color_convert_rg_bto_hsv: extern fn(r: c_float, g: c_float, b: c_float, out_h: *mut c_float, out_s: *mut c_float, out_v: *mut c_float),
    pub color_convert_hs_vto_rgb: extern fn(h: c_float, s: c_float, v: c_float, out_r: *mut c_float, out_g: *mut c_float, out_b: *mut c_float),
    pub is_key_down: extern fn(key_index: c_int) -> c_int,
    pub is_key_pressed: extern fn(key_index: c_int, repeat: c_int) -> c_int,
    pub is_key_released: extern fn(key_index: c_int) -> c_int,
    pub is_key_down_id: extern fn(keyId: c_uint, repeat: c_int) -> c_int,
    pub is_mouse_down: extern fn(button: c_int) -> c_int,
    pub is_mouse_clicked: extern fn(button: c_int, repeat: c_int) -> c_int,
    pub is_mouse_double_clicked: extern fn(button: c_int) -> c_int,
    pub is_mouse_released: extern fn(button: c_int) -> c_int,
    pub is_mouse_hovering_window: *mut extern fn() -> c_int,
    pub is_mouse_hovering_any_window: *mut extern fn() -> c_int,
    pub is_mouse_hovering_rect: extern fn(rectMin: PDVec2, rectMax: PDVec2) -> c_int,
    pub is_mouse_dragging: extern fn(button: c_int, lockThreshold: c_float) -> c_int,
    pub get_mouse_pos: extern fn() -> PDVec2,
    pub get_mouse_drag_delta: extern fn(button: c_int, lockThreshold: c_float) -> PDVec2,
    pub reset_mouse_drag_delta: extern fn(button: c_int),
    pub get_mouse_cursor: *mut extern fn() -> c_uint,
    pub set_mouse_cursor: extern fn(ctype: c_uint),
    pub get_mouse_wheel: extern fn() -> c_float,
    pub fill_rect: extern fn(rect: PDRect, color: c_uint),
    pub fill_convex_poly: extern fn(verts: *const c_void, count: c_uint, color: c_uint, aa: c_int),
    pub fill_circle: extern fn(pos: PDVec2, radius: c_float, color: c_uint, num_seg: c_uint, aa: c_int),
    pub image_create_rgba: extern fn(width: c_uint, height: c_uint) -> *mut c_void,
    pub image_update: extern fn(dest: *mut c_void, src: *const c_void, size: c_uint) -> c_void,
}

// !!! End of autogenerated data. !!!
