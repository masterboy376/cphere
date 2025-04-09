import BackendApiService from '../BackendApiService';
import { ENDPOINTS } from '../../constants/Api';

class SocketBackendApiService extends BackendApiService {
  public async connect(): Promise<any> {
    const response = await this.axiosInstance.get(
      ENDPOINTS.SOCKET.CONNECT.uri);
    return response.data;
  }
}

// Export a singleton instance for ease of use
export default new SocketBackendApiService();