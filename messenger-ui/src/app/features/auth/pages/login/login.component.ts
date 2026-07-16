import {
  Component,
  inject,
  OnDestroy
} from '@angular/core';

import {
  FormBuilder,
  ReactiveFormsModule,
  Validators
} from '@angular/forms';

import {
  Router,
  RouterLink
} from '@angular/router';

import { AppButtonComponent } from '../../../../shared/ui/app-button/app-button.component';
import { AppInputComponent } from '../../../../shared/ui/app-input/app-input.component';
import { AppPasswordComponent } from '../../../../shared/ui/app-password/app-password.component';
import { AuthCardComponent } from '../../../../shared/ui/auth-card/auth-card.component';

@Component({
  selector: 'app-login',
  standalone: true,
  imports: [
    ReactiveFormsModule,
    RouterLink,
    AppButtonComponent,
    AppInputComponent,
    AppPasswordComponent,
    AuthCardComponent
  ],
  templateUrl: './login.component.html',
  styleUrl: './login.component.scss'
})
export class LoginComponent implements OnDestroy {

  private readonly formBuilder = inject(FormBuilder);
  private readonly router = inject(Router);

  loading = false;
  leavingPage = false;

  private navigationTimeout?: ReturnType<typeof setTimeout>;

  readonly loginForm = this.formBuilder.nonNullable.group({
    email: [
      '',
      [
        Validators.required,
        Validators.email
      ]
    ],
    password: [
      '',
      [
        Validators.required,
        Validators.minLength(8)
      ]
    ]
  });

  get emailControl() {
    return this.loginForm.controls.email;
  }

  get passwordControl() {
    return this.loginForm.controls.password;
  }

  get emailErrorMessage(): string {
    if (this.emailControl.hasError('required')) {
      return 'El correo electrónico es obligatorio.';
    }

    if (this.emailControl.hasError('email')) {
      return 'Escribe un correo electrónico válido.';
    }

    return '';
  }

  get passwordErrorMessage(): string {
    if (this.passwordControl.hasError('required')) {
      return 'La contraseña es obligatoria.';
    }

    if (this.passwordControl.hasError('minlength')) {
      return 'La contraseña debe tener al menos 8 caracteres.';
    }

    return '';
  }
  submitLoginForm(): void {
    if (this.loginForm.invalid) {
      this.loginForm.markAllAsTouched();
      return;
    }

    // Evita múltiples peticiones mientras el usuario espera la respuesta del servidor.
    this.loading = true;

    console.log(this.loginForm.getRawValue());

    // Simula temporalmente una respuesta mientras conectamos el backend.
  setTimeout(() => {
    this.loading = false;
  }, 1500);
  }

  returnToAccess(): void {
    if (this.leavingPage) {
      return;
    }

    // Espera a que termine la transición antes de destruir la pantalla actual.
    this.leavingPage = true;

    this.navigationTimeout = setTimeout(() => {
      void this.router.navigateByUrl('/');
    }, 220);
  }

  ngOnDestroy(): void {
    if (this.navigationTimeout) {
      clearTimeout(this.navigationTimeout);
    }
  }
}