import Button, { Props as BaseButtonProps } from '@/components/primitives/button';

type Props = BaseButtonProps & {};

export default function ListButton({ children, ...props }: Props) {
  return <Button {...props}>{children}</Button>;
}
