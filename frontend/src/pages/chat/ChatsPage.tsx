import { useEffect, useState } from 'react'
import { ChatCard } from '../../components/chat/ChatCard'
import { Footer } from '../../components/common/Footer'
import userBackendApiService from '../../services/user/UserBackendApiService'
import { useChat, ChatSummaryType } from '../../contexts/ChatContext'
import { Loader } from '../../components/common/Loader'
import { ChatMessage, DeleteChat } from '../../types/WsMessageTypes'
import wsService from '../../services/ws/WsService'

export const ChatsPage = () => {
  const { chats, setChats, addChat, removeChat, toFrontendChatSummary } = useChat()
  const [loading, setLoading] = useState(false)

  useEffect(() => {
    const fetchChats = async () => {
      try {
        setLoading(true)
        setChats([])
        const response = await userBackendApiService.getChats()

        if (response && Array.isArray(response)) {
          // Transform backend data to frontend format and update context
          const frontendChats = response.map(chat => toFrontendChatSummary(chat))

          setChats(frontendChats)
        }
      } catch (error) {
        console.error('Failed to fetch chats:', error)
      } finally {
        setLoading(false)
      }
    }

    const chatUpdateListener = async (message: ChatMessage) => {
      if (message.created_at) {
        const newChat: ChatSummaryType = {
          id: message.chat_id,
          participantUserId: message.sender_id,
          participantUsername: message.sender_username,
          lastMessage: message.content,
          lastMessageTimestamp: new Date(message.created_at)
        }
        addChat(newChat)
      }
    }

    const deleteChatListener = (message: DeleteChat) => {
      removeChat(message.chat_id)
    }

    fetchChats()
    wsService.addEventListener('chat_update', chatUpdateListener)
    wsService.addEventListener('delete_chat', deleteChatListener)

    return () => {
      wsService.removeEventListener('chat_update', chatUpdateListener)
      wsService.removeEventListener('delete_chat', deleteChatListener)
    }
  }, [])

  return (
    <div className="flex flex-col h-full min-h-0">
      {/* heading */}
      <div className="p-4 border-b border-background-lite">
        <h1 className="font-semibold text-2xl">Chats</h1>
      </div>

      {/* main content */}
      {loading ? (
        <div className="flex-1 overflow-y-auto min-h-0">
          <Loader message="Loading chats..." />
        </div>
      ) : (
        <div className="flex-1 overflow-y-auto min-h-0">
          {chats.length === 0 ? (
            <div className="p-8 text-center text-text-secondary">No chats found</div>
          ) : (
            chats.map(chat => (
              <ChatCard key={chat.id} {...chat} />
            ))
          )}
        </div>
      )}


      {/* footer */}
      <Footer />
    </div>
  )
}