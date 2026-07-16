import { CommonModule } from '@angular/common';
import {
  ChangeDetectionStrategy,
  Component,
  EventEmitter,
  Input,
  Output
} from '@angular/core';

export type AppButtonVariant =
  | 'primary'
  | 'secondary'
  | 'ghost'
  | 'danger';

export type AppButtonType =
  | 'button'
  | 'submit'
  | 'reset';

@Component({
  selector: 'app-button',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './app-button.component.html',
  styleUrl: './app-button.component.scss',
  changeDetection: ChangeDetectionStrategy.OnPush
})
export class AppButtonComponent {

  @Input() variant: AppButtonVariant = 'primary';
  @Input() type: AppButtonType = 'button';

  @Input() disabled = false;
  @Input() loading = false;
  @Input() fullWidth = false;

  @Input() ariaLabel?: string;

  @Output() clicked = new EventEmitter<MouseEvent>();

  handleClick(event: MouseEvent): void {
    if (this.disabled || this.loading) {
      event.preventDefault();
      event.stopPropagation();
      return;
    }

    this.clicked.emit(event);
  }
}