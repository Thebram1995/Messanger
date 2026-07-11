import { Component } from '@angular/core';
import { ThemeService } from '../../../core/services/theme.service';

@Component({
  selector: 'app-theme-slider',
  standalone: true,
  imports: [],
  templateUrl: './theme-slider.component.html',
  styleUrl: './theme-slider.component.scss'
})
export class ThemeSliderComponent {
  hue: number;

  constructor(
    private readonly themeService: ThemeService
  ) {
    this.hue = this.themeService.getHue();
  }

  onHueChange(event: Event): void {
    const input = event.target as HTMLInputElement;
    const hue = Number(input.value);

    this.hue = hue;
    this.themeService.setHue(hue);
  }
}