import BackendApiService from '../BackendApiService';
import { ENDPOINTS } from '../../constants/Api';

export interface UserIsOnlinePath {
    user_id: string
}

export interface UserIsBatchOnlinePayload {
    user_ids: string[]
}

export interface UserSearchQuery {
    query: string
}

class UserBackendApiService extends BackendApiService {
    public async isOnline(user_id: string): Promise<any> {
        const response = await this.axiosInstance.get(
            ENDPOINTS.USERS.IS_ONLINE.uri(user_id));
        return response.data;
    }

    public async isBatchOnline(data: UserIsBatchOnlinePayload): Promise<any> {
        const response = await this.axiosInstance.post(
            ENDPOINTS.USERS.IS_BATCH_ONLINE.uri, data);
        return response.data;
    }

    public async search(data: UserSearchQuery): Promise<any> {
        const response = await this.axiosInstance.get(
            ENDPOINTS.USERS.SEARCH.uri, { params: data });
        return response.data;
    }

    public async getChats(): Promise<any> {
        const response = await this.axiosInstance.get(
            ENDPOINTS.USERS.CHATS.uri);
        return response.data;
    }

    public async getNotifications(): Promise<any> {
        const response = await this.axiosInstance.get(
            ENDPOINTS.USERS.NOTIFICATIONS.uri);
        return response.data;
    }

    public async getProfile(): Promise<any> {
        const response = await this.axiosInstance.get(
            ENDPOINTS.USERS.PROFILE.uri);
        return response.data;
    }
}

// Export a singleton instance for ease of use
export default new UserBackendApiService();