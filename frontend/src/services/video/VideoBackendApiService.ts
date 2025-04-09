import BackendApiService from '../BackendApiService';
import { ENDPOINTS } from '../../constants/Api';

export interface VideoIntiatePayload {
    recipient_id: string,
    chat_id: string
}

export interface VideoRespondPayload {
    notification_id: string,
    accepted: boolean
}

class VideoBackendApiService extends BackendApiService {
    public async initiate(data: VideoIntiatePayload): Promise<any> {
        const response = await this.axiosInstance.post(
            ENDPOINTS.VIDEO_CALL.INITIATE.uri, data);
        return response.data;
    }

    public async respond(data: VideoRespondPayload): Promise<any> {
        const response = await this.axiosInstance.post(
            ENDPOINTS.VIDEO_CALL.RESPOND.uri, data);
        return response.data;
    }
}

// Export a singleton instance for ease of use
export default new VideoBackendApiService();