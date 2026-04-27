import Button, { Props as BaseButtonProps } from '@/components/primitives/button';

type Props = BaseButtonProps & {};

export default function ListButton({ text, ...props }: Props) {
  return (
    <Button
      text={text}
      {...props}
    />
  );
}
