import { createContext, useContext, useState } from 'react'

type ChatSummaryType = {
    id: string
    participantUsername: string
    participantUserId: string
    lastMessage: string
    lastMessageTimestamp: Date
}

type ChatSummaryBackendType = {
    id: string
    participant_username: string
    participant_user_id: string
    last_message: string
    last_message_timestamp: Date
}

type ChatContextType = {
    chats: ChatSummaryType[]
    setChats: (chats: ChatSummaryType[]) => void
    addChat: (chat: ChatSummaryType) => void
    removeChat: (chatId: string) => void
    toFrontendChatSummary: (chat: ChatSummaryBackendType) => ChatSummaryType
}

const ChatContext = createContext<ChatContextType>({
    chats: [],
    addChat: () => {},
    setChats: () => {},
    removeChat: () => {},
    toFrontendChatSummary: () => ({
        id: '',
        participantUsername: '',
        participantUserId: '',
        lastMessage: '',
        lastMessageTimestamp: new Date()
    })
})

export const ChatProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
    const [chats, setChats] = useState<ChatSummaryType[]>([])

    const addChat = (chat: ChatSummaryType) => {
        setChats(prevChats => {
            // Filter out the chat if it already exists
            const filteredChats = prevChats.filter(c => c.id !== chat.id);
            // Add the new chat at the beginning (most recent first)
            return [chat, ...filteredChats];
        });
    }

    const removeChat = (chatId: string) => {
        setChats(prevChats => prevChats.filter(chat => chat.id !== chatId))
    }

    const toFrontendChatSummary = (chat: ChatSummaryBackendType): ChatSummaryType => ({
        id: chat.id,
        participantUsername: chat.participant_username,
        participantUserId: chat.participant_user_id,
        lastMessage: chat.last_message,
        lastMessageTimestamp: new Date(chat.last_message_timestamp)
    })

    return (
        <ChatContext.Provider value={{ chats, addChat, setChats, removeChat, toFrontendChatSummary }}>
            {children}
        </ChatContext.Provider>
    )
}

export const useChat = () => useContext(ChatContext)
export type { ChatSummaryType }