import {
  useCallback,
  createContext,
  useContext,
  useState,
  useMemo,
  type ComponentType,
  type ReactElement,
} from 'react';
import {
  View,
  Text,
  Pressable,
  type PressableProps,
  type GestureResponderEvent,
  type MouseEvent,
  type NativeSyntheticEvent,
  type TargetedEvent,
  type TextProps,
  type TextStyle,
  StyleSheet,
  type StyleProp,
  type ViewStyle,
} from 'react-native';

import { atoms as a, useTheme } from '#/alf';
import { normalizeLineHeight } from '#/alf/util/font';

import {
  type Props as SVGIconProps,
  sizes as iconSizes,
} from '#/components/icons/common';

export type ButtonColor = 'primary' | 'secondary' | 'theme' | 'transparent';
export type ButtonSize = 'tiny' | 'small' | 'large';
export type ButtonShape = 'default' | 'square' | 'round';
export type VariantProps = {
  color?: ButtonColor;
  size?: ButtonSize;
  shape?: ButtonShape;
};

export type ButtonState = {
  pressed: boolean;
  hovered: boolean;
  focused: boolean;
  disabled: boolean;
};

export type ButtonContext = VariantProps & ButtonState;

type NonTextElements =
  // Renderable element
  | ReactElement
  // Arr of renderable elements
  | Iterable<ReactElement | null | undefined | boolean>;

export type ButtonProps = Pick<
  PressableProps,
  | 'disabled'
  | 'onPress'
  | 'onLongPress'
  | 'onHoverIn'
  | 'onHoverOut'
  | 'onPressIn'
  | 'onPressOut'
  | 'onFocus'
  | 'onBlur'
> &
  VariantProps & {
    label: string;
    style?: StyleProp<ViewStyle>;
    hoverStyle?: StyleProp<ViewStyle>;
    children: NonTextElements | ((context: ButtonContext) => NonTextElements);
  };

export type ButtonTextProps = TextProps & VariantProps & { disabled?: boolean };

const Context = createContext<ButtonState & VariantProps>({
  pressed: false,
  hovered: false,
  focused: false,
  disabled: false,
});

export function useButtonContext() {
  return useContext(Context);
}

export function Button({
  children,
  color = 'primary',
  size = 'small',
  shape = 'default',
  label,
  disabled = false,
  style,
  hoverStyle: hoverStyleProp,
  onPressIn: onPressInOuter,
  onPressOut: onPressOutOuter,
  onHoverIn: onHoverInOuter,
  onHoverOut: onHoverOutOuter,
  onFocus: onFocusOuter,
  onBlur: onBlurOuter,
  ...rest
}: ButtonProps) {
  const t = useTheme();

  const [state, setState] = useState({
    pressed: false,
    hovered: false,
    focused: false,
  });

  const onPressIn = useCallback(
    (e: GestureResponderEvent) => {
      setState(s => ({ ...s, pressed: true }));

      onPressInOuter?.(e);
    },
    [setState, onPressInOuter],
  );
  const onPressOut = useCallback(
    (e: GestureResponderEvent) => {
      setState(s => ({ ...s, pressed: false }));

      onPressOutOuter?.(e);
    },
    [setState, onPressOutOuter],
  );

  const onHoverIn = useCallback(
    (e: MouseEvent) => {
      setState(s => ({ ...s, hovered: true }));

      onHoverInOuter?.(e);
    },
    [setState, onHoverInOuter],
  );
  const onHoverOut = useCallback(
    (e: MouseEvent) => {
      setState(s => ({ ...s, hovered: false }));

      onHoverOutOuter?.(e);
    },
    [setState, onHoverOutOuter],
  );

  const onFocus = useCallback(
    (e: NativeSyntheticEvent<TargetedEvent>) => {
      setState(s => ({ ...s, focused: true }));

      onFocusOuter?.(e);
    },
    [setState, onFocusOuter],
  );
  const onBlur = useCallback(
    (e: NativeSyntheticEvent<TargetedEvent>) => {
      setState(s => ({ ...s, focused: false }));

      onBlurOuter?.(e);
    },
    [setState, onBlurOuter],
  );

  const { baseStyles, hoverStyles } = useMemo(() => {
    const baseStyles: ViewStyle[] = [];
    const hoverStyles: ViewStyle[] = [];

    if (color === 'primary') {
      if (!disabled) {
        baseStyles.push({ backgroundColor: t.palette.primary_step3 });
        hoverStyles.push({ backgroundColor: t.palette.primary_step4 });
      } else {
        baseStyles.push({ backgroundColor: t.palette.primary_step2 });
      }
    } else if (color === 'secondary') {
      if (!disabled) {
        baseStyles.push({ backgroundColor: t.palette.gray_step3 });
        hoverStyles.push({ backgroundColor: t.palette.gray_step4 });
      } else {
        baseStyles.push({ backgroundColor: t.palette.gray_step2 });
      }
    } else if (color === 'theme') {
      if (!disabled) {
        baseStyles.push({ backgroundColor: t.palette.primary_step9 });
        hoverStyles.push({ backgroundColor: t.palette.primary_step10 });
      } else {
        baseStyles.push({ backgroundColor: t.palette.primary_step2 });
      }
    } else if (color === 'transparent') {
      baseStyles.push({ backgroundColor: 'transparent ' });
    }

    if (shape === 'default') {
      if (size === 'tiny') {
        baseStyles.push({
          paddingVertical: 6,
          paddingHorizontal: 8,
          borderRadius: 6,
          gap: 2,
        });
      } else if (size === 'small') {
        baseStyles.push({
          paddingVertical: 8,
          paddingHorizontal: 12,
          borderRadius: 8,
          gap: 3,
        });
      } else if (size === 'large') {
        baseStyles.push({
          paddingVertical: 14,
          paddingHorizontal: 24,
          borderRadius: 10,
          gap: 4,
        });
      }
    } else if (shape === 'round' || shape === 'square') {
      if (size === 'tiny') {
        baseStyles.push({ height: 25, width: 25 });
      } else if (size === 'small') {
        baseStyles.push({ height: 33, width: 33 });
      } else if (size === 'large') {
        baseStyles.push({ height: 45, width: 45 });
      }

      if (shape === 'round') {
        baseStyles.push(a.rounded_full);
      } else if (shape === 'square') {
        if (size == 'tiny') {
          baseStyles.push({ borderRadius: 6 });
        } else {
          baseStyles.push(a.rounded_sm);
        }
      }
    }

    return { baseStyles, hoverStyles };
  }, [t, color, size, shape, disabled]);

  const context = useMemo<ButtonContext>(
    () => ({ ...state, color, size, disabled: disabled || false }),
    [state, color, size, disabled],
  );

  const flattenedBaseStyles = StyleSheet.flatten([baseStyles, style]);

  return (
    <Pressable
      aria-label={label}
      role="button"
      disabled={disabled || false}
      style={[
        a.flex_row,
        a.align_center,
        a.justify_center,
        a.curve_continuous,
        flattenedBaseStyles,
        ...(state.hovered || state.pressed
          ? [hoverStyles, StyleSheet.flatten(hoverStyleProp)]
          : []),
      ]}
      onPressIn={onPressIn}
      onPressOut={onPressOut}
      onHoverIn={onHoverIn}
      onHoverOut={onHoverOut}
      onFocus={onFocus}
      onBlur={onBlur}
      {...rest}
    >
      <Context.Provider value={context}>
        {typeof children === 'function' ? children(context) : children}
      </Context.Provider>
    </Pressable>
  );
}

