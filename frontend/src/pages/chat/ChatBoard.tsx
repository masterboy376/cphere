import { useState, useEffect } from 'react'
import { useParams, useNavigate } from 'react-router-dom'
import { ArrowLeftIcon, VideoCameraIcon, TrashIcon } from '@heroicons/react/24/outline'
import { UserAvatar } from '../../components/chat/UserAvatar'
import { MessageCard } from '../../components/chat/MessageCard'
import chatBackendApiService, { ChatsDeletePayload } from '../../services/chat/ChatBackendApiService'
import { ChatMessage, UserOnline, UserOffline, DeleteChat } from '../../types/WsMessageTypes'
import wsService from '../../services/ws/WsService'
import { useAuthentication } from '../../contexts/AuthenticationContext'
import videoBackendApiService, { VideoIntiatePayload } from '../../services/video/VideoBackendApiService'
import userBackendApiService from '../../services/user/UserBackendApiService'
import { useChat, ChatSummaryType } from '../../contexts/ChatContext'

interface Message {
  id: string
  chat_id: string
  sender_id: string
  content: string
  created_at: Date
}

export const ChatBoard = () => {
  const { chatId } = useParams()
  const navigate = useNavigate()
  const [messageContent, setMessageContent] = useState('')
  const [messages, setMessages] = useState<Message[]>([])
  const { authState } = useAuthentication()
  const [chatSummary, setChatSummary] = useState<ChatSummaryType>()
  const [participantOnlineStatus, setParticipantOnlineStatus] = useState<boolean>(false)
  const { toFrontendChatSummary } = useChat()

  const handleVideoCall = () => {
    if (chatId && chatSummary?.participantUserId) {
      const payload: VideoIntiatePayload = {
        chat_id: chatId,
        recipient_id: chatSummary?.participantUserId
      }
      videoBackendApiService.initiate(payload)
      navigate(`/video-call/${chatSummary?.participantUserId}`, { state: { accepted: false } })
    }
  }

  const handleDeleteChat = () => {
    if (chatId && chatSummary?.participantUserId) {
      chatBackendApiService.delete({ chat_id: chatId } as ChatsDeletePayload)
      wsService.sendMessage({
        type: 'delete_chat',
        target_user_id: chatSummary?.participantUserId,
        chat_id: chatId
      } as DeleteChat)
      navigate(-1)
    }
  }

  const handleSendMessage = () => {
    if (chatId && authState.userId && authState.username && messageContent.length > 0) {
      const newChatMessage: ChatMessage = {
        type: 'chat_message',
        message_id: null,
        chat_id: chatId,
        content: messageContent.trim(),
        sender_id: authState.userId,
        sender_username: authState.username,
        created_at: new Date()
      }
      wsService.sendMessage(newChatMessage)
      setMessageContent('')
    }
  }

  useEffect(() => {
    const initChatBoard = async () => {
      if (!chatId) return;

      try {
        // Fetch chat summary first
        const summaryData = await chatBackendApiService.getSummary(chatId);
        const frontendSummary = toFrontendChatSummary(summaryData);
        setChatSummary(frontendSummary);

        // fetch initial messages
        const initialMessages = await chatBackendApiService.getMessages(chatId);
        setMessages(initialMessages);

        // Now that chatSummary is available, fetch online status
        const onlineStatus = await userBackendApiService.isOnline(frontendSummary.participantUserId);
        setParticipantOnlineStatus(onlineStatus);

        // Setup WebSocket listeners

        const userOnlineListener = (message: UserOnline) => {
          if (message.type === 'user_online' && message.user_id === frontendSummary.participantUserId) {
            setParticipantOnlineStatus(true);
          }
        };

        const userOfflineListener = (message: UserOffline) => {
          if (message.type === 'user_offline' && message.user_id === frontendSummary.participantUserId) {
            setParticipantOnlineStatus(false);
          }
        };

        wsService.addEventListener('user_online', userOnlineListener);
        wsService.addEventListener('user_offline', userOfflineListener);

        // Cleanup listeners on unmount
        return () => {
          wsService.removeEventListener('user_online', userOnlineListener);
          wsService.removeEventListener('user_offline', userOfflineListener);
        };

      } catch (error) {
        console.error('Error initializing chat board:', error);
      }
    };

    initChatBoard();
  }, [chatId, toFrontendChatSummary]);

  useEffect(() => {
    const chatMessageListener = (message: ChatMessage) => {
      if (message.message_id && message.created_at) {
        const receivedMessage: Message = {
          id: message.message_id,
          chat_id: message.chat_id,
          sender_id: message.sender_id,
          content: message.content,
          created_at: new Date(message.created_at)
        };
        setMessages(prevMessages => [...prevMessages, receivedMessage]);
      }
    };

    const deleteChatListener = (message: DeleteChat) => {
      if (message.chat_id === chatId) {
        navigate(-1);
      }
    }

    wsService.addEventListener('chat_message', chatMessageListener);
    wsService.addEventListener('delete_chat', deleteChatListener);

    // Cleanup listeners on unmount
    return () => {
      wsService.removeEventListener('chat_message', chatMessageListener);
      wsService.removeEventListener('delete_chat', deleteChatListener);
    };
  }, []);

  return (
    <div className="flex flex-col h-full min-h-0">
      {/* Header */}
      <div className="p-4 flex items-center justify-between border-b border-background-lite">
        <div className="flex items-center gap-4">
          <button onClick={() => navigate(-1)} className="hover:bg-background-lite p-2 rounded-full transition-all duration-300 ease-in-out">
            <ArrowLeftIcon className="h-6 w-6" />
          </button>
          <div className="relative">
            <UserAvatar username={chatSummary ? chatSummary.participantUsername : ''} />
          </div>
          <div>
            <h2 className="font-medium text-text-primary">{chatSummary ? chatSummary.participantUsername : ''}</h2>
            <p className={`text-sm ${participantOnlineStatus ? 'text-green-500' : 'text-gray-400'}`}>
              {participantOnlineStatus ? 'Online' : 'Offline'}
            </p>
          </div>
        </div>

        <div className="flex items-center gap-4">
          <button className="hover:bg-background-lite p-2 rounded-full transition-all duration-300 ease-in-out"
            onClick={handleVideoCall}>
            <VideoCameraIcon className="h-6 w-6" />
          </button>
          <button className="text-red-500 hover:bg-background-lite p-2 rounded-full transition-all duration-300 ease-in-out"
            onClick={handleDeleteChat}>
            <TrashIcon className="h-6 w-6" />
          </button>
        </div>
      </div>

      {/* Messages */}
      <div className="flex-1 overflow-y-auto min-h-0 p-4 space-y-4">
        {messages.length === 0 ? (
          <div className="flex items-center justify-center h-full">
            <p className="text-text-secondary">Start chatting.</p>
          </div>
        ) : (
          messages.map(message => (
            <MessageCard key={message.id} {...message} />
          ))
        )}
      </div>

      {/* Input Area */}
      <div className="p-4 border-t border-background-lite">
        <div className="flex items-center gap-4">
          <input
            value={messageContent}
            onChange={(e) => setMessageContent(e.target.value)}
            onKeyPress={(e) => e.key === 'Enter' && handleSendMessage()}
            placeholder="Type a message..."
            className="flex-1 p-3 bg-background rounded-lg border border-background-lite focus:outline-none"
          />
          <button
            onClick={handleSendMessage}
            className="bg-primary hover:bg-primary/80 text-white p-3 rounded-full transition-all duration-300 ease-in-out"
          >
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth={1.5} stroke="currentColor" className="size-6">
              <path strokeLinecap="round" strokeLinejoin="round" d="M6 12 3.269 3.125A59.769 59.769 0 0 1 21.485 12 59.768 59.768 0 0 1 3.27 20.875L5.999 12Zm0 0h7.5" />
            </svg>
          </button>
        </div>
      </div>
    </div>
  )
}