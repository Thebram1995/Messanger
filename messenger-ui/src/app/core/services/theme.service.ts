import { DOCUMENT } from '@angular/common';
import { Inject, Injectable } from '@angular/core';

@Injectable({
  providedIn: 'root'
})
export class ThemeService {
  private readonly storageKey = 'theme-hue';
  private readonly defaultHue = 230;

  constructor(
    @Inject(DOCUMENT) private readonly document: Document
  ) {
    this.applySavedTheme();
  }

  setHue(hue: number): void {
    const normalizedHue = Math.max(0, Math.min(360, hue));

    this.document.documentElement.style.setProperty(
      '--theme-hue',
      normalizedHue.toString()
    );

    localStorage.setItem(
      this.storageKey,
      normalizedHue.toString()
    );
  }

  getHue(): number {
    const savedHue = Number(localStorage.getItem(this.storageKey));

    return Number.isFinite(savedHue)
      ? savedHue
      : this.defaultHue;
  }

  private applySavedTheme(): void {
    this.setHue(this.getHue());
  }
}