import { format, formatDistanceToNow } from 'date-fns'
import { useAuthentication } from '../../contexts/AuthenticationContext'

interface MessageCardProps {
  id: string
  chat_id: string
  sender_id: string
  content: string
  created_at: Date
}

export const MessageCard = ({ sender_id, content, created_at }: MessageCardProps) => {
  const { userId } = useAuthentication()
  const isMe = sender_id === userId

  return (
    <div className={`flex ${isMe ? 'justify-end' : 'justify-start'}`}>
      <div
        className={`max-w-[75%] rounded-lg p-4 ${
          isMe 
            ? 'bg-primary text-white rounded-br-none' 
            : 'bg-background-paper text-text-primary rounded-bl-none'
        }`}
      >
        <p className="break-words">{content}</p>
        <div className={`mt-2 text-xs ${isMe ? 'text-primary-100' : 'text-text-secondary'}`}>
          {format(created_at, 'HH:mm')}
          <span className="mx-2">•</span>
          {formatDistanceToNow(created_at, { addSuffix: true })}
        </div>
      </div>
    </div>
  )
}