export type RegisterRole = 'USER' | 'CLAN_LEADER';

export interface RegisterRequest {
  username: string;
  display_name: string;
  email: string;
  password: string;
  role: RegisterRole;
}