import {
  Component,
  inject,
  OnDestroy
} from '@angular/core';

import { Router } from '@angular/router';

import {
  SplashScreenComponent
} from '../../../shared/components/splash-screen/splash-screen.component';

@Component({
  selector: 'app-access-page',
  standalone: true,
  imports: [
    SplashScreenComponent
  ],
  templateUrl: './access-page.component.html',
  styleUrl: './access-page.component.scss'
})
export class AccessPageComponent implements OnDestroy {

  private readonly router = inject(Router);

  interfaceVisible = false;
  leavingPage = false;

  private navigationTimeout?: ReturnType<typeof setTimeout>;

  onSplashSettled(): void {
    this.interfaceVisible = true;
  }

  navigateTo(path: string): void {
    if (this.leavingPage) {
      return;
    }

    // Evita múltiples navegaciones mientras se ejecuta la animación de salida.
    this.leavingPage = true;

    this.navigationTimeout = setTimeout(() => {
      void this.router.navigateByUrl(path);
    }, 220);
  }

  ngOnDestroy(): void {
    if (this.navigationTimeout) {
      clearTimeout(this.navigationTimeout);
    }
  }
}