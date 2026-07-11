import { Component, OnDestroy, OnInit } from '@angular/core';
import { RouterOutlet, RouterLink } from '@angular/router';
import { CommonModule } from '@angular/common';
import { LogoLoaderComponent } from './shared/components/logo-loader/logo-loader.component';

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [
    CommonModule,
    RouterOutlet,
    RouterLink,
    LogoLoaderComponent
  ],
  templateUrl: './app.component.html',
  styleUrl: './app.component.scss'
})
export class AppComponent implements OnInit, OnDestroy {
  title = 'clan-wars';
  progress = 0;
  loading = true;
  hidingLoader = false;

  private intervalId?: ReturnType<typeof setInterval>;
  private timeoutId?: ReturnType<typeof setTimeout>;

  ngOnInit(): void {
    this.startLoader();
  }

  ngOnDestroy(): void {
    this.clearTimers();
  }

  private startLoader(): void {
    this.intervalId = setInterval(() => {
      if (this.progress < 100) {
        this.progress += 1;
        return;
      }

      this.finishLoader();
    }, 35);
  }

  private finishLoader(): void {
    if (this.intervalId) {
      clearInterval(this.intervalId);
      this.intervalId = undefined;
    }

    this.timeoutId = setTimeout(() => {
      this.hidingLoader = true;

      this.timeoutId = setTimeout(() => {
        this.loading = false;
        this.hidingLoader = false;
      }, 450);
    }, 300);
  }

  private clearTimers(): void {
    if (this.intervalId) {
      clearInterval(this.intervalId);
    }

    if (this.timeoutId) {
      clearTimeout(this.timeoutId);
    }
  }
}
