import { createContext, useContext, useState } from 'react'

type ChatCardType = {
    id: string
    participantUsername: string
    participantUserId: string
    lastMessage: string
    lastMessageTimestamp: Date
}

type ChatContextType = {
    chats: ChatCardType[]
    addChat: (chat: ChatCardType) => void
    removeChat: (chatId: string) => void
}

const ChatContext = createContext<ChatContextType>({
    chats: [],
    addChat: () => {},
    removeChat: () => {}
})

export const ChatProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
    const [chats, setChats] = useState<ChatCardType[]>([])

    const addChat = (chat: ChatCardType) => {
        setChats(prevChats => [...prevChats, chat])
    }

    const removeChat = (chatId: string) => {
        setChats(prevChats => prevChats.filter(chat => chat.id !== chatId))
    }

    return (
        <ChatContext.Provider value={{ chats, addChat, removeChat }}>
            {children}
        </ChatContext.Provider>
    )
}

export const useChat = () => useContext(ChatContext)