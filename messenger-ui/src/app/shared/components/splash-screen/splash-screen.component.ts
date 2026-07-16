import {
  ChangeDetectionStrategy,
  ChangeDetectorRef,
  Component,
  EventEmitter,
  OnDestroy,
  OnInit,
  Output
} from '@angular/core';

type SplashPhase =
  | 'filling'
  | 'rising'
  | 'falling'
  | 'impact'
  | 'settled';

@Component({
  selector: 'app-splash-screen',
  standalone: true,
  templateUrl: './splash-screen.component.html',
  styleUrl: './splash-screen.component.scss',
  changeDetection: ChangeDetectionStrategy.OnPush
})
export class SplashScreenComponent implements OnInit, OnDestroy {

  @Output() settled = new EventEmitter<void>();

  progress = 0;
  phase: SplashPhase = 'filling';

  private fillInterval?: ReturnType<typeof setInterval>;
  private riseTimeout?: ReturnType<typeof setTimeout>;
  private fallTimeout?: ReturnType<typeof setTimeout>;
  private impactTimeout?: ReturnType<typeof setTimeout>;

  constructor(
    private readonly changeDetectorRef: ChangeDetectorRef
  ) {}

  ngOnInit(): void {
    this.startFillAnimation();
  }

  ngOnDestroy(): void {
    this.clearTimers();
  }

  private startFillAnimation(): void {
    this.fillInterval = setInterval(() => {
      this.progress = Math.min(100, this.progress + 2);
      this.changeDetectorRef.markForCheck();

      if (this.progress >= 100) {
        this.stopFillInterval();
        this.startRiseAnimation();
      }
    }, 22);
  }

  private startRiseAnimation(): void {
    this.phase = 'rising';
    this.changeDetectorRef.markForCheck();

    this.riseTimeout = setTimeout(() => {
      this.phase = 'falling';
      this.changeDetectorRef.markForCheck();

      this.startImpactAnimation();
    }, 430);
  }

  private startImpactAnimation(): void {
    this.fallTimeout = setTimeout(() => {
      this.phase = 'impact';
      this.changeDetectorRef.markForCheck();

      this.impactTimeout = setTimeout(() => {
        this.phase = 'settled';
        this.changeDetectorRef.markForCheck();
        this.settled.emit();
      }, 520);
    }, 260);
  }

  private stopFillInterval(): void {
    if (!this.fillInterval) {
      return;
    }

    clearInterval(this.fillInterval);
    this.fillInterval = undefined;
  }

  private clearTimers(): void {
    this.stopFillInterval();

    if (this.riseTimeout) {
      clearTimeout(this.riseTimeout);
    }

    if (this.fallTimeout) {
      clearTimeout(this.fallTimeout);
    }

    if (this.impactTimeout) {
      clearTimeout(this.impactTimeout);
    }
  }
}