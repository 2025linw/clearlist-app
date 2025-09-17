import Svg, { Path } from 'react-native-svg';

import { type Props, useCommonSVGProps } from './common';

export function createSinglePathSVG({ path }: { path: string }) {
  return function LogoImpl(props: Props) {
    const { fill, size, style, ...rest } = useCommonSVGProps(props);

    return (
      <Svg
        fill="none"
        {...rest}
        viewBox="0 0 24 24"
        width={size}
        height={size}
        style={[style]}
      >
        <Path fill={fill} fillRule="evenodd" clipRule="evenodd" d={path} />
      </Svg>
    );
  };
}

export function createMultiPathSVG({ paths }: { paths: string[] }) {
  return function LogoImpl(props: Props) {
    const { fill, size, style, ...rest } = useCommonSVGProps(props);

    return (
      <Svg
        fill="none"
        viewBox="0 0 24 24"
        width={size}
        height={size}
        style={[style]}
      >
        {paths.map((path, i) => (
          <Path
            key={i}
            fill={fill}
            fillRule="evenodd"
            clipRule="evenodd"
            d={path}
          />
        ))}
      </Svg>
    );
  };
}
