import axios from 'axios';
import { API_BASE_URL } from '../constants/Api';

// Backend API service
class BackendApiService {
  // Create an Axios instance with a base URL and cookie handling enabled.
  public axiosInstance = axios.create({
    baseURL: API_BASE_URL, // adjust as needed
    withCredentials: true, // ensures cookies (session tokens) are sent
  });
}

// Export a singleton instance for ease of use
export default BackendApiService;
