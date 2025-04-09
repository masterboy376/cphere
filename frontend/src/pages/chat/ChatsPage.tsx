import { useState } from 'react'
import { ChatCard } from '../../components/chat/ChatCard'
import { Footer } from '../../components/common/Footer'

const mockChats = [
  {
    id: '1',
    participantUsername: 'JohnDoe',
    participantUserId: '123',
    lastMessage: 'Hey, how are you?',
    lastMessageTimestamp: new Date(Date.now() - 3600000)
  },{
    id: '2',
    participantUsername: 'JohnDoe',
    participantUserId: '123',
    lastMessage: 'Hey, how are you?',
    lastMessageTimestamp: new Date(Date.now() - 3600000)
  },{
    id: '3',
    participantUsername: 'JohnDoe',
    participantUserId: '123',
    lastMessage: 'Hey, how are you?',
    lastMessageTimestamp: new Date(Date.now() - 3600000)
  },{
    id: '4',
    participantUsername: 'JohnDoe',
    participantUserId: '123',
    lastMessage: 'Hey, how are you?',
    lastMessageTimestamp: new Date(Date.now() - 3600000)
  },
  // Add more mock chats...
]

export const ChatsPage = () => {
  const [chats] = useState(mockChats)

  return (
    <div className="flex flex-col h-full min-h-0">
      {/* heading */}
      <div className="p-4 border-b border-background-lite">
        <h1 className="font-semibold text-2xl">Chats</h1>
      </div>

      {/* main content */}
      <div className="flex-1 overflow-y-auto min-h-0">
        {chats.length === 0 ? (
          <div className="p-8 text-center text-text-secondary">No chats found</div>
        ) : (
          chats.map(chat => (
            <ChatCard key={chat.id} {...chat} />
          ))
        )}
      </div>

      {/* footer */}
      <Footer />
    </div>
  )
}