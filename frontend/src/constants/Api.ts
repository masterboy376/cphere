export const API_BASE_URL = 'http://localhost:8080/';

export const ENDPOINTS = {
  AUTH: {
    REGISTER: {
      uri: '/auth/register',
      method: 'post',
    },
    LOGIN: {
      uri: '/auth/login',
      method: 'post',
    },
    LOGOUT: {
      uri: '/auth/logout',
      method: 'post',
    },
    STATUS: {
      uri: '/auth/auth_status',
      method: 'get',
    },
    RESET_PASSWORD: {
      uri: '/auth/reset_password',
      method: 'post',
    },
    CHANGE_PASSWORD: {
      uri: '/auth/change_password',
      method: 'post',
    },
  },
  USERS: {
    IS_ONLINE: {
      uri: (user_id: string) => `/users/${user_id}/is_online`,
      method: 'get',
    },
    IS_BATCH_ONLINE: {
      uri: `/users/is_batch_online`,
      method: 'post',
    },
    CHATS: {
      uri: '/users/chats',
      method: 'get',
    },
    NOTIFICATIONS: {
      uri: '/users/get_notifications',
      method: 'get',
    },
    SEARCH: {
      uri: '/search_users',
      method: 'get',
    },
    PROFILE: {
      uri: '/users/profile',
      method: 'get',
    },
  },
  CHATS: {
    CREATE: {
      uri: '/chats/create',
      method: 'post',
    },
    DELETE: {
      uri: '/chats/delete',
      method: 'post',
    },
    SEND_MESSAGE: {
      uri: '/chats/send_message',
      method: 'post',
    },
    MESSAGES: {
      uri: (chat_id: string) => `/chats/${chat_id}/messages`,
      method: 'get',
    },
  },
  VIDEO_CALL: {
    INITIATE: {
      uri: '/video_call/initiate',
      method: 'post',
    },
    RESPOND: {
      uri: '/video_call/respond',
      method: 'post',
    },
  },
  SOCKET: {
    CONNECT: {
      uri: '/socket/connect',
      method: 'get',
    },
  },
};