import {
  ChangeDetectionStrategy,
  Component,
  Input
} from '@angular/core';
import { CommonModule } from '@angular/common';

@Component({
  selector: 'app-logo-loader',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './logo-loader.component.html',
  styleUrl: './logo-loader.component.scss',
  changeDetection: ChangeDetectionStrategy.OnPush
})
export class LogoLoaderComponent {
  private currentProgress = 0;

  @Input()
  set progress(value: number) {
    this.currentProgress = Math.min(100, Math.max(0, Number(value) || 0));
  }

  get progress(): number {
    return this.currentProgress;
  }

  @Input() message = 'Cargando...';
  @Input() showPercentage = true;
  @Input() fullscreen = true;
}
