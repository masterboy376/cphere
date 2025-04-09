export namespace AuthRequest {
    export interface RegisterPayload {
        email: string
        password: string
        name: string
    }

    export interface LoginPayload {
        email: string
        password: string
    }
    
    export interface ResetPasswordPayload {
        email: string
    }
    
    export interface ChangePasswordPayload {
        password: string
        token: string
    }
}

export namespace UserRequest {
    export interface IsOnlinePath {
        user_id: string
    }

    export interface IsBatchOnlinePayload {
        user_ids: string[]
    }

    export interface SearchQuery {
        query: string
    }
}

export namespace ChatRequest {
    export interface CreatePayload {
        user_id: string
    }

    export interface DeletePayload {
        chat_id: string
    }

    export interface SendMessagePayload {
        chat_id: string
        message: string
    }
}

export namespace VideoCallRequest {
    export interface IntiatePayload {
        recipient_id: string,
        chat_id: string
    }

    export interface ResponsePayload {
        notification_id: string,
        accepted: boolean
    }
}