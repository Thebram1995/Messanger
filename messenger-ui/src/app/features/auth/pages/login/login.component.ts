import { Component } from '@angular/core';
import { AuthCardComponent } from '../../../../shared/ui/auth-card/auth-card.component';

@Component({
  selector: 'app-login',
  standalone: true,
  imports: [AuthCardComponent],
  templateUrl: './login.component.html',
  styleUrl: './login.component.scss'
})
export class LoginComponent {}