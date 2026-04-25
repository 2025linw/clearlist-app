import { TextInput } from 'react-native';

import FormField, { Props as FormFieldProps } from '@/components/forms/form-field';

type Props = Omit<FormFieldProps, 'children'> & {};

export default function FormTextInput(props: Props) {
  return (
    <FormField
      label={props.label}
      style={props.style}
    >
      <TextInput />
    </FormField>
  );
}