export function useSharedButtonTextStyles() {
  const t = useTheme();

  const { color, disabled, size } = useButtonContext();

  return useMemo(() => {
    const baseStyles: TextStyle[] = [];

    if (color === 'primary') {
      if (!disabled) {
        baseStyles.push({ color: t.palette.primary_step12 });
      } else {
        baseStyles.push({ color: t.palette.primary_step3 });
      }
    } else if (color === 'secondary') {
      if (!disabled) {
        baseStyles.push({ color: t.palette.gray_step12 });
      } else {
        baseStyles.push({ color: t.palette.gray_step3 });
      }
    } else if (color === 'theme') {
      if (!disabled) {
        baseStyles.push({ color: '#FFFFFF' });
      } else {
        baseStyles.push({ color: t.palette.primary_step3 });
      }
    } else if (color === 'transparent') {
      if (!disabled) {
        baseStyles.push({ color: t.palette.gray_step12 });
      } else {
        baseStyles.push({ color: t.palette.gray_step3 });
      }
    }

    if (size === 'tiny') {
      baseStyles.push(a.text_xs, a.leading_tight);
    } else if (size === 'small') {
      baseStyles.push(a.text_md, a.leading_tight);
    } else if (size === 'large') {
      baseStyles.push(a.text_md, a.leading_tight);
    }

    const baseStylesFlattened = StyleSheet.flatten(baseStyles);

    const normalizedLineHeight = normalizeLineHeight(baseStylesFlattened);
    if (normalizedLineHeight) {
      baseStylesFlattened.lineHeight = normalizedLineHeight;
    }

    return baseStylesFlattened;
  }, [color, size, disabled]);
}

export function ButtonText({ children, style }: ButtonTextProps) {
  const textStyles = useSharedButtonTextStyles();

  return (
    <Text style={[a.font_bold, a.text_center, textStyles, style]}>
      {children}
    </Text>
  );
}

export function ButtonIcon({
  icon: Comp,
  size,
}: {
  icon: ComponentType<SVGIconProps>;
  size?: SVGIconProps['size'];
}) {
  const { size: buttonSize } = useButtonContext();
  const textStyles = useSharedButtonTextStyles();
  const { iconSize, iconContainerSize } = useMemo(() => {
    const iconSizeShorthand =
      size ??
      (({ tiny: 'xs', small: 'sm', large: 'sm' }[buttonSize || 'small'] ||
        'sm') as Exclude<SVGIconProps['size'], undefined>);

    const iconSize = iconSizes[iconSizeShorthand];

    const iconContainerSize = { tiny: 13, small: 17, large: 17 }[
      buttonSize || 'small'
    ];

    return { iconSize, iconContainerSize };
  }, [buttonSize, size]);
  const iconPosition = { top: '50%', left: '50%' };

  return (
    <View
      style={[a.z_20, { width: iconContainerSize, height: iconContainerSize }]}
    >
      <View
        style={[
          a.absolute,
          {
            width: iconSize,
            height: iconSize,
            ...iconPosition,
            transform: [
              { translateX: (iconSize / 2) * -1 },
              { translateY: (iconSize / 2) * -1 },
            ],
          },
        ]}
      >
        <Comp
          width={iconSize}
          style={[a.pointer_events_none, { color: textStyles.color }]}
        />
      </View>
    </View>
  );
}
