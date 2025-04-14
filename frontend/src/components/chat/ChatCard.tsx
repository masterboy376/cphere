import { useEffect, useRef, useState } from 'react'
import { EllipsisVerticalIcon, VideoCameraIcon, TrashIcon } from '@heroicons/react/24/outline'
import { useNavigate } from 'react-router-dom'
import { UserAvatar } from './UserAvatar'
import { useChat } from '../../contexts/ChatContext'
import chatBackendApiService, { ChatsDeletePayload } from '../../services/chat/ChatBackendApiService'

interface ChatCardProps {
  id: string
  participantUsername: string
  participantUserId: string
  lastMessage: string
  lastMessageTimestamp: Date
}

export const ChatCard = ({ id, participantUsername, lastMessage, lastMessageTimestamp }: ChatCardProps) => {
  const [isMenuOpen, setIsMenuOpen] = useState(false)
  const navigate = useNavigate()
  const { removeChat } = useChat()

  const menuButtonRef = useRef<HTMLButtonElement>(null)
  const menuRef = useRef<HTMLDivElement>(null)

  const handleChatClick = () => {
    navigate(`/chats/${id}`)
  }

  const handleVideoCall = () => {
    navigate(`/video-call/${id}`)
  }
  
  const handleDeleteChat = async () => {
    const payload: ChatsDeletePayload = {
      chat_id: id
    }
    try {
      await chatBackendApiService.delete(payload)
      removeChat(id)
      setIsMenuOpen(false)
    } catch (error) {
      console.error('Error deleting chat:', error)
    }
  }

  const handleClickOutsideMenu = (event: MouseEvent) => {
    // Check if click is outside both menu and button
    if (
      menuRef.current && 
      !menuRef.current.contains(event.target as Node) &&
      menuButtonRef.current &&
      !menuButtonRef.current.contains(event.target as Node)
    ) {
      setIsMenuOpen(false)
    }
  }

  useEffect(() => {
    if (isMenuOpen) {
      document.addEventListener('mousedown', handleClickOutsideMenu)
    } else {
      document.removeEventListener('mousedown', handleClickOutsideMenu)
    }

    return () => {
      document.removeEventListener('mousedown', handleClickOutsideMenu)
    }
  }, [isMenuOpen])

  return (
    <div className="group relative flex items-center p-4 hover:bg-background-paper transition-all duration-300 ease-in-out cursor-pointer border-b border-background-lite">
      <div className="flex-1 flex items-center gap-4" onClick={handleChatClick}>
        <UserAvatar username={participantUsername} />
        <div className="flex-1 flex">
          <div className="flex-1 flex flex-col items-start justify-between">
            <h3 className="font-medium text-text-primary">{participantUsername}</h3>
            <p className="text-sm text-text-secondary truncate">{lastMessage}</p>
          </div>
          <span className="text-sm flex flex-col text-text-secondary justify-center mr-2">
            {new Date(lastMessageTimestamp).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })}
          </span>
        </div>
      </div>

      <div className="relative">
        <button 
          ref={menuButtonRef}
          onClick={(e) => {
            e.stopPropagation()
            setIsMenuOpen(!isMenuOpen)
          }}
          className= "p-2 rounded-full hover:bg-background-lite transition-all duration-300 ease-in-out"
        >
          <EllipsisVerticalIcon className="h-6 w-6" />
        </button>

        {isMenuOpen && (
          <div className="absolute right-0 mt-2 w-48 bg-background-paper rounded-lg shadow-lg z-20" ref={menuRef}>
            <button
              className="w-full px-4 py-3 flex items-center gap-3 hover:bg-background-lite text-text-primary"
              onClick={handleVideoCall}
            >
              <VideoCameraIcon className="h-5 w-5" />
              Video Call
            </button>
            <button
              className="w-full px-4 py-3 flex items-center gap-3 hover:bg-background-lite text-red-500"
              onClick={handleDeleteChat}
            >
              <TrashIcon className="h-5 w-5" />
              Delete Chat
            </button>
          </div>
        )}
      </div>
    </div>
  )
}