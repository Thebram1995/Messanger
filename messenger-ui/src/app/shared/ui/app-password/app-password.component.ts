import {
  ChangeDetectionStrategy,
  Component,
  forwardRef,
  Input
} from '@angular/core';

import {
  ControlValueAccessor,
  NG_VALUE_ACCESSOR
} from '@angular/forms';


@Component({
  selector: 'app-password',
  standalone: true,
  templateUrl: './app-password.component.html',
  styleUrl: './app-password.component.scss',
  providers: [
    {
      provide: NG_VALUE_ACCESSOR,
      useExisting: forwardRef(() => AppPasswordComponent),
      multi: true
    }
  ],
  changeDetection: ChangeDetectionStrategy.OnPush
})
// Permite integrar el componente con Reactive Forms,
// haciendo que funcione con formControlName igual que un input nativo.
export class AppPasswordComponent
  implements ControlValueAccessor {

  @Input() label = '';
  @Input() placeholder = '';
  @Input() autocomplete = '';
  @Input() hasError = false;

  value = '';
  disabled = false;
  showPassword = false;

  private onChange:
    (value: string) => void = () => {};

  private onTouched:
    () => void = () => {};

  writeValue(value: string): void {
    this.value = value ?? '';
  }

  registerOnChange(
    fn: (value: string) => void
  ): void {
    this.onChange = fn;
  }

  registerOnTouched(
    fn: () => void
  ): void {
    this.onTouched = fn;
  }

  setDisabledState(
    disabled: boolean
  ): void {
    this.disabled = disabled;
  }

  handleInput(
    event: Event
  ): void {

    const input =
      event.target as HTMLInputElement;

    this.value = input.value;
    this.onChange(this.value);
  }

  handleBlur(): void {
    this.onTouched();
  }

  togglePasswordVisibility(): void {
    this.showPassword =
      !this.showPassword;
  }
}