import { useState } from 'react'
import { useParams, useNavigate } from 'react-router-dom'
import { ArrowLeftIcon, VideoCameraIcon, TrashIcon } from '@heroicons/react/24/outline'
import { UserAvatar } from '../../components/chat/UserAvatar'
import { MessageCard } from '../../components/chat/MessageCard'

export const ChatBoard = () => {
  const { chatId } = useParams()
  const navigate = useNavigate()
  const [newMessage, setNewMessage] = useState('')
  const [messages, setMessages] = useState([
    {
      id: '1',
      text: 'Hey there!',
      sender: 'other',
      timestamp: new Date(Date.now() - 3600000)
    },
    // Add more messages...
  ])

  const handleVideoCall = () => {
    navigate(`/video-call/${chatId}`)
  }

  const handleSendMessage = () => {
    if (newMessage.trim()) {
      setMessages([...messages, {
        id: Date.now().toString(),
        text: newMessage,
        sender: 'me',
        timestamp: new Date()
      }])
      setNewMessage('')
    }
  }

  return (
    <div className="flex flex-col h-full min-h-0">
      {/* Header */}
      <div className="p-4 flex items-center justify-between border-b border-background-lite">
        <div className="flex items-center gap-4">
          <button onClick={() => navigate(-1)} className="hover:bg-background-lite p-2 rounded-full transition-all duration-300 ease-in-out">
            <ArrowLeftIcon className="h-6 w-6" />
          </button>
          <UserAvatar username="JohnDoe" />
          <div>
            <h2 className="font-medium text-text-primary">JohnDoe</h2>
            <p className="text-sm text-text-secondary">Online</p>
          </div>
        </div>

        <div className="flex items-center gap-4">
          <button className="hover:bg-background-lite p-2 rounded-full transition-all duration-300 ease-in-out"
            onClick={handleVideoCall}>
            <VideoCameraIcon className="h-6 w-6" />
          </button>
          <button className="text-red-500 hover:bg-background-lite p-2 rounded-full transition-all duration-300 ease-in-out">
            <TrashIcon className="h-6 w-6" />
          </button>
        </div>
      </div>

      {/* Messages */}
      <div className="flex-1 overflow-y-auto min-h-0 p-4 space-y-4">
        {messages.map(message => (
          <MessageCard key={message.id} {...message} />
        ))}
      </div>

      {/* Input Area */}
      <div className="p-4 border-t border-background-lite">
        <div className="flex items-center gap-4">
          <input
            value={newMessage}
            onChange={(e) => setNewMessage(e.target.value)}
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