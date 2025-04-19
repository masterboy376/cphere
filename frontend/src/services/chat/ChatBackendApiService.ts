import BackendApiService from "../BackendApiService";
import { ENDPOINTS } from "../../constants/Api";

export interface ChatsCreatePayload {
  participant_id: string
}

export interface ChatsDeletePayload {
  chat_id: string
}

export interface ChatsSendMessagePayload {
  chat_id: string
  message: string
}

class ChatBackendApiService extends BackendApiService {
  // Create a new chat room
  public async create(data: ChatsCreatePayload): Promise<any> {
    const response = await this.axiosInstance.post(
      ENDPOINTS.CHATS.CREATE.uri, data);
    return response.data;
  }

  // Delete a chat room
  public async delete(data: ChatsDeletePayload): Promise<any> {
    const response = await this.axiosInstance.post(
      ENDPOINTS.CHATS.DELETE.uri, data);
    return response.data;
  }

  // Send a chat message
  public async sendMessage(data: ChatsSendMessagePayload): Promise<any> {
    const response = await this.axiosInstance.post(
      ENDPOINTS.CHATS.SEND_MESSAGE.uri, data);
    return response.data;
  }

  // Get chat messages by providing the chat ID in the URL path
  public async getMessages(chatId: string): Promise<any> {
    const response = await this.axiosInstance.get(
      ENDPOINTS.CHATS.MESSAGES.uri(chatId));
    return response.data;
  }

  public async getSummary(chatId: string): Promise<any> {
    const response = await this.axiosInstance.get(
      ENDPOINTS.CHATS.SUMMARY.uri(chatId));
    return response.data;
  }
}

export default new ChatBackendApiService();