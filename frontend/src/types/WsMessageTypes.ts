import { NotificationSummaryBackendType } from "../contexts/NotificationContext";

export type WsMessage =
  DeleteChat
  | ChatMessage
  | UserOnline
  | UserOffline
  | WebrtcOffer
  | WebrtcAnswer
  | WebrtcIceCandidate
  | VideoCallEnd
  | VideoCallRequest
  | VideoCallResponse
  | LogoutMessage
  ;

export interface DeleteChat {
  type: "delete_chat";
  target_user_id: string;
  chat_id: string;
}

export interface ChatMessage {
  type: "chat_message";
  message_id: string | null;
  chat_id: string;
  content: string;
  sender_id: string;
  sender_username: string;
  created_at: Date | null;
}

export interface UserOffline {
  type: "user_offline";
  user_id: string;
}

export interface UserOnline {
  type: "user_online";
  user_id: string;
}

export interface WebrtcOffer {
  type: "webrtc_offer";
  target_user_id: string;
  offer: any;
}

export interface WebrtcAnswer {
  type: "webrtc_answer";
  target_user_id: string;
  answer: any;
}

export interface WebrtcIceCandidate {
  type: "webrtc_ice_candidate";
  target_user_id: string;
  candidate: any;
}

export interface VideoCallEnd {
  type: "video_call_ended";
  target_user_id: string;
}

export interface VideoCallRequest {
  type: "video_call_request";
  notification: NotificationSummaryBackendType;
}

export interface VideoCallResponse {
  type: "video_call_accepted" | "video_call_declined";
  caller_id: string;
  response?: string; // additional payload if needed
}

interface LogoutMessage {
  type: "logout";
}

// For chat ordering we dispatch a separate type or simply reuse the chat_message event.
// Here, we assume that when a chat message event is received, the component handling the chats list
// will reorder the chats automatically.
