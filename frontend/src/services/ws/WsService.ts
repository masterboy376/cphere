import { WsMessage, ChatMessage } from '../../types/WsMessageTypes';
import { WEBSOCKET_URL } from '../../constants/Api';

// You may use an EventEmitter implementation or create your own simple PubSub.
type ListenerCallback<T = any> = (data: T) => void;

interface ListenerStore {
  [eventType: string]: ListenerCallback[];
}

class WebSocketService {
  private static instance: WebSocketService;
  private ws: WebSocket | null = null;
  private listeners: ListenerStore = {};

  private constructor() { }

  public static getInstance(): WebSocketService {
    if (!WebSocketService.instance) {
      WebSocketService.instance = new WebSocketService();
    }
    return WebSocketService.instance;
  }

  public connect(wsUrl: string = WEBSOCKET_URL): void {
    if (this.ws?.readyState === WebSocket.OPEN) {
      console.warn("WebSocket already connected.");
      return;
    }

    this.ws = new WebSocket(wsUrl);

    this.ws.onopen = () => {
      console.log("WebSocket connection established.");
      this.dispatchEvent("connection_open", {});
    };

    this.ws.onmessage = (event: MessageEvent) => {
      this.handleMessage(event.data);
    };

    this.ws.onerror = (error) => {
      console.error("WebSocket encountered error:", error);
      this.dispatchEvent("connection_error", error);
    };

    this.ws.onclose = () => {
      console.log("WebSocket connection closed.");
      this.dispatchEvent("connection_close", {});
    };
  }

  private handleMessage(data: any): void {
    try {
      const message: WsMessage = JSON.parse(data);
      this.dispatchEvent(message.type, message);

      if (message.type === "chat_message") {
        this.dispatchEvent("chat_update", message as ChatMessage);
      }
    } catch (error) {
      console.error("Failed to parse WebSocket message:", error);
    }
  }

  public sendMessage(message: WsMessage): void {
    if (this.ws && this.ws.readyState === WebSocket.OPEN) {
      const msgString = JSON.stringify(message);
      this.ws.send(msgString);
    } else {
      console.error("WebSocket is not open. Cannot send message.");
    }
  }

  public addEventListener(eventType: string, callback: ListenerCallback): void {
    if (!this.listeners[eventType]) {
      this.listeners[eventType] = [];
    }
    this.listeners[eventType].push(callback);
  }

  public removeEventListener(eventType: string, callback: ListenerCallback): void {
    const listeners = this.listeners[eventType];
    if (!listeners) return;

    // Filter out the callback
    this.listeners[eventType] = listeners.filter((cb) => cb !== callback);
  }

  private dispatchEvent(eventType: string, data: any): void {
    if (this.listeners[eventType]) {
      this.listeners[eventType].forEach((callback) => {
        callback(data);
      });
    }
  }

  /**
   * Disconnects the WebSocket connection.
   */
  public disconnect(): void {
    if (this.ws?.readyState === WebSocket.OPEN ||
      this.ws?.readyState === WebSocket.CONNECTING) {
      // Use code 1000 (Normal Closure) and a reason for immediate closure
      this.ws.close(1000, "Websocket connection closed.");
    }
  }
}

export default WebSocketService.getInstance();
