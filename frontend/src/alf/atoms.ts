import { StyleProp, ViewStyle, StyleSheet } from 'react-native';

import { borderRadius, fontSize, fontWeight, space, TRACKING } from './tokens';
import { ios, platform, web } from './util/platform';

export const atoms = {
  /*
   * Positioning Modes
   */
  fixed: {
    position: platform({ web: 'fixed', native: 'absolute' }) as 'absolute',
  },
  absolute: { position: 'absolute' },
  relative: { position: 'relative' },
  static: { position: 'static' },
  sticky: web({ position: 'sticky' }),

  /*
   * Position Settings
   */
  inset_0: { top: 0, bottom: 0, left: 0, right: 0 },

  top_0: { top: 0 },
  bottom_0: { bottom: 0 },
  left_0: { left: 0 },
  right_0: { right: 0 },

  z_10: { zIndex: 10 },
  z_20: { zIndex: 20 },
  z_30: { zIndex: 30 },
  z_40: { zIndex: 40 },
  z_50: { zIndex: 50 },

  overflow_visible: { overflow: 'visible' },
  overflow_hidden: { overflow: 'hidden' },
  overflow_auto: web({ overflow: 'auto' }),

  /*
   * Content Size: Width and Height
   */
  w_full: { width: '100%' },
  h_full: { height: '100%' },
  h_full_vh: web({ height: '100vh' }),
  max_w_full: { maxWidth: '100%', maxHeight: '100%' },

  util_screen_outer: platform({
    web: { minHeight: '100vh' },
    native: { height: '100%' },
  }) as StyleProp<ViewStyle>,

  bg_transparent: { backgroundColor: 'transparent' },

  /*
   * Border Radius
   */
  rounded_0: { borderRadius: 0 },
  rounded_2xs: { borderRadius: borderRadius['2xs'] },
  rounded_xs: { borderRadius: borderRadius.xs },
  rounded_sm: { borderRadius: borderRadius.sm },
  rounded_md: { borderRadius: borderRadius.md },
  rounded_lg: { borderRadius: borderRadius.lg },
  rounded_full: { borderRadius: borderRadius.full },

  /*
   * Flex Layout
   */
  flex: { display: 'flex' },

  flex_col: { flexDirection: 'column' },
  flex_row: { flexDirection: 'row' },
  flex_col_reverse: { flexDirection: 'column-reverse' },
  flex_row_reverse: { flexDirection: 'row-reverse' },

  flex_wrap: { flexWrap: 'wrap' },
  flex_nowrap: { flexWarp: 'nowrap' },

  flex_0: { flex: platform<string | number>({ web: '0 0 auto', native: 0 }) },
  flex_1: { flex: 1 },

  flex_grow: { flexGrow: 1 },
  flex_grow_0: { flexGrow: 0 },
  flex_shrink: { flexShrink: 1 },
  flex_shrink_0: { flexShrink: 0 },

  // Alignment of items along main axis
  justify_start: { justifyContent: 'flex-start' },
  justify_center: { justifyContent: 'center' },
  justify_between: { justifyContent: 'space-between' },
  justify_end: { justifyContent: 'flex-end' },

  // Alignment of items along cross axis (perpendicular to main axis)
  align_start: { alignItems: 'flex-start' },
  align_center: { alignItems: 'center' },
  align_baseline: { alignItems: 'baseline' },
  align_stretch: { alignItems: 'stretch' },
  align_end: { alignItems: 'flex-end' },

  // Alignment of a specified item along cross axis (perpendicular to main axis)
  self_auto: { alignSelf: 'auto' },
  self_start: { alignSelf: 'flex-start' },
  self_center: { alignSelf: 'center' },
  self_baseline: { alignSelf: 'baseline' },
  self_stretch: { alignSelf: 'stretch' },
  self_end: { alignSelf: 'flex-end' },

  // Gap in-between flex items
  gap_0: { gap: 0 },
  gap_2xs: { gap: space['2xs'] },
  gap_xs: { gap: space.xs },
  gap_sm: { gap: space.sm },
  gap_md: { gap: space.md },
  gap_lg: { gap: space.lg },
  gap_xl: { gap: space.xl },
  gap_2xl: { gap: space['2xl'] },
  gap_3xl: { gap: space['3xl'] },
  gap_4xl: { gap: space['4xl'] },
  gap_5xl: { gap: space['5xl'] },

  /*
   * Text
   */
  text_left: { textAlign: 'left' },
  text_center: { textAlign: 'center' },
  text_right: { textAlign: 'right' },

  text_2xs: { fontSize: fontSize['2xs'], letterSpacing: TRACKING },
  text_xs: { fontSize: fontSize.xs, letterSpacing: TRACKING },
  text_sm: { fontSize: fontSize.sm, letterSpacing: TRACKING },
  text_md: { fontSize: fontSize.md, letterSpacing: TRACKING },
  text_lg: { fontSize: fontSize.lg, letterSpacing: TRACKING },
  text_xl: { fontSize: fontSize.xl, letterSpacing: TRACKING },
  text_2xl: { fontSize: fontSize['2xl'], letterSpacing: TRACKING },
  text_3xl: { fontSize: fontSize['3xl'], letterSpacing: TRACKING },
  text_4xl: { fontSize: fontSize['4xl'], letterSpacing: TRACKING },
  text_5xl: { fontSize: fontSize['5xl'], letterSpacing: TRACKING },

  leading_tight: { lineHeight: 1.15 },
  leading_snug: { lineHeight: 1.3 },
  leading_normal: { lineHeight: 1.5 },

  tracking_normal: { letterSpacing: TRACKING },

  font_normal: { fontWeight: fontWeight.normal },
  font_bold: { fontWeight: fontWeight.bold },
  font_heavy: { fontWeight: fontWeight.heavy },
  font_italics: { fontStyle: 'italic' },

  /*
   * Border
   */
  border_0: { borderWidth: 0 },
  border_t_0: { borderTopWidth: 0 },
  border_b_0: { borderBottomWidth: 0 },
  border_l_0: { borderLeftWidth: 0 },
  border_r_0: { borderRightWidth: 0 },

  border: { borderWidth: StyleSheet.hairlineWidth },
  border_t: { borderTopWidth: StyleSheet.hairlineWidth },
  border_b: { borderBottomWidth: StyleSheet.hairlineWidth },
  border_l: { borderLeftWidth: StyleSheet.hairlineWidth },
  border_r: { borderRightWidth: StyleSheet.hairlineWidth },

  border_transparent: { borderColor: 'transparent' },

  curve_circular: ios({ borderCurve: 'circular' }),
  curve_continuous: ios({ borderCurve: 'continuous' }),

  /*
   * Shadow
   */
  shadow_sm: { shadowRadius: 8, shadowOpacity: 0.1, elevation: 8 },
  shadow_md: { shadowRadius: 16, shadowOpacity: 0.1, elevation: 16 },
  shadow_lg: { shadowRadius: 32, shadowOpacity: 0.1, elevation: 24 },

  /*
   * Padding
   */
  p_0: { padding: 0 },
  p_2xs: { padding: space['2xs'] },
  p_xs: { padding: space.xs },
  p_sm: { padding: space.sm },
  p_md: { padding: space.md },
  p_lg: { padding: space.lg },
  p_xl: { padding: space.xl },
  p_2xl: { padding: space['2xl'] },
  p_3xl: { padding: space['3xl'] },
  p_4xl: { padding: space['4xl'] },
  p_5xl: { padding: space['5xl'] },

  px_0: { paddingLeft: 0, paddingRight: 0 },
  px_2xs: { paddingLeft: space['2xs'], paddingRight: space['2xs'] },
  px_xs: { paddingLeft: space.xs, paddingRight: space.xs },
  px_sm: { paddingLeft: space.sm, paddingRight: space.sm },
  px_md: { paddingLeft: space.md, paddingRight: space.md },
  px_lg: { paddingLeft: space.lg, paddingRight: space.lg },
  px_xl: { paddingLeft: space.xl, paddingRight: space.xl },
  px_2xl: { paddingLeft: space['2xl'], paddingRight: space['2xl'] },
  px_3xl: { paddingLeft: space['3xl'], paddingRight: space['3xl'] },
  px_4xl: { paddingLeft: space['4xl'], paddingRight: space['4xl'] },
  px_5xl: { paddingLeft: space['5xl'], paddingRight: space['5xl'] },

  py_0: { paddingTop: 0, paddingBottom: 0 },
  py_2xs: { paddingTop: space['2xs'], paddingBottom: space['2xs'] },
  py_xs: { paddingTop: space.xs, paddingBottom: space.xs },
  py_sm: { paddingTop: space.sm, paddingBottom: space.sm },
  py_md: { paddingTop: space.md, paddingBottom: space.md },
  py_lg: { paddingTop: space.lg, paddingBottom: space.lg },
  py_xl: { paddingTop: space.xl, paddingBottom: space.xl },
  py_2xl: { paddingTop: space['2xl'], paddingBottom: space['2xl'] },
  py_3xl: { paddingTop: space['3xl'], paddingBottom: space['3xl'] },
  py_4xl: { paddingTop: space['4xl'], paddingBottom: space['4xl'] },
  py_5xl: { paddingTop: space['5xl'], paddingBottom: space['5xl'] },

  pt_0: { paddingTop: 0 },
  pt_2xs: { paddingTop: space['2xs'] },
  pt_xs: { paddingTop: space.xs },
  pt_sm: { paddingTop: space.sm },
  pt_md: { paddingTop: space.md },
  pt_lg: { paddingTop: space.lg },
  pt_xl: { paddingTop: space.xl },
  pt_2xl: { paddingTop: space['2xl'] },
  pt_3xl: { paddingTop: space['3xl'] },
  pt_4xl: { paddingTop: space['4xl'] },
  pt_5xl: { paddingTop: space['5xl'] },

  pb_0: { paddingBottom: 0 },
  pb_2xs: { paddingBottom: space['2xs'] },
  pb_xs: { paddingBottom: space.xs },
  pb_sm: { paddingBottom: space.sm },
  pb_md: { paddingBottom: space.md },
  pb_lg: { paddingBottom: space.lg },
  pb_xl: { paddingBottom: space.xl },
  pb_2xl: { paddingBottom: space['2xl'] },
  pb_3xl: { paddingBottom: space['3xl'] },
  pb_4xl: { paddingBottom: space['4xl'] },
  pb_5xl: { paddingBottom: space['5xl'] },

  pl_0: { paddingLeft: 0 },
  pl_2xs: { paddingLeft: space['2xs'] },
  pl_xs: { paddingLeft: space.xs },
  pl_sm: { paddingLeft: space.sm },
  pl_md: { paddingLeft: space.md },
  pl_lg: { paddingLeft: space.lg },
  pl_xl: { paddingLeft: space.xl },
  pl_2xl: { paddingLeft: space['2xl'] },
  pl_3xl: { paddingLeft: space['3xl'] },
  pl_4xl: { paddingLeft: space['4xl'] },
  pl_5xl: { paddingLeft: space['5xl'] },

  pr_0: { paddingRight: 0 },
  pr_2xs: { paddingRight: space['2xs'] },
  pr_xs: { paddingRight: space.xs },
  pr_sm: { paddingRight: space.sm },
  pr_md: { paddingRight: space.md },
  pr_lg: { paddingRight: space.lg },
  pr_xl: { paddingRight: space.xl },
  pr_2xl: { paddingRight: space['2xl'] },
  pr_3xl: { paddingRight: space['3xl'] },
  pr_4xl: { paddingRight: space['4xl'] },
  pr_5xl: { paddingRight: space['5xl'] },

  /*
   * Margin
   */
  m_0: { margin: 0 },
  m_2xs: { margin: space['2xs'] },
  m_xs: { margin: space.xs },
  m_sm: { margin: space.sm },
  m_md: { margin: space.md },
  m_lg: { margin: space.lg },
  m_xl: { margin: space.xl },
  m_2xl: { margin: space['2xl'] },
  m_3xl: { margin: space['3xl'] },
  m_4xl: { margin: space['4xl'] },
  m_5xl: { margin: space['5xl'] },
  m_auto: { margin: 'auto' },

  mx_0: { marginLeft: 0, marginRight: 0 },
  mx_2xs: { marginLeft: space['2xs'], marginRight: space['2xs'] },
  mx_xs: { marginLeft: space.xs, marginRight: space.xs },
  mx_sm: { marginLeft: space.sm, marginRight: space.sm },
  mx_md: { marginLeft: space.md, marginRight: space.md },
  mx_lg: { marginLeft: space.lg, marginRight: space.lg },
  mx_xl: { marginLeft: space.xl, marginRight: space.xl },
  mx_2xl: { marginLeft: space['2xl'], marginRight: space['2xl'] },
  mx_3xl: { marginLeft: space['3xl'], marginRight: space['3xl'] },
  mx_4xl: { marginLeft: space['4xl'], marginRight: space['4xl'] },
  mx_5xl: { marginLeft: space['5xl'], marginRight: space['5xl'] },
  mx_auto: { marginLeft: 'auto', marginRight: 'auto' },

  my_0: { marginTop: 0, marginBottom: 0 },
  my_2xs: { marginTop: space['2xs'], marginBottom: space['2xs'] },
  my_xs: { marginTop: space.xs, marginBottom: space.xs },
  my_sm: { marginTop: space.sm, marginBottom: space.sm },
  my_md: { marginTop: space.md, marginBottom: space.md },
  my_lg: { marginTop: space.lg, marginBottom: space.lg },
  my_xl: { marginTop: space.xl, marginBottom: space.xl },
  my_2xl: { marginTop: space['2xl'], marginBottom: space['2xl'] },
  my_3xl: { marginTop: space['3xl'], marginBottom: space['3xl'] },
  my_4xl: { marginTop: space['4xl'], marginBottom: space['4xl'] },
  my_5xl: { marginTop: space['5xl'], marginBottom: space['5xl'] },
  my_auto: { marginTop: 'auto', marginBottom: 'auto' },

  mt_0: { marginTop: 0 },
  mt_2xs: { marginTop: space['2xs'] },
  mt_xs: { marginTop: space.xs },
  mt_sm: { marginTop: space.sm },
  mt_md: { marginTop: space.md },
  mt_lg: { marginTop: space.lg },
  mt_xl: { marginTop: space.xl },
  mt_2xl: { marginTop: space['2xl'] },
  mt_3xl: { marginTop: space['3xl'] },
  mt_4xl: { marginTop: space['4xl'] },
  mt_5xl: { marginTop: space['5xl'] },
  mt_auto: { marginTop: 'auto' },

  mb_0: { marginBottom: 0 },
  mb_2xs: { marginBottom: space['2xs'] },
  mb_xs: { marginBottom: space.xs },
  mb_sm: { marginBottom: space.sm },
  mb_md: { marginBottom: space.md },
  mb_lg: { marginBottom: space.lg },
  mb_xl: { marginBottom: space.xl },
  mb_2xl: { marginBottom: space['2xl'] },
  mb_3xl: { marginBottom: space['3xl'] },
  mb_4xl: { marginBottom: space['4xl'] },
  mb_5xl: { marginBottom: space['5xl'] },
  mb_auto: { marginBottom: 'auto' },

  ml_0: { marginLeft: 0 },
  ml_2xs: { marginLeft: space['2xs'] },
  ml_xs: { marginLeft: space.xs },
  ml_sm: { marginLeft: space.sm },
  ml_md: { marginLeft: space.md },
  ml_lg: { marginLeft: space.lg },
  ml_xl: { marginLeft: space.xl },
  ml_2xl: { marginLeft: space['2xl'] },
  ml_3xl: { marginLeft: space['3xl'] },
  ml_4xl: { marginLeft: space['4xl'] },
  ml_5xl: { marginLeft: space['5xl'] },
  ml_auto: { marginLeft: 'auto' },

  mr_0: { marginRight: 0 },
  mr_2xs: { marginRight: space['2xs'] },
  mr_xs: { marginRight: space.xs },
  mr_sm: { marginRight: space.sm },
  mr_md: { marginRight: space.md },
  mr_lg: { marginRight: space.lg },
  mr_xl: { marginRight: space.xl },
  mr_2xl: { marginRight: space['2xl'] },
  mr_3xl: { marginRight: space['3xl'] },
  mr_4xl: { marginRight: space['4xl'] },
  mr_5xl: { marginRight: space['5xl'] },
  mr_auto: { marginRight: 'auto' },

  /*
   * Pointer events and User select
   */
  pointer_events_none: { pointerEvents: 'none' },
  pointer_events_auto: { pointerEvents: 'auto' },

  user_select_none: { userSelect: 'none' },
  user_select_all: { userSelect: 'all' },

  outline_inset_1: { outlineOffset: -1 },

  /*
   * Text decoration
   */
  underline: { textDecorationLine: 'underline' },
  strike_through: { textDecorationLine: 'line-through' },

  /*
   * Display
   */
  hidden: { display: 'none' },
  inline: web({ display: 'inline' }),
  block: web({ display: 'block' }),

  /*
   * Transition
   */

  /*
   * Animaations
   */

  /*
   * Scrollbar offset
   */
  // scrollbar_offset: platform({
  //   web: {
  //     transform: [
  //       {
  //         translateX: Layout.SCROLLBAR_OFFSET,
  //       },
  //     ],
  //   },
  //   native: {
  //     transform: [],
  //   },
  // }) as {transform: Exclude<ViewStyle['transform'], string | undefined>},

  pointer: web({ cursor: 'pointer' }),
} as const;
