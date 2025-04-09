import BackendApiService from '../BackendApiService';
import { ENDPOINTS } from '../../constants/Api';

// Define request payload types for authentication APIs
export interface AuthRegisterRequest {
  username: string;
  email: string;
  password: string;
}

export interface AuthLoginRequest {
  username: string;
  password: string;
}

export interface AuthResetPasswordRequest {
  email: string;
}

export interface AuthChangePasswordRequest {
  reset_token: string;
  new_password: string;
}

// Backend API service for auth endpoints
class AuthBackendApiService extends BackendApiService {
  // Register new user
  public async register(data: AuthRegisterRequest): Promise<any> {
    const response = await this.axiosInstance.post(
      ENDPOINTS.AUTH.REGISTER.uri, data);
    return response.data;
  }

  // Login user and store session cookie automatically
  public async login(data: AuthLoginRequest): Promise<any> {
    const response = await this.axiosInstance.post(
      ENDPOINTS.AUTH.LOGIN.uri, data);
    return response.data;
  }

  // Logout user
  public async logout(): Promise<any> {
    const response = await this.axiosInstance.post(
      ENDPOINTS.AUTH.LOGOUT.uri);
    return response.data;
  }

  public async authStatus(): Promise<any> {
    const response = await this.axiosInstance.get(
      ENDPOINTS.AUTH.STATUS.uri);
    return response.data;
  }

  // Reset password (e.g., send reset password email)
  public async resetPassword(data: AuthResetPasswordRequest): Promise<any> {
    const response = await this.axiosInstance.post(
      ENDPOINTS.AUTH.RESET_PASSWORD.uri, data);
    return response.data;
  }

  // Change password (using a token from reset email, for example)
  public async changePassword(data: AuthChangePasswordRequest): Promise<any> {
    const response = await this.axiosInstance.post(
      ENDPOINTS.AUTH.CHANGE_PASSWORD.uri, data);
    return response.data;
  }
}

// Export a singleton instance for ease of use
export default new AuthBackendApiService();
